use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::convert::AsRef;
use std::ops::DerefMut;
//use std::borrow::Borrow;

pub fn mat_mul<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.as_ref();
    let arg1 = arg1.as_ref();
    same_graph_2(arg0, arg1)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::mat_mul(g.deref_mut(), arg0.id, arg1.id)?
    };
    wrapper.as_expr(result)
}


