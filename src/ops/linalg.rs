use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::*;


#[derive(Debug, Clone)]
pub struct MatrixMul {}

impl Operator for MatrixMul {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let anc = g.get_node(x)?.ancestors.clone();
        if anc.len() == 2 {
            let mut res = Vec::new();
            if flow_tree[anc[0]] {
                res.push((anc[0], ids::mat_mul(g, anc[1], ids::reorder(g, dx, None)?)?));
            }
            if flow_tree[anc[1]] {
                res.push((anc[1], ids::mat_mul(g, ids::reorder(g, anc[0], None)?, dx)?));
            }
            Ok(res)
        } else {
            unimplemented!()
        }
    }

    fn verify_args(&self, g: &Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        // Verify all args are 2 dimensional and that their mid shapes match
        let mut last_shape = g.get_node(args[0]).unwrap().shape.0.clone();
        let mut index = None;
        for (i, &a) in args.iter().enumerate() {
            let ref shape = g.get_node(a).unwrap().shape;
            if shape.order() > 2 {
                return Err(ErrorKind::InvalidArguments(String::new() + meta.name, args.clone(),
                                                       format!("The tensor at index {} is of order {} > 2.", i, shape.order())).into());
            } else if last_shape != shape.0 {
                index = Some(i);
                break;
            }
            last_shape = shape.1.clone();
        }
        if let Some(id) = index {
            return Err(ErrorKind::InvalidShapes(String::new() + meta.name,
                                                format!("{}", g.get_node(args[id-1]).unwrap().shape),
                                                format!("{}", g.get_node(args[id]).unwrap().shape)).into())
        }
        Ok(args)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static MATRIX_MUL: OperatorMetaData = OperatorMetaData{
            name: "MatrixMul",
            arity: Arity::Nary,
            num_outputs: 1,
            differential_parents: ::std::usize::MAX,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &MATRIX_MUL
    }

    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        let m = g.get_node(args[0]).unwrap().shape.0.clone();
        let n = g.get_node(*args.last().unwrap()).unwrap().shape.1.clone();
        Shape(m, n, 1.into(), 1.into())
    }
}
