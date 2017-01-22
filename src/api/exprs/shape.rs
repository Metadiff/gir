use graph::*;
use errors::*;
use super::super::ids;

pub fn shape(arg: &Expr) -> Result<(Expr, Expr, Expr, Expr)> {
    let id_shape = ids::shape(&arg.graph, arg.id)?;
    Ok((Expr {graph: arg.graph.clone(), id: id_shape.0},
        Expr {graph: arg.graph.clone(), id: id_shape.1},
        Expr {graph: arg.graph.clone(), id: id_shape.2},
        Expr {graph: arg.graph.clone(), id: id_shape.3},
    ))
}