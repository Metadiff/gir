use primitives::*;
use ops::*;
use graph::*;
use errors::*;

pub fn dim(graph: &Graph, arg:usize, axis: Axis) -> Result<usize> {
    graph.apply_op(Box::new(TensorShape {axis: axis}), vec![arg])
}

pub fn shape(graph: &Graph, arg: usize) -> Result<(usize, usize, usize, usize)> {
    let shape0 = graph.apply_op(Box::new(TensorShape {axis: Axis::Axis0}), vec![arg])?;
    let shape1 = graph.apply_op(Box::new(TensorShape {axis: Axis::Axis1}), vec![arg])?;
    let shape2 = graph.apply_op(Box::new(TensorShape {axis: Axis::Axis2}), vec![arg])?;
    let shape3 = graph.apply_op(Box::new(TensorShape {axis: Axis::Axis3}), vec![arg])?;
    Ok((shape0, shape1, shape2, shape3))
}