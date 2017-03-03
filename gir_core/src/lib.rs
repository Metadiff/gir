#![feature(ptr_eq)]
extern crate symbolic_polynomials;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate slog;
extern crate slog_term;

pub mod primitives;
pub mod errors;
pub mod props;
pub mod graph;

pub mod ops;
#[macro_use]
pub mod api;
pub mod derivative;
pub mod utils;
pub mod export;
pub mod backend;

pub use primitives::*;
pub use graph::*;
pub use backend::*;

use std::collections::HashMap;

pub fn layer(x: &Expr, num_units: usize, params: &mut Vec<Expr>, prefix: &str) -> errors::Result<Expr> {
    let g = x.wrapper.clone();
    let d: usize = x.get()?.shape.0.eval(&HashMap::new()).unwrap() as usize;
    let w = f_param!(g, (num_units, d), format!("{}::w", prefix))?;
    let b = f_param!(g, (num_units), format!("{}::b", prefix))?;
    let out = api::tanh((api::mat_mul(&w, x)? + &b))?;
    params.push(w);
    params.push(b);
    Ok(out)
}

#[allow(unused_variables, unused_mut)]
pub fn make_example_graph() -> errors::Result<GraphFunction> {
    // Make the graph
    let g = GraphWrapper::default();
    // Learning rate
    let alpha = &f_param!(g, (), "alpha")?;
    // Dummy
    let beta = &f_var!(g, ());
    // Input
    let mut x = f_var!(g, (784, "n"), "input");
    // Targets
    let y = f_var!(g, (10, "n"), "target");
    // Parameters
    let mut params = Vec::new();
    // 6 layers
    let mut h = x.clone();
    h = layer(&h, 1024, &mut params, "1")?;
    h = layer(&h, 512, &mut params, "2")?;
    h = layer(&h, 1024, &mut params, "3")?;
    h = layer(&h, 512, &mut params, "4")?;
    h = layer(&h, 1024, &mut params, "5")?;
    h = layer(&h, 10, &mut params, "6")?;
    // Error
    let error = api::sum_all((&h - &y) * (&h - &y))? / api::dim1(&y)?;
    // Calculate gradients
    let grads = derivative::gradient(&error, &params)?;
    // Generate SGD updates
    g.get_mut().scope.push("updates".into());
    let updates: Vec<(Expr, Expr)> = params.iter().zip(grads.iter())
        .map(|(& ref p, ref g)| (p.clone(), p - alpha * g)).collect();
    g.get_mut().scope.clear();
    let f = GraphFunction::new_from_expr(&[x, y], &[error],
                                         false, &updates[..], Some("test_func".into()))?;
    Ok(f)
}
