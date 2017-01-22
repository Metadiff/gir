use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::*;

#[derive(Debug, Clone)]
pub struct Cast {
    pub data_type: FundamentalType,
}

impl Operator for Cast {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::cast(g, dx, self.data_type)?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static CAST: OperatorMetaData = OperatorMetaData{
            name: "Cast",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &CAST
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
}

#[derive(Debug, Clone)]
pub struct Broadcast {}

impl Operator for Broadcast {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            // Needs reduction sum
            unimplemented!()
        } else {
            Ok(Vec::new())
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static BROADCAST: OperatorMetaData = OperatorMetaData{
            name: "Broadcast",
            arity: Arity::Binary,
            num_outputs: 1,
            differential_parents: 1,
            elementwise: false,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &BROADCAST
    }
    #[allow(unused_variables, unused_mut)]
    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        g.get_node(args[1]).unwrap().shape.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MakeConstant {}

impl Operator for MakeConstant {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Result<Vec<(usize, usize)>> {
        // No gradients are passed trough this operator
        Ok(Vec::new())
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static MAKE_CONSTANT: OperatorMetaData = OperatorMetaData{
            name: "MakeConstant",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 0,
            elementwise: true,
            type_preserving: true,
            reduction: false,
            differentiable: false,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &MAKE_CONSTANT
    }
}