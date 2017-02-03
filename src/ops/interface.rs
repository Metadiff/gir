use primitives::*;
use graph::*;
use errors::*;
use api::ids;
use std::any::Any;
//use std::borrow::Borrow;
//use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorMetaData {
    pub name: &'static str,
    pub arity: Arity,
    pub num_outputs: usize,
    pub differential_parents: usize,
    pub ordered_parents: bool,
    pub elementwise: bool,
    pub type_preserving: bool,
    pub reduction: bool,
    pub differentiable: bool,
    pub scalar_output: bool,
    pub shape_operator: bool,
    pub fixed_output_type: Option<FundamentalType>,
}

pub trait Operator: ::std::fmt::Debug {
    /// Calculates the derivative of the parent expressions given the derivative
    /// of the current.
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>>;

    /// Clones the concrete operator and wraps it in a box again
    fn clone_box(&self) -> Box<Operator>;

    /// Gives back any extra args boxed
    fn get_args(&self) -> Option<Box<Any>> {
        None
    }

    /// Returns the meta data
    fn get_meta(&self) -> &OperatorMetaData;

//    fn apply_expr(&self, args: &Vec<Expr>) -> Result<ExprData> {
//        if args.len() == 0 {
//            Err(ErrorKind::InvalidArguments(
//                String::new() + self.get_meta().name, Vec::new(),
//                "apply_expr takes at least 1 argument.".into()).into())
//        } else {
//            self.apply(&args[0].graph, args.iter().map(|x| x.id).collect())
//        }
//    }

    fn apply(&self, g: &mut Graph, args: Vec<usize>) -> Result<ExprData> {
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
            scope: Vec::new(),
            sym_int: None
        })
    }

    /// Should be implemented only by operators with no arguments
    fn apply_null(&self) -> ExprData {
        unimplemented!()
    }

    fn verify_args(&self, g: &mut Graph, args: Vec<usize>) -> Result<Vec<usize>> {
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
        args.iter().map(|&x| g.nodes[x].grad_level).max().unwrap()
    }
}

impl Clone for Box<Operator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}



pub mod default {
    use super::*;

    pub fn verify_args(meta: &OperatorMetaData, g: &mut Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        // Verify number of arguments
        let l = args.len();
        match meta.arity {
            Arity::Nullary => if l != 0 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 0 arguments, got {}.", l)).into())
            },
            Arity::Unary => if l != 1 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 1 arguments, got {}.", l)).into())
            },
            Arity::Binary => if l != 2 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 2 arguments, got {}.", l)).into())
            },
            Arity::Ternary => if l != 3 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 3 arguments, got {}.", l)).into())
            },
            Arity::Quaternary => if l != 4 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 4 arguments, got {}.", l)).into())
            },
            Arity::Quinary => if l != 5 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting 5 arguments, got {}.", l)).into())
            },
            Arity::Nary => if l < 2 {
                return Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("Expecting at least 2 arguments, got {}.", l)).into())
            },
        }
        // Verify individual arguments
        for &arg in &args {
            let node = g.get_node(arg)?;
            if node.op.get_meta().name == "Update" {
                return Err(ErrorKind::Msg("Attempting to use 'Update' \
                in another operation.".into()).into())
            }
        }
        Ok(args)
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

    pub fn get_reduction_shape(mut shape: Shape, axes: &[bool; 4]) -> Shape {
        let axes = axes.as_ref();
        for &axis in Axis::iter() {
            if axes[axis as usize] {
                shape.set(axis, 1.into());
            }
        }
        shape
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

    pub fn broadcast_shapes(graph: &mut Graph, name: &str, mut args: Vec<usize>) -> Result<Vec<usize>> {
        let mut shape = graph.get_node(args[0]).unwrap().shape.clone();
        let mut shape_i = [args[0], args[0], args[0], args[0]];
        for &a in args.iter().skip(1) {
            let ref ai_shape = graph.get_node(a).unwrap().shape;
            for &axis in Axis::iter() {
                if shape.get(axis) != ai_shape.get(axis) {
                    if *shape.get(axis) == 1 as i64 {
                        shape.set(axis, ai_shape.get(axis).clone());
                        shape_i[axis as usize] = a;
                    } else if *ai_shape.get(axis) != 1 as i64 {
                        return Err(ErrorKind::InvalidShapes(
                            format!("{}", name),
                            format!("{}", shape),
                            format!("{}", ai_shape)).into())
                    }
                }
            }
        }
        // Make sure all arguments are up to that shape, if not broadcast them accordingly
        for a in args.iter_mut() {
            if shape != graph.get_node(*a).unwrap().shape  &&
                graph.get_node(*a).unwrap().shape != Shape::scalar_shape() {
                let br: Vec<Option<usize>> = Axis::iter().zip(shape_i.iter())
                    .map(|(&axis, &arg_id)| {
                        if shape.get(axis) != graph.get_node(*a).unwrap().shape.get(axis) {
                            Some(ids::dim(graph, arg_id, axis).unwrap())
                        } else {
                            None
                        }
                    }).collect();
                match graph.props.policies.implicit_broadcast {
                    Policy::Quite => {},
                    Policy::Warn => {
                        warn!(graph.log,
                        format!("[{}] Implicit broadcast from shape {} to shape {}.",
                                name,  graph.get_node(*a).unwrap().shape, shape));
                    },
                    Policy::Raise => {
                        return Err(ErrorKind::InvalidShapes(
                            format!("{}", name),
                            format!("{}", graph.get_node(*a).unwrap().shape),
                            format!("{}", shape)).into())
                    },
                }
                *a = ids::broadcast(graph, *a, [br[0], br[1], br[2], br[3]])?;
            }
        }
        Ok(args)
    }

    pub fn same_graph<T: AsRef<Expr>>(exprs: &Vec<T>) -> Result<()> {
        if exprs.len() > 1 {
            let ref g0 = exprs[0].as_ref().wrapper;
            for expr in exprs.iter().skip(1) {
                if !::std::ptr::eq(&*g0.graph, &*expr.as_ref().wrapper.graph) {
                    return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
                }
            }
        }
        Ok(())
    }

    pub fn same_graph_2<T1: AsRef<Expr>, T2: AsRef<Expr>>(expr0: T1, expr1: T2) -> Result<()> {
        if !::std::ptr::eq(&*expr0.as_ref().wrapper.graph, &*expr1.as_ref().wrapper.graph) {
            return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
        } else {
            Ok(())
        }
    }

    pub fn same_graph_3(expr0: &Expr, expr1: &Expr, expr2: &Expr) -> Result<()> {
        if !::std::ptr::eq(&*expr0.wrapper.graph, &*expr1.wrapper.graph) ||
            !::std::ptr::eq(&*expr0.wrapper.graph, &*expr2.wrapper.graph) {
            return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
        } else {
            Ok(())
        }
    }

    pub fn same_graph_4(expr0: &Expr, expr1: &Expr, expr2: &Expr, expr3: &Expr) -> Result<()> {
        if !::std::ptr::eq(&*expr0.wrapper.graph, &*expr1.wrapper.graph) ||
            !::std::ptr::eq(&*expr0.wrapper.graph, &*expr2.wrapper.graph) ||
            !::std::ptr::eq(&*expr0.wrapper.graph, &*expr3.wrapper.graph) {
            return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
        } else {
            Ok(())
        }
    }
}
