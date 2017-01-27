use primitives::*;
use graph::*;
use errors::*;
use super::super::ids;
use std::borrow::Borrow;

pub fn sum<T: Borrow<Expr>>(arg: T, axis: Axis) -> Result<Expr> {
    let arg = arg.borrow();
    let mut axes = [false; 4];
    axes[axis as usize] = true;
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::sum(&arg.graph, arg.id, axes)?})
}

pub fn sum_all<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::sum(&arg.graph, arg.id, [true; 4])?})
}

pub fn sum_axes<T: Borrow<Expr>>(arg: T, axes: [bool; 4]) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::sum(&arg.graph, arg.id, axes)?})
}

