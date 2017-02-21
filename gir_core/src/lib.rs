#![feature(ptr_eq)]
extern crate symbolic_polynomials;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate slog;
extern crate slog_term;
//#[macro_use]
//extern crate tera;
//extern crate ocl;
//#[macro_use(af_print)]
//extern crate arrayfire;

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

#[allow(unused_variables, unused_mut)]
pub fn make_example_graph() -> errors::Result<GraphFunction> {
    // Make the graph
    let g = GraphWrapper::default();
    // Learning rate
    let alpha = &f_param!(g, (), "alpha")?;
    // Dummy
    let beta = &f_var!(g, ());
    // Input
    let x = &f_var!(g, (784, "n"), "input");
    // Targets
    let y = &f_var!(g, (10, "n"), "target");
    // Parameters
    let w1 = &f_param!(g, (10, 784), "w1")?;
    let b1 = &f_param!(g, (10), "b1")?;
    // Calculate outputs
    let h1 = &api::tanh((api::mat_mul(w1, x)? + b1))?;
    // Error
    let error = api::sum_all((h1 - y) * (h1 - y))? / api::dim1(y)?;
    let params = &vec![w1, b1];
    // Calculate gradients
    let grads = derivative::gradient(&error, &params)?;
    // Generate SGD updates
    g.get_mut().scope.push("updates".into());
    let updates: Vec<(Expr, Expr)> = params.iter().zip(grads.iter())
        .map(|(&& ref p, ref g)| (p.clone(), p - alpha * g)).collect();
    g.get_mut().scope.clear();
    let f = GraphFunction::new_from_expr(&[x.clone(), y.clone()], &[error],
                                              false, &updates[..], Some("test_func".into()))?;
    Ok(f)
}
