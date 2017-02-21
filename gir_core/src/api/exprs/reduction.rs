use primitives::*;
use graph::*;
use errors::*;
use super::super::ids;
use std::ops::DerefMut;
//use std::borrow::Borrow;

pub fn sum<T: AsRef<Expr>>(arg: T, axis: Axis) -> Result<Expr> {
    let arg = arg.as_ref();
    let mut axes = [false; 4];
    axes[axis as usize] = true;
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::sum(g.deref_mut(), arg.id, axes)?
    };
    wrapper.as_expr(result)
}

pub fn sum_all<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::sum(g.deref_mut(), arg.id, [true; 4])?
    };
    wrapper.as_expr(result)
}

pub fn sum_axes<T: AsRef<Expr>>(arg: T, axes: [bool; 4]) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::sum(g.deref_mut(), arg.id, axes)?
    };
    wrapper.as_expr(result)
}

