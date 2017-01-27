use primitives::*;
use ops::*;
use graph::*;
use errors::*;
use api::ids;

pub fn cast(graph: &Graph, arg: usize, data_type: FundamentalType) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Cast {data_type: data_type}), vec![arg])?)
}

pub fn broadcast(graph: &Graph, arg: usize, shape: [Option<usize>; 4]) -> Result<usize> {
    let shape_arg = graph.get_node(arg)?.shape.clone();
    let mut args = vec![arg];
    let mut axes = [false; 4];
    for &axis in Axis::iter() {
        if let Some(s) = shape[axis as usize] {
            if shape_arg.get(axis) != graph.get_node(s)?.sym_int.as_ref().unwrap() {
                args.push(s);
                axes[axis as usize] = true;
            }
        }
    }
    if args.len() > 1 {
        graph.apply_op(Box::new(Broadcast {axes: axes}), args)
    } else {
        Ok(arg)
    }
}

pub fn broadcast_to(graph: &Graph, arg: usize, to: usize) -> Result<usize> {
    let arg_shape = graph.get_node(arg).unwrap().shape.clone();
    if arg_shape != graph.get_node(to).unwrap().shape {
        let mut broadcast_shape = [None; 4];
        for &axis in Axis::iter() {
            if arg_shape.get(axis) != graph.get_node(to).unwrap().shape.get(axis) {
                broadcast_shape[axis as usize] = Some(ids::dim(graph, to, axis)?);
            }
        }
        broadcast(graph, arg, broadcast_shape)
    } else {
        Ok(arg)
    }
}

pub fn make_constant(graph: &Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(MakeConstant {}), vec![arg])?)
}

/// Reverses the axes if order is None (e.g. transpose)
pub fn reorder(graph: &Graph, arg: usize, order: Option<[Axis; 4]>) -> Result<usize> {
    match order {
        Some(o) => Ok(graph.apply_op(Box::new(Reorder {order: o}), vec![arg])?),
        None => {
            let o = graph.get_node(arg)?.shape.order();
            match o {
                0 => Ok(arg),
                1 | 2 => Ok(graph.apply_op(Box::new(Reorder {
                    order: [Axis::Axis1, Axis::Axis0, Axis::Axis2, Axis::Axis3]
                }), vec![arg])?),
                3 => Ok(graph.apply_op(Box::new(Reorder {
                    order: [Axis::Axis2, Axis::Axis1, Axis::Axis0, Axis::Axis3]
                }), vec![arg])?),
                _ => Ok(graph.apply_op(Box::new(Reorder {
                    order: [Axis::Axis3, Axis::Axis2, Axis::Axis1, Axis::Axis0]
                }), vec![arg])?)
            }
        }
    }
}