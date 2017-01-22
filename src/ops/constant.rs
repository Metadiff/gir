use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;

#[derive(Debug, Clone)]
pub struct ConstantScalar {
    pub value: f64,
    pub data_type: FundamentalType
}

impl Operator for ConstantScalar {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Result<Vec<(usize, usize)>> {
        unimplemented!()
    }

    fn apply_null(&self) -> ExprData {
        ExprData{
            id: 0,
            name: "".into(),
            ancestors: Vec::new(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: self.data_type,
            shape: Shape::scalar_shape(),
            is_input_dependent: false,
            is_differentiable: false,
            matrix_positivity: MatrixPositivity::Indefinite,
            matrix_symmetry: MatrixSymmetry::NonSymmetric,
            matrix_fill: MatrixFill::NonStructuredFill,
            grad_level: 0,
            scope: "".into(),
            sym_int: None
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static CONSTANT_SCALAR: OperatorMetaData = OperatorMetaData{
            name: "ConstantScalar",
            arity: Arity::Nullary,
            num_outputs: 1,
            differential_parents: 0,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: false,
            scalar_output: true,
            shape_operator: false,
            fixed_output_type: None,
        };
        &CONSTANT_SCALAR
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
}

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