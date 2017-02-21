use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;

//#[derive(Debug, Clone)]
//pub struct Shape {
//    dim: usize,
//    value: SymInt
//}
//
//impl Operator for Shape {
//    fn clone_box(&self) -> Box<Operator> {
//        Box::new(self.clone())
//    }
//
//    fn get_meta(&self) -> &OperatorMetaData {
//        static SHAPE: OperatorMetaData = OperatorMetaData{
//            name: "Shape",
//            arity: Arity::Unary,
//            num_outputs: 1,
//            differential_parents: 0,
//            elementwise: false,
//            type_preserving: false,
//            reduction: false,
//            differentiable: false,
//            fixed_output_type: Some(FundamentalType::UnsignedInt),
//            scalar_output: true
//        };
//        &SHAPE
//    }
//
//    fn get_is_input_dependent(&self, args: &Vec<Expr>) -> bool {
//        false
//    }
//}