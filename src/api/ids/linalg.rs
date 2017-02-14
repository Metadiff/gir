use ops::*;
use graph::*;
use errors::*;

pub fn mat_mul(graph: &mut Graph, arg0: usize, arg1: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(MatMul {}), vec![arg0, arg1])?)
}
