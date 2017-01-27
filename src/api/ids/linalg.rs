use ops::*;
use graph::*;
use errors::*;

pub fn mat_mul(graph: &Graph, arg0: usize, arg1: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(MatrixMul {}), vec![arg0, arg1])?)
}
