use ops::*;
use graph::*;
use errors::*;

pub fn tanh(graph: &Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Tanh {}), vec![arg])?)
}