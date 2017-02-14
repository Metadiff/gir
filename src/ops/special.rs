use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::*;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Update {}

impl Operator for Update {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        unimplemented!()
    }

    fn verify_args(&self, g: &mut Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        // Verify first argument is a Parameter
        if g.get_node(args[0])?.op.get_meta().name != "Parameter" {
            return Err(ErrorKind::InvalidArguments(
                String::new() + meta.name, args,
                "First argument must be a parameter.".into()).into())
        }
        // Verify that the first argument does not already have an Update
        match g.op_map.get("Update").unwrap_or(&Vec::new()).iter().position(|&x| x == args[0]) {
            Some(_) => {
                let param_name = g.get_node(args[0])?.op.get_args().unwrap()
                    .downcast::<(String, FundamentalType, Shape)>().unwrap().0;
                Err(ErrorKind::InvalidArguments(
                    String::new() + meta.name, args,
                    format!("The parameter '{}' already has an update.", param_name)).into())
            },
            None => Ok(args)
        }

    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static UPDATE: OperatorMetaData = OperatorMetaData{
            name: "Update",
            arity: Arity::Binary,
            num_outputs: 0,
            differential_parents: 0,
            ordered_parents: true,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: false,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &UPDATE
    }
}


#[derive(Debug, Clone)]
pub struct Cast {
    pub data_type: FundamentalType,
}

impl Operator for Cast {
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::cast(g, dx, self.data_type)?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn verify_args(&self, g: &mut Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        let dt = g.get_node(args[0]).unwrap().data_type;
        if self.data_type < dt {
            match g.props.policies.downcast {
                Policy::Quite => {},
                Policy::Warn => {
                    warn!(g.log, format!("[{}] Down tensor casting from {} to {}.",
                                         meta.name, dt, self.data_type));
                },
                Policy::Raise => {
                    return Err(ErrorKind::Downcast(dt, self.data_type).into());
                },
            }
        }
        Ok(args)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.data_type)))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static CAST: OperatorMetaData = OperatorMetaData{
            name: "Cast",
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
        &CAST
    }
    #[allow(unused_variables, unused_mut)]
    fn get_data_type(&self, g: &Graph, args: &Vec<usize>) -> FundamentalType {
        self.data_type
    }
}

#[derive(Debug, Clone)]
pub struct Broadcast {
    pub axes: [bool; 4]
}

impl Operator for Broadcast {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::sum(g, dx, self.axes)?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn verify_args(&self, g: &mut Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        // Verify tensor has unit shape on the axes
        for (&br, &axis)  in self.axes.iter().zip(Axis::iter()) {
            if br{
                let ref shape = g.get_node(args[0]).unwrap().shape;
                if shape.get(axis) != &1 {
                    return Err(ErrorKind::InvalidArguments(
                        String::new() + meta.name, args,
                        format!("Trying to broadcast shape {} along axis {}, which is not unit.",
                                shape, axis)).into())
                }
            }
        }
        // Verify all args except the first are ShapeOperators
        let shape_operators = args.iter().skip(1).fold(true, |check, &a|
            check && g.get_node(a).unwrap().op.get_meta().shape_operator);
        if ! shape_operators {
            Err(ErrorKind::InvalidArguments(String::new() + meta.name, args,
                                            format!("The operator can accept only 'shape_operators'.")).into())
        } else {
            Ok(args)
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.axes.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static BROADCAST: OperatorMetaData = OperatorMetaData{
            name: "Broadcast",
            arity: Arity::Nary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
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
        let mut c = 0;
        let mut shapes = Shape::scalar_shape();
        for &axis in Axis::iter() {
            shapes.set(axis, if self.axes[axis as usize] {
                c += 1;
                g.get_node(args[c]).unwrap().sym_int.as_ref().unwrap().clone()
            } else {
                g.get_node(args[0]).unwrap().shape.get(axis).clone()
            });
        }
        shapes
    }
}

#[derive(Debug, Clone)]
pub struct MakeConstant {}

impl Operator for MakeConstant {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
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
            ordered_parents: true,
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

#[derive(Debug, Clone)]
pub struct Reorder {
    pub order: [Axis; 4],
}

impl Reorder {
    pub fn get_reverse(&self) -> [Axis; 4] {
        let map = |a| {
            if self.order[0] == a {
                Axis::Axis0
            } else if self.order[1] == a {
                Axis::Axis1
            } else if self.order[2] == a {
                Axis::Axis2
            } else {
                Axis::Axis3
            }
        };
        [map(Axis::Axis0), map(Axis::Axis1), map(Axis::Axis2), map(Axis::Axis3)]
    }
}

impl Operator for Reorder {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &mut Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::reorder(g, dx, Some(self.get_reverse()))?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_args(&self) -> Option<Box<Any>> {
        Some(Box::new((self.order.clone())))
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static REORDER: OperatorMetaData = OperatorMetaData{
            name: "Reorder",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
            elementwise: false,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &REORDER
    }
    #[allow(unused_variables, unused_mut)]
    fn get_shape(&self, g: &Graph, args: &Vec<usize>) -> Shape {
        let map = |x| match x {
            Axis::Axis0 => g.get_node(args[0]).unwrap().shape.0.clone(),
            Axis::Axis1 => g.get_node(args[0]).unwrap().shape.1.clone(),
            Axis::Axis2 => g.get_node(args[0]).unwrap().shape.2.clone(),
            Axis::Axis3 => g.get_node(args[0]).unwrap().shape.3.clone(),
        };
        Shape(map(self.order[0]), map(self.order[1]), map(self.order[2]), map(self.order[3]))
    }
}