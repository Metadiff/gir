use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::ops::{Add, Neg, Sub, Mul, Div, DerefMut};
use std::convert::AsRef;
//use std::borrow::Borrow;


pub fn add<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.as_ref();
    let arg1 = arg1.as_ref();
    same_graph_2(arg0, arg1)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::add(g.deref_mut(), vec![arg0.id, arg1.id])?
    };
    wrapper.as_expr(result)
}

pub fn add_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    same_graph_3(arg0, arg1, arg2)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::add(g.deref_mut(), vec![arg0.id, arg1.id, arg2.id])?
    };
    wrapper.as_expr(result)
}

pub fn add_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    same_graph_4(arg0, arg1, arg2, arg3)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::add(g.deref_mut(), vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    };
    wrapper.as_expr(result)
}

pub fn add_n<T: AsRef<Expr>>(args: &Vec<T>) -> Result<Expr> {
    same_graph(args)?;
    let ref wrapper = args[0].as_ref().wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        let args = args.iter().map(|x| x.as_ref().id).collect::<Vec<usize>>();
        ids::add(g.deref_mut(), args)?
    };
    wrapper.as_expr(result)
}

pub fn neg<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::neg(g.deref_mut(), arg.id)?
    };
    wrapper.as_expr(result)
}

pub fn sub<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.as_ref();
    let arg1 = arg1.as_ref();
    same_graph_2(arg0, arg1)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::sub(g.deref_mut(), arg0.id, arg1.id)?
    };
    wrapper.as_expr(result)
}

pub fn mul<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.as_ref();
    let arg1 = arg1.as_ref();
    same_graph_2(arg0, arg1)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::mul(g.deref_mut(), vec![arg0.id, arg1.id])?
    };
    wrapper.as_expr(result)
}

pub fn mul_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    same_graph_3(arg0, arg1, arg2)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::mul(g.deref_mut(), vec![arg0.id, arg1.id, arg2.id])?
    };
    wrapper.as_expr(result)
}

pub fn mul_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    same_graph_4(arg0, arg1, arg2, arg3)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::mul(g.deref_mut(), vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    };
    wrapper.as_expr(result)
}

pub fn mul_n<T: AsRef<Expr>>(args: &Vec<T>) -> Result<Expr> {
    same_graph(args)?;
    let ref wrapper = args[0].as_ref().wrapper;
    let args = args.iter().map(|x| x.as_ref().id).collect();
    let result = {
        let mut g = wrapper.get_mut();
        ids::mul(g.deref_mut(), args)?
    };
    wrapper.as_expr(result)
}

pub fn reciprocal<T: AsRef<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.as_ref();
    let ref wrapper = arg.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::reciprocal(g.deref_mut(), arg.id)?
    };
    wrapper.as_expr(result)
}

pub fn div<T1: AsRef<Expr>, T2: AsRef<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.as_ref();
    let arg1 = arg1.as_ref();
    same_graph_2(arg0, arg1)?;
    let ref wrapper = arg0.wrapper;
    let result = {
        let mut g = wrapper.get_mut();
        ids::div(g.deref_mut(), arg0.id, arg1.id)?
    };
    wrapper.as_expr(result)
}

impl<T> Add<T> for Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn add(self, rhs: T) -> Self::Output {
        self::add(self, rhs).unwrap()
    }
}

impl<'a, T> Add<T> for &'a Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn add(self, rhs: T) -> Self::Output {
        self::add(self, rhs).unwrap()
    }
}

impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        self::neg(self).unwrap()
    }
}

impl<'a> Neg for &'a Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        self::neg(self).unwrap()
    }
}

impl<T> Sub<T> for Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn sub(self, rhs: T) -> Self::Output {
        self::sub(self, rhs).unwrap()
    }
}

impl<'a, T> Sub<T> for &'a Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn sub(self, rhs: T) -> Self::Output {
        self::sub(self, rhs).unwrap()
    }
}

impl<T> Mul<T> for Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn mul(self, rhs: T) -> Self::Output {
        self::mul(self, rhs).unwrap()
    }
}

impl<'a, T> Mul<T> for &'a Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn mul(self, rhs: T) -> Self::Output {
        self::mul(self, rhs).unwrap()
    }
}

impl<T> Div<T> for Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn div(self, rhs: T) -> Self::Output {
        self::div(self, rhs).unwrap()
    }
}

impl<'a, T> Div<T> for &'a Expr where T: AsRef<Expr> {
    type Output = Expr;
    fn div(self, rhs: T) -> Self::Output {
        self::div(self, rhs).unwrap()
    }
}
