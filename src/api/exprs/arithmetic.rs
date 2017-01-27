use graph::*;
use errors::*;
use ops::interface::default::*;
use super::super::ids;
use std::ops::{Add, Neg, Sub, Mul, Div};
use std::borrow::Borrow;


pub fn add<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.borrow();
    let arg1 = arg1.borrow();
    same_graph_2(arg0, arg1)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, vec![arg0.id, arg1.id])?
    })
}

pub fn add_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    same_graph_3(arg0, arg1, arg2)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, vec![arg0.id, arg1.id, arg2.id])?
    })
}

pub fn add_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    same_graph_4(arg0, arg1, arg2, arg3)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    })
}

pub fn add_n<T: Borrow<Expr>>(args: &Vec<T>) -> Result<Expr> {
    same_graph(args)?;
    Ok(Expr {
        graph: args[0].borrow().graph.clone(),
        id: ids::add(&args[0].borrow().graph, args.iter().map(|x| x.borrow().id).collect())?
    })
}

pub fn neg<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::neg(&arg.graph, arg.id)?
    })
}

pub fn sub<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.borrow();
    let arg1 = arg1.borrow();
    same_graph_2(arg0, arg1)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::sub(&arg0.graph, arg0.id, arg1.id)?
    })
}

pub fn mul<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.borrow();
    let arg1 = arg1.borrow();
    same_graph_2(arg0, arg1)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, vec![arg0.id, arg1.id])?
    })
}

pub fn mul_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    same_graph_3(arg0, arg1, arg2)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, vec![arg0.id, arg1.id, arg2.id])?
    })
}

pub fn mul_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    same_graph_4(arg0, arg1, arg2, arg3)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    })
}

pub fn mul_n<T: Borrow<Expr>>(args: &Vec<T>) -> Result<Expr> {
    same_graph(args)?;
    Ok(Expr {
        graph: args[0].borrow().graph.clone(),
        id: ids::mul(&args[0].borrow().graph, args.iter().map(|x| x.borrow().id).collect())?
    })
}

pub fn reciprocal<T: Borrow<Expr>>(arg: T) -> Result<Expr> {
    let arg = arg.borrow();
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::reciprocal(&arg.graph, arg.id)?
    })
}

pub fn div<T1: Borrow<Expr>, T2: Borrow<Expr>>(arg0: T1, arg1: T2) -> Result<Expr> {
    let arg0 = arg0.borrow();
    let arg1 = arg1.borrow();
    same_graph_2(arg0, arg1)?;
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::div(&arg0.graph, arg0.id, arg1.id)?
    })
}

impl<T> Add<T> for Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn add(self, rhs: T) -> Self::Output {
        self::add(self, rhs).unwrap()
    }
}

impl<'a, T> Add<T> for &'a Expr where T: Borrow<Expr> {
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

impl<T> Sub<T> for Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn sub(self, rhs: T) -> Self::Output {
        self::sub(self, rhs).unwrap()
    }
}

impl<'a, T> Sub<T> for &'a Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn sub(self, rhs: T) -> Self::Output {
        self::sub(self, rhs).unwrap()
    }
}

impl<T> Mul<T> for Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn mul(self, rhs: T) -> Self::Output {
        self::mul(self, rhs).unwrap()
    }
}

impl<'a, T> Mul<T> for &'a Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn mul(self, rhs: T) -> Self::Output {
        self::mul(self, rhs).unwrap()
    }
}

impl<T> Div<T> for Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn div(self, rhs: T) -> Self::Output {
        self::div(self, rhs).unwrap()
    }
}

impl<'a, T> Div<T> for &'a Expr where T: Borrow<Expr> {
    type Output = Expr;
    fn div(self, rhs: T) -> Self::Output {
        self::div(self, rhs).unwrap()
    }
}
