use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use symbolic_polynomials::variable;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Input {
    pub data_type: FundamentalType,
    pub shape: Shape
}

impl Operator for Input {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        Ok(Vec::new())
    }

    fn apply_null(&self) -> ExprData {
        ExprData{
            id: 0,
            name: "".into(),
            ancestors: Vec::new(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: self.data_type,
            shape: self.shape.clone(),
            is_input_dependent: true,
            is_differentiable: true,
            matrix_positivity: MatrixPositivity::Indefinite,
            matrix_symmetry: MatrixSymmetry::NonSymmetric,
            matrix_fill: MatrixFill::NonStructuredFill,
            grad_level: 0,
            scope: Vec::new(),
            sym_int: None
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.data_type, self.shape.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static INPUT: OperatorMetaData = OperatorMetaData{
            name: "Input",
            arity: Arity::Nullary,
            num_outputs: 1,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &INPUT
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
    #[allow(unused_variables, unused_mut)]
    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        self.shape.clone()
    }
    #[allow(unused_variables, unused_mut)]
    fn get_is_input_dependent(&self, g: &Graph, args: &Vec<usize>) -> bool {
        true
    }
    #[allow(unused_variables, unused_mut)]
    fn get_is_differentiable(&self, g: &Graph, args: &Vec<usize>) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub param_name: String,
    pub data_type: FundamentalType,
    pub shape: Shape
}

impl Operator for Parameter {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        Ok(Vec::new())
    }

    fn apply_null(&self) -> ExprData {
        ExprData{
            id: 0,
            name: self.param_name.clone(),
            ancestors: Vec::new(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: self.data_type,
            shape: self.shape.clone(),
            is_input_dependent: true,
            is_differentiable: true,
            matrix_positivity: MatrixPositivity::Indefinite,
            matrix_symmetry: MatrixSymmetry::NonSymmetric,
            matrix_fill: MatrixFill::NonStructuredFill,
            grad_level: 0,
            scope: Vec::new(),
            sym_int: None
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.data_type, self.shape.clone(), self.param_name.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static PARAMETER: OperatorMetaData = OperatorMetaData{
            name: "Parameter",
            arity: Arity::Nullary,
            num_outputs: 1,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &PARAMETER
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
    #[allow(unused_variables, unused_mut)]
    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        self.shape.clone()
    }
    #[allow(unused_variables, unused_mut)]
    fn get_is_input_dependent(&self, g: &Graph, args: &Vec<usize>) -> bool {
        true
    }
    #[allow(unused_variables, unused_mut)]
    fn get_is_differentiable(&self, g: &Graph, args: &Vec<usize>) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Scalar {
    pub value: f64,
    pub data_type: FundamentalType
}

impl Operator for Scalar {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        Ok(Vec::new())
    }

    fn apply_null(&self) -> ExprData {
        ExprData{
            id: 0,
            name: "Scalar".into(),
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
            scope: Vec::new(),
            sym_int: None
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.value, self.data_type)))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static SCALAR: OperatorMetaData = OperatorMetaData{
            name: "Scalar",
            arity: Arity::Nullary,
            num_outputs: 1,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: false,
            scalar_output: true,
            shape_operator: false,
            fixed_output_type: None,
        };
        &SCALAR
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
}

#[derive(Debug, Clone)]
pub struct SymIntInput {
    pub identifier: String,
}

impl Operator for SymIntInput {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        Ok(Vec::new())
    }

    fn apply_null(&self) -> ExprData {
        ExprData{
            id: 0,
            name: "SymInt".into(),
            ancestors: Vec::new(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: FundamentalType::UnsignedInt,
            shape: Shape::scalar_shape(),
            is_input_dependent: true,
            is_differentiable: false,
            matrix_positivity: MatrixPositivity::PositiveDefinite,
            matrix_symmetry: MatrixSymmetry::Symmetric,
            matrix_fill: MatrixFill::Diagonal,
            grad_level: 0,
            scope: Vec::new(),
            sym_int: Some(variable(self.identifier.clone()))
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.identifier.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static SYM_INT_INPUT: OperatorMetaData = OperatorMetaData{
            name: "SymIntInput",
            arity: Arity::Nullary,
            num_outputs: 1,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: false,
            scalar_output: true,
            shape_operator: true,
            fixed_output_type: Some(FundamentalType::SignedInt),
        };
        &SYM_INT_INPUT
    }
    #[allow(unused_variables, unused_mut)]
    fn get_is_input_dependent(&self, g: &Graph, args: &Vec<usize>) -> bool {
        true
    }
}




