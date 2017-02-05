#[macro_use]
extern crate gir;
extern crate slog_term;

use gir::api::*;
use std::fs::File;

fn test() -> gir::errors::Result<()> {
    // Make the graph
    let g = gir::GraphWrapper::default();
    // Learning rate
    let alpha = &f_param!(g, (), "alpha")?;
    // Dummy
    let beta = &f_var!(g, ());
    // Input
    let x = &f_var!(g, (784, "n"), "input");
    // Targets
    let y = &f_var!(g, ("n"), "target");
    let y_t = &transpose(y)?;
    // Parameters
    let w1 = &f_param!(g, (1, 784), "w1")?;
    let b1 = &f_param!(g, (1), "b1")?;
    // Calculate outputs
    let h1 = &tanh((mat_mul(w1, x)? + b1))?;
    // Error
    let error = sum_all((h1 - y_t) * (h1 - y_t))? / dim0(y)?;
    let params = &vec![w1, b1];
    // Calculate gradients
    let grads = gir::derivative::gradient(&error, &params)?;
    // Generate SGD updates
    g.get_mut().scope.push("updates".into());
    let updates: Vec<(gir::Expr, gir::Expr)> = params.iter().zip(grads.iter())
        .map(|(ref p, ref g)| ((**p).clone(), **p - alpha * g)).collect();
    g.get_mut().scope.clear();
    // Compile function
    let f = gir::function::GraphFunction::new_from_expr(&[x.clone(), y.clone()], &[error],
        false, &updates[..], Some("test_func".into()))?;
    println!("{} - {}", g.get().nodes.len(), f.graph.nodes.len());
    gir::export::dot::to_dot(&mut ::std::io::stdout(), &f.graph).unwrap();
    let mut file = File::create("target/html/foo.html").unwrap();
    gir::export::cytoscape::to_cytoscape_html(&mut file, &f.graph).unwrap();
    Ok(())
}

fn main() {
    test().unwrap();
}
