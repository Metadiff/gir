use graph::*;
use primitives::*;
use errors::*;

use std::io;
use std::collections::HashMap;

pub trait CompiledFunction<TI, TO> {
    fn initialized(&self) -> bool;
    fn eval(&mut self, inputs: &[TI]) -> Result<Vec<TO>>;
    fn free_memory(&mut self);
}

#[derive(Debug, Clone, Default)]
pub struct MemoryMap {
    pub abstract_map: HashMap<usize, (SymInt, SymInt)>,
    pub current_map: HashMap<usize, (SymInt, SymInt)>,
    pub current_size: u64
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