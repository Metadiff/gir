use graph::*;
use errors::*;
use super::super::ids;
use std::ops::DerefMut;
//use std::borrow::Borrow;

pub fn tanh<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::tanh(g.deref_mut(), arg.id)?
    };
    wrapper.as_expr(result)
}


