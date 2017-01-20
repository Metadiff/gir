use ops::*;
use graph::*;
use errors::*;

pub fn add(graph: &Graph, args: &Vec<usize>) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Add {}), args)?)
}

pub fn neg(graph: &Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Neg {}), &vec![arg])?)
}

pub fn sub(graph: &Graph, arg0: usize, arg1: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Add {}),
                      &vec![arg0,
                            graph.apply_op(Box::new(Neg {}), &vec![arg1])?])?)

}

pub fn mul(graph: &Graph, args: &Vec<usize>) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Mul {}), args)?)
}



