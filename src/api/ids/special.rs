use primitives::*;
use ops::*;
use graph::*;
use errors::*;

pub fn cast(graph: &Graph, arg: usize, data_type: FundamentalType) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Cast {data_type: data_type}), &vec![arg])?)
}

pub fn broadcast(graph: &Graph, arg: usize, to: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Broadcast {}), &vec![arg, to])?)
}

pub fn make_constant(graph: &Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(MakeConstant {}), &vec![arg])?)
}