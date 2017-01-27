use primitives::*;
use graph::*;
use errors::*;
use super::super::ids;
use std::borrow::Borrow;

pub fn dim<T: Borrow<Expr>>(arg: T, axis: Axis) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::dim(&arg.graph, arg.id, axis)?
    })
}

pub fn dim0<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::dim(&arg.graph, arg.id, Axis::Axis0)?
    })
}

pub fn dim1<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::dim(&arg.graph, arg.id, Axis::Axis1)?
    })
}

pub fn dim2<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::dim(&arg.graph, arg.id, Axis::Axis2)?
    })
}

pub fn dim3<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::dim(&arg.graph, arg.id, Axis::Axis3)?
    })
}

pub fn shape<T: Borrow<Expr>>(arg: T) -> Result<(Expr, Expr, Expr, Expr)> {
    let arg = arg.borrow();
    let id_shape = ids::shape(&arg.graph, arg.id)?;
    Ok((Expr {graph: arg.graph.clone(), id: id_shape.0},
        Expr {graph: arg.graph.clone(), id: id_shape.1},
        Expr {graph: arg.graph.clone(), id: id_shape.2},
        Expr {graph: arg.graph.clone(), id: id_shape.3},
    ))
}