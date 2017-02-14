use primitives::*;
use ops::*;
use graph::*;
use errors::*;
use api::ids;

pub fn overwrite_update(graph: &mut Graph, arg:usize, upd: usize) -> Result<bool> {
    let existed = remove_update(graph, arg)?;
    update(graph, arg, upd)?;
    Ok(existed)
//    let candidates = graph.get_node(arg)?.children.clone();
//    let mut old_update = None;
//    for &c in &candidates {
//        let node = graph.get_node(c)?;
//        if node.op.get_meta().name == "Update" {
//            old_update = Some((c, node.ancestors[1]));
//            break;
//        }
//    }
//    if let Some((c, prev_upd)) = old_update {
//        // Remove from arg
//        {
//            let mut node = graph.get_node_mut(arg).unwrap();
//            let pos = node.children.iter().position(|&x| x == c).unwrap();
//            node.children.remove(pos);
//        }
//        // Remove from previous upd
//        {
//            let mut node = graph.get_node_mut(prev_upd).unwrap();
//            let pos = node.children.iter().position(|&x| x == c).unwrap();
//            node.children.remove(pos);
//        }
//        // Remove from op_map
//        let new_updates = graph.op_map.get("Update").unwrap()
//            .iter().cloned().filter(|&x| x != c).collect();
//        graph.op_map.insert("Update".into(), new_updates);
//    }
}

pub fn update(graph: &mut Graph, arg:usize, upd: usize) -> Result<usize> {
    // Verify first argument is a Parameter
    if graph.get_node(arg)?.op.get_meta().name != "Parameter" {
        return Err(ErrorKind::InvalidArguments(
            "Update".into(), vec![arg, upd],
            "First argument must be a parameter.".into()).into())
    }
    // Verify that the first argument does not already have an Update
    for &u in graph.op_map.get("Update").unwrap() {
        if graph.nodes[u].ancestors[0] == arg {
            let ref param_name = graph.nodes[arg].name;
            return Err(ErrorKind::InvalidArguments(
                "Update".into(), vec![arg, upd],
                format!("The parameter '{}' already has an update - {}.", param_name, u)).into())
        }
    }
    graph.apply_op(Box::new(Update {}), vec![arg, upd])
}

pub fn remove_update(graph: &mut Graph, arg:usize) -> Result<bool> {
    let updates = graph.op_map.get("Update").unwrap();
    for &u in updates {
        if graph.nodes[u].ancestors[0] == arg {
            let upd = graph.nodes[u].ancestors[1];
            graph.nodes[arg].children.remove(&u);
            graph.nodes[upd].children.remove(&u);
            let op = Box::new(Cleared{});
            graph.nodes[u] = op.apply_null();
            return Ok(true)
        }
    }
    Ok(false)
}

pub fn cast(graph: &mut Graph, arg: usize, data_type: FundamentalType) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Cast {data_type: data_type}), vec![arg])?)
}

pub fn broadcast(graph: &mut Graph, arg: usize, shape: [Option<usize>; 4]) -> Result<usize> {
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

pub fn broadcast_to(graph: &mut Graph, arg: usize, to: usize) -> Result<usize> {
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

pub fn make_constant(graph: &mut Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(MakeConstant {}), vec![arg])?)
}

/// Reverses the axes if order is None (e.g. transpose)
pub fn reorder(graph: &mut Graph, arg: usize, order: Option<[Axis; 4]>) -> Result<usize> {
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