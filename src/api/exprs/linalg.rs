use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::borrow::Borrow;

pub fn mat_mul<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.borrow();
    let arg1 = arg1.borrow();
    same_graph_2(arg0, arg1)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mat_mul(&arg0.graph, arg0.id, arg1.id)?
    })
}


