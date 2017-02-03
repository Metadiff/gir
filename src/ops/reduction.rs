use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::ids;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Sum {
    pub axes: [bool; 4]
}

impl Operator for Sum {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::broadcast_to(g, dx, ancestor)?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.axes.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static SUM: OperatorMetaData = OperatorMetaData{
            name: "Sum",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: true,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &SUM
    }

    #[allow(unused_variables, unused_mut)]
    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        default::get_reduction_shape(g.get_node(args[0]).unwrap().shape.clone(), &self.axes)
    }
}