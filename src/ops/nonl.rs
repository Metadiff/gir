use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::ids;


#[derive(Debug, Clone)]
pub struct Tanh {}

impl Operator for Tanh {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let anc = g.get_node(x)?.ancestors[0];
        if flow_tree[anc] {
            let one = g.constant_scalar(1.0, FundamentalType::Float).id;
            let dp = ids::sub(g, one, ids::mul(g, vec![x, x])?)?;
            Ok(vec![(anc, ids::mul(g, vec![dx, dp])?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static TANH: OperatorMetaData = OperatorMetaData{
            name: "Tanh",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &TANH
    }
}
