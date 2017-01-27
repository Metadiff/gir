use graph::*;
use errors::*;
use super::super::ids;
use std::borrow::Borrow;

pub fn tanh<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let r = arg.borrow();
    Ok(Expr {
        graph: r.graph.clone(),
        id: ids::tanh(&r.graph, r.id)?
    })
}


