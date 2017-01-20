use primitives::*;
use graph::*;
use errors::*;

#[derive(Debug, Clone)]
pub struct OperatorMetaData {
    pub name: &'static str,
    pub arity: Arity,
    pub num_outputs: usize,
    pub differential_parents: usize,
    pub elementwise: bool,
    pub type_preserving: bool,
    pub reduction: bool,
    pub differentiable: bool,
    pub scalar_output: bool,
    pub fixed_output_type: Option<FundamentalType>,
}

pub trait Operator: ::std::fmt::Debug {
    /// Calculates the derivative of the parent expressions given the derivative
    /// of the current.
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
        -> Result<Vec<(usize, usize)>>;

    /// Clones the concrete operator and wraps it in a box again
    fn clone_box(&self) -> Box<Operator>;

    /// Returns the meta data
    fn get_meta(&self) -> &OperatorMetaData;

    fn apply_expr(&self, args: &Vec<Expr>) -> Result<ExprData> {
        if args.len() == 0 {
            Err(ErrorKind::InvalidArguments(0).into())
        } else {
            self.apply(&args[0].graph, &args.iter().map(|x| x.id).collect())
        }
    }

    fn apply(&self, g: &Graph, args: &Vec<usize>) -> Result<ExprData> {
        self.verify_args(g, args)?;
        Ok(ExprData{
            id: 0,
            name: "".into(),
            ancestors: args.clone(),
            children: Vec::new(),
            op: self.clone_box(),
            data_type: self.get_data_type(g, args),
            shape: self.get_shape(g, args),
            is_input_dependent: self.get_is_input_dependent(g, args),
            is_differentiable: self.get_is_differentiable(g, args),
            matrix_positivity: self.get_matrix_positivity(g, args),
            matrix_symmetry: self.get_matrix_symmetry(g, args),
            matrix_fill: self.get_matrix_fill(g, args),
            grad_level: self.get_grad_level(g, args),
            scope: "".into()
        })
    }

    /// Should be implemented only by operators with no arguments
    fn apply_null(&self) -> ExprData {
        unimplemented!()
    }

    fn verify_args(&self, g: &Graph, args: &Vec<usize>) -> Result<()> {
        let meta = self.get_meta();
        default::verify_args(meta, g, args)
    }

    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        let meta = self.get_meta();
        default::get_data_type(meta, g, args)
    }

    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        let meta = self.get_meta();
        default::get_shape(meta, g, args)
    }

    fn get_is_input_dependent(&self, g: &Graph, args: &Vec<usize>) -> bool {
        let meta = self.get_meta();
        default::get_is_input_dependent(meta, g, args)
    }

    fn get_is_differentiable(&self, g: &Graph, args: &Vec<usize>) -> bool {
        let meta = self.get_meta();
        default::get_is_differentiable(meta, g, args)
    }

    #[allow(unused_variables, unused_mut)]
    fn get_matrix_positivity(&self, g: &Graph, args: &Vec<usize>) -> MatrixPositivity {
        MatrixPositivity::Indefinite
    }
    #[allow(unused_variables, unused_mut)]
    fn get_matrix_symmetry(&self, g: &Graph, args: &Vec<usize>) -> MatrixSymmetry {
        MatrixSymmetry::NonSymmetric
    }
    #[allow(unused_variables, unused_mut)]
    fn get_matrix_fill(&self, g: &Graph, args: &Vec<usize>) -> MatrixFill {
        MatrixFill::NonStructuredFill
    }

    fn get_grad_level(&self, g: &Graph, args: &Vec<usize>) -> usize {
        args.iter().map(|&x| g.get().nodes[x].grad_level).max().unwrap()
    }
}

impl Clone for Box<Operator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}



mod default {
    use super::*;

    pub fn verify_args(meta: &OperatorMetaData, g: &Graph, args: &Vec<usize>) -> Result<()> {
        // Verify number of arguments
        match meta.arity {
            Arity::Nullary => if args.len() != 0 {
                return Err(ErrorKind::InvalidArguments(0).into())
            },
            Arity::Unary => if args.len() != 1 {
                return Err(ErrorKind::InvalidArguments(0).into())
            },
            Arity::Binary => if args.len() != 2 {
                return Err(ErrorKind::InvalidArguments(0).into())
            },
            Arity::Ternary => if args.len() != 3 {
                return Err(ErrorKind::InvalidArguments(0).into())
            },
            Arity::Nary => if args.len() < 2 {
                return Err(ErrorKind::InvalidArguments(0).into())
            },
        }
        // Verify individual arguments
        for &arg in args {
            g.get().nodes.get(arg).ok_or(ErrorKind::InvalidArguments(0))?;
        }
        Ok(())
    }

    pub fn get_data_type(meta: &OperatorMetaData, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        if let Some(data_type) = meta.fixed_output_type {
            data_type
        } else if meta.type_preserving {
            g.get_node(args[0]).unwrap().data_type
        } else {
            args.iter().map(|&i| g.get_node(i).unwrap().data_type).max().unwrap()
        }
    }

    pub fn get_shape(meta: &OperatorMetaData, g: &Graph, args: &Vec<usize>) -> Shape {
        if meta.scalar_output {
            Shape(1.into(), 1.into(), 1.into(), 1.into())
        } else if meta.elementwise {
            g.get_node(args[0]).unwrap().shape.clone()
        } else {
            unimplemented!()
        }
    }

    #[allow(unused_variables, unused_mut)]
    pub fn get_is_input_dependent(meta: &OperatorMetaData, g: &Graph, args: &Vec<usize>) -> bool {
        for &i in args {
            if g.get_node(i).unwrap().is_input_dependent {
                return true
            }
        }
        false
    }
    #[allow(unused_variables, unused_mut)]
    pub fn get_is_differentiable(meta: &OperatorMetaData, g: &Graph, args: &Vec<usize>) -> bool {
        if !meta.differentiable {
            false
        } else {
            for &i in args {
                if g.get_node(i).unwrap().is_differentiable {
                    return true
                }
            }
            false
        }
    }
}
