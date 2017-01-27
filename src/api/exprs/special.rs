use primitives::*;
use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::borrow::Borrow;

pub fn cast(arg: &Expr, data_type: FundamentalType) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::cast(&arg.graph, arg.id, data_type)?
    })
}

pub fn broadcast<T: Borrow<Expr>>(arg: T, shape: [Option<&Expr>; 4]) -> Result<Expr> {
    for opt_e in shape.iter() {
        if let &Some(expr) = opt_e {
            same_graph_2(arg.borrow(), expr)?;
        }
    }
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::broadcast(&arg.graph, arg.id, [
            shape[0].map(|e| e.borrow().id),
            shape[1].map(|e| e.borrow().id),
            shape[2].map(|e| e.borrow().id),
            shape[3].map(|e| e.borrow().id)
        ])?})
}

pub fn broadcast_to<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg: T1, to: T2) -> Result<Expr> {
    let arg = arg.borrow();
    let to = to.borrow();
    same_graph_2(arg, to)?;
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::broadcast_to(&arg.graph, arg.id, to.id)?
    })
}

pub fn make_constant<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::make_constant(&arg.graph, arg.id)?
    })
}

pub fn reorder<T: Borrow<Expr>>(arg: T, order: [Axis; 4]) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::reorder(&arg.graph, arg.id, Some(order))?
    })
}

pub fn transpose<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::reorder(&arg.graph, arg.id, None)?
    })
}
