use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use std::any::Any;


#[derive(Debug, Clone)]
pub struct TensorShape {
    pub axis: Axis,
}

impl Operator for TensorShape {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        unimplemented!()
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.axis)))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static TENSOR_SHAPE: OperatorMetaData = OperatorMetaData{
            name: "TensorShape",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: false,
            type_preserving: false,
            reduction: false,
            differentiable: false,
            scalar_output: true,
            shape_operator: true,
            fixed_output_type: Some(FundamentalType::UnsignedInt),
        };
        &TENSOR_SHAPE
    }

    fn apply(&self, g: &Graph, args: Vec<usize>) -> Result<ExprData> {
        let args = self.verify_args(g, args)?;
        Ok(ExprData{
            id: 0,
            name: "".into(),
            ancestors: args.clone(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: self.get_data_type(g, &args),
            shape: self.get_shape(g, &args),
            is_input_dependent: self.get_is_input_dependent(g, &args),
            is_differentiable: self.get_is_differentiable(g, &args),
            matrix_positivity: self.get_matrix_positivity(g, &args),
            matrix_symmetry: self.get_matrix_symmetry(g, &args),
            matrix_fill: self.get_matrix_fill(g, &args),
            grad_level: self.get_grad_level(g, &args),
            scope: "".into(),
            sym_int: Some(g.get_node(args[0]).unwrap().shape.get(self.axis).clone())
        })
    }
}

