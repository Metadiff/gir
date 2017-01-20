use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;

#[derive(Debug, Clone)]
pub struct Diagonal {}

impl Operator for Diagonal {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, Expr)> {
        unimplemented!()
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static DIAGONAL: OperatorMetaData = OperatorMetaData{
            name: "Diagonal",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            elementwise: false,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            fixed_output_type: None,
            scalar_output: false
        };
        &DIAGONAL
    }

    fn get_shape(&self, args: &Vec<Expr>) -> Shape {
        let node = args[0].get().unwrap();
        if node.shape.order() == 1 {
            Shape(node.shape.0.clone(), node.shape.0.clone(), 1.into(), 1.into())
        } else {
            Shape(node.shape.0.clone(), 1.into(), 1.into(), 1.into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct LowerTriangular {
    pub k: i32
}

impl Operator for LowerTriangular {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, usize)> {
        unimplemented!()
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static LOWER_TRIANGULAR: OperatorMetaData = OperatorMetaData{
            name: "LowerTriangular",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            elementwise: true,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            fixed_output_type: None,
            scalar_output: false
        };
        &LOWER_TRIANGULAR
    }
}

#[derive(Debug, Clone)]
pub struct UpperTriangular {
    pub k: i32
}

impl Operator for UpperTriangular {
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Vec<(usize, Expr)> {
        unimplemented!()
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static UPPER_TRIANGULAR: OperatorMetaData = OperatorMetaData{
            name: "UpperTriangular",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            elementwise: true,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            fixed_output_type: None,
            scalar_output: false
        };
        &UPPER_TRIANGULAR
    }
}

//#[derive(Debug, Clone)]
//pub struct Reshape {}
//
//impl Operator for Reshape {
//    fn clone_box(&self) -> Box<Operator> {
//        Box::new(self.clone())
//    }
//
//    fn get_meta(&self) -> &OperatorMetaData {
//        static UPPER_TRIANGULAR: OperatorMetaData = OperatorMetaData{
//            name: "UpperTriangular",
//            arity: Arity::Unary,
//            num_outputs: 1,
//            differential_parents: 1,
//            elementwise: true,
//            type_preserving: true,
//            reduction: false,
//            differentiable: true,
//            fixed_output_type: None,
//            scalar_output: false
//        };
//        &UPPER_TRIANGULAR
//    }
//}

