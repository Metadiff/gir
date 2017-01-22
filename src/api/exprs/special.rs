use primitives::*;
use graph::*;
use errors::*;
use super::super::ids;

pub fn cast(arg: &Expr, data_type: FundamentalType) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::cast(&arg.graph, arg.id, data_type)?
    })
}

pub fn broadcast(arg: &Expr, to: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::broadcast(&arg.graph, arg.id, to.id)?
    })
}

pub fn make_constant(arg: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::make_constant(&arg.graph, arg.id)?
    })
}
