use ops::*;
use graph::*;
use errors::*;

pub fn add(graph: &mut Graph, args: Vec<usize>) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Add {}), args)?)
}

pub fn neg(graph: &mut Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Neg {}), vec![arg])?)
}

pub fn sub(graph: &mut Graph, arg0: usize, arg1: usize) -> Result<usize> {
    let arg1 = neg(graph, arg1)?;
    Ok(graph.apply_op(Box::new(Add {}), vec![arg0, arg1])?)

}

pub fn mul(graph: &mut Graph, args: Vec<usize>) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Mul {}), args)?)
}

pub fn reciprocal(graph: &mut Graph, arg: usize) -> Result<usize> {
    Ok(graph.apply_op(Box::new(Div {}), vec![arg])?)
}

pub fn div(graph: &mut Graph, arg0: usize, arg1: usize) -> Result<usize> {
    let arg1 = reciprocal(graph, arg1)?;
    Ok(graph.apply_op(Box::new(Mul {}), vec![arg0, arg1])?)

}



