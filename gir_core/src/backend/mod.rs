use graph::*;
use primitives::*;
use errors::*;

use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct BackendPrecisions {
    pub integer_precision: Precision,
    pub float_precision: Precision,
    pub complex_precision: Precision
}

impl Default for BackendPrecisions {
    fn default() -> Self {
        BackendPrecisions {
            integer_precision: Precision::P32,
            float_precision: Precision::P32,
            complex_precision: Precision::P32,
        }
    }
}

pub trait CompiledFunction<TI, TO> {
    fn initialized(&self) -> bool;
    fn eval(&mut self, inputs: &[&TI]) -> Result<Vec<TO>>;
    fn free_memory(&mut self);
}

#[derive(Debug, Clone, Default)]
pub struct AbstractMemoryMap {
    // Maps node id to (offset in memory, size in memory)
    pub abstract_map: HashMap<usize, (SymInt, SymInt)>,
    // (number of booleans, number of integers, number of floats, number of complex)
    pub abstract_size: (SymInt, SymInt, SymInt, SymInt),
}

pub trait Backend<F>: Default {
    fn info(&self, f: &mut io::Write) -> io::Result<()>;
    fn general_info(&self, f: &mut io::Write) -> io::Result<()>;

    fn print_info(&self) -> io::Result<()> {
        self.info(&mut io::stdout())
    }

    fn print_general_info(&self) -> io::Result<()> {
        self.general_info(&mut io::stdout())
    }

    fn get_precisions(&self) -> &BackendPrecisions;
    fn set_precisions(&mut self, precisions: BackendPrecisions);

    fn make_function(&self, graph_function: GraphFunction) -> F;
}

pub fn verify_shapes(new_shapes: &[[usize; 4]],
                     last_shapes: &[[usize; 4]],
                     symbolic_shapes: &[Shape])
                     -> Result<Option<HashMap<String, i64>>> {
    let changed = new_shapes.len() != last_shapes.len() ||
        new_shapes.iter().zip(last_shapes.iter()).any(|(s1, s2)| s1 != s2);
    if changed {
        let mut implicit = Vec::new();
        for (index, (s, sym_s)) in new_shapes.iter().zip(symbolic_shapes.iter()).enumerate() {
            for &axis in Axis::iter() {
                let actual = s[axis as usize];
                let symbolic = sym_s.get(axis);
                if ! symbolic.is_constant() {
                    implicit.push((symbolic, actual as i64));
                } else if symbolic.eval(&HashMap::new()).unwrap() != actual as i64 {
                    return Err(ErrorKind::Msg(format!(
                        "Incorrect shape of input at index {}. \
                        The shape on dimension {} does not match.\
                        Expected: {}, actual: {}.",
                        index, axis,
                        symbolic.eval(&HashMap::new()).unwrap(),
                        actual)).into())
                }
            }
        }
        let deduced = ::symbolic_polynomials::deduce_values(&implicit)?;
        Ok(Some(deduced))
    } else {
        Ok(None)
    }
}

pub fn build_memory_map(gf: &GraphFunction) -> AbstractMemoryMap {
    let mut map = HashMap::new();
    let mut offset: SymInt = 0.into();
    let mut b_size: SymInt = 0.into();
    let mut i_size: SymInt = 0.into();
    let mut f_size: SymInt = 0.into();
    let mut c_size: SymInt = 0.into();
    for &i in &gf.graph.order {
        let ref node = gf.graph.nodes[i];
        let op_meta = node.op.get_meta();
        match op_meta.name {
            "Scalar" | "SymIntInput" | "TensorShape" | "Broadcast" | "Parameter" => {},
            _ => {
                let n = gf.graph.nodes[i].shape.elements();
                map.insert(i, (offset.clone(), n.clone()));
                match gf.graph.nodes[i].data_type {
                    FundamentalType::Boolean => {
                        b_size += &n;
                    },
                    FundamentalType::SignedInt | FundamentalType::UnsignedInt => {
                        i_size += &n;
                    },
                    FundamentalType::Float => {
                        f_size += &n;
                    },
                    FundamentalType::Complex => {
                        c_size += &n;
                    }
                }
                offset += &n;
            }
        }
    }
    AbstractMemoryMap {
        abstract_map: map,
        abstract_size: (b_size, i_size, f_size, c_size),
    }
}
