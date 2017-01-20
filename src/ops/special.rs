use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;

#[derive(Debug, Clone)]
pub struct Cast {
    data_type: FundamentalType,
}

impl Operator for Cast {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, Expr)> {
        unimplemented!()
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
            fixed_output_type: None,
            scalar_output: false
        };
        &CAST
    }

    fn get_data_type(&self, args: &Vec<Expr>) -> FundamentalType {
        self.data_type
    }
}

#[derive(Debug, Clone)]
pub struct Broadcast {}

impl Operator for Broadcast {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, Expr)> {
        unimplemented!()
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
            fixed_output_type: None,
            scalar_output: false
        };
        &BROADCAST
    }
}

#[derive(Debug, Clone)]
pub struct MakeConstant {}

impl Operator for MakeConstant {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, Expr)> {
        unimplemented!()
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
            fixed_output_type: None,
            scalar_output: false
        };
        &MAKE_CONSTANT
    }
}