use primitives::*;
use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::ops::DerefMut;

pub fn overwrite_update<T: AsRef<Expr>>(arg: T, upd: T) -> Result<()> {
    let arg = arg.as_ref();
    let upd = upd.as_ref();
    same_graph_2(arg, upd)?;
    ids::overwrite_update(arg.wrapper.get_mut().deref_mut(), arg.id, upd.id)
}

pub fn update<T: AsRef<Expr>>(arg: T, upd: T) -> Result<()> {
    let arg = arg.as_ref();
    let upd = upd.as_ref();
    same_graph_2(arg, upd)?;
    ids::update(arg.wrapper.get_mut().deref_mut(), arg.id, upd.id)
}

pub fn cast<T: AsRef<Expr>>(arg: T, data_type: FundamentalType) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::cast(g.deref_mut(), arg.id, data_type)?
    };
    wrapper.as_expr(result)
}

pub fn broadcast<T: AsRef<Expr>>(arg: T, shape: [Option<&Expr>; 4]) -> Result<Expr> {
    for opt_e in shape.iter() {
        if let &Some(expr) = opt_e {
            same_graph_2(arg.as_ref(), expr)?;
        }
    }
    let arg = arg.as_ref();
    let shape = [
        shape[0].map(|e| e.as_ref().id),
        shape[1].map(|e| e.as_ref().id),
        shape[2].map(|e| e.as_ref().id),
        shape[3].map(|e| e.as_ref().id)
    ];
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::broadcast(g.deref_mut(), arg.id, shape)?
    };
    wrapper.as_expr(result)
}

pub fn broadcast_to<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg: T1, to: T2) -> Result<Expr> {
    let arg = arg.as_ref();
    let to = to.as_ref();
    same_graph_2(arg, to)?;
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::broadcast_to(g.deref_mut(), arg.id, to.id)?
    };
    wrapper.as_expr(result)
}

pub fn make_constant<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::make_constant(g.deref_mut(), arg.id)?
    };
    wrapper.as_expr(result)
}

pub fn reorder<T: AsRef<Expr>>(arg: T, order: [Axis; 4]) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::reorder(g.deref_mut(), arg.id, Some(order))?
    };
    wrapper.as_expr(result)
}

pub fn transpose<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::reorder(g.deref_mut(), arg.id, None)?
    };
    wrapper.as_expr(result)
}
