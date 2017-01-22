use graph::*;
use errors::*;
use super::super::ids;

pub fn add_2(arg0: &Expr, arg1: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, &vec![arg0.id, arg1.id])?
    })
}

pub fn add_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, &vec![arg0.id, arg1.id, arg2.id])?
    })
}

pub fn add_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::add(&arg0.graph, &vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    })
}

pub fn add_n(args: &Vec<Expr>) -> Result<Expr> {
    Ok(Expr {
        graph: args[0].graph.clone(),
        id: ids::add(&args[0].graph, &args.iter().map(|x| x.id).collect())?
    })
}

pub fn neg(arg: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::neg(&arg.graph, arg.id)?
    })
}

pub fn sub(arg0: &Expr, arg1: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::sub(&arg0.graph, arg0.id, arg1.id)?
    })
}

pub fn mul_2(arg0: &Expr, arg1: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, &vec![arg0.id, arg1.id])?
    })
}

pub fn mul_3(arg0: &Expr, arg1: &Expr, arg2: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, &vec![arg0.id, arg1.id, arg2.id])?
    })
}

pub fn mul_4(arg0: &Expr, arg1: &Expr, arg2: &Expr, arg3: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::mul(&arg0.graph, &vec![arg0.id, arg1.id, arg2.id, arg3.id])?
    })
}

pub fn mul_n(args: &Vec<Expr>) -> Result<Expr> {
    Ok(Expr {
        graph: args[0].graph.clone(),
        id: ids::mul(&args[0].graph, &args.iter().map(|x| x.id).collect())?
    })
}

pub fn reciprocal(arg: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg.graph.clone(),
        id: ids::reciprocal(&arg.graph, arg.id)?
    })
}

pub fn div(arg0: &Expr, arg1: &Expr) -> Result<Expr> {
    Ok(Expr {
        graph: arg0.graph.clone(),
        id: ids::div(&arg0.graph, arg0.id, arg1.id)?
    })
}



