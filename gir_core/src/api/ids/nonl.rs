use ops::*;
use graph::*;
use errors::*;

pub fn tanh(graph: &mut Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Tanh {}), vec![arg])?)
}