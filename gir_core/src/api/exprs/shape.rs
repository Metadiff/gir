use primitives::*;
use graph::*;
use errors::*;
use super::super::ids;
use std::ops::DerefMut;
//use std::borrow::Borrow;

pub fn dim<T: AsRef<Expr>>(arg: T, axis: Axis) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::dim(g.deref_mut(), arg.id, axis)?
    };
    wrapper.as_expr(result)
}

pub fn dim0<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::dim(g.deref_mut(), arg.id, Axis::Axis0)?
    };
    wrapper.as_expr(result)
}

pub fn dim1<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::dim(g.deref_mut(), arg.id, Axis::Axis1)?
    };
    wrapper.as_expr(result)
}

pub fn dim2<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::dim(g.deref_mut(), arg.id, Axis::Axis2)?
    };
    wrapper.as_expr(result)
}

pub fn dim3<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::dim(g.deref_mut(), arg.id, Axis::Axis3)?
    };
    wrapper.as_expr(result)
}

pub fn shape<T: AsRef<Expr>>(arg: T) -> Result<(Expr, Expr, Expr, Expr)> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::shape(g.deref_mut(), arg.id)?
    };
    Ok((wrapper.as_expr(result.0).unwrap(),
        wrapper.as_expr(result.1).unwrap(),
        wrapper.as_expr(result.2).unwrap(),
        wrapper.as_expr(result.3).unwrap()))

}