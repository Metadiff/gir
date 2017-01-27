#[macro_use]
extern crate graph_ir;
extern crate slog_term;

use graph_ir as gir;
use gir::api::*;
use std::fs::File;

fn test() -> gir::errors::Result<()> {
    let g = gir::Graph::default();
    let x = &f_var!(g, (784, "n"), "input");
    let y = &f_var!(g, ("n"), "target");
    let y_t = &transpose(y)?;
    let w1 = &f_param!(g, (1, 784), "w1")?;
    let b1 = &f_param!(g, (1), "b1")?;
    let h1 = &tanh((mat_mul(w1, x)? + b1))?;
    let error = sum_all((h1 - y_t) * (h1 - y_t))? / dim0(y)?;
    let params = &vec![w1, b1];
    let grads = gir::derivative::gradient(error, params)?;
    println!("{:?}", grads.iter().map(|x| x.id).collect::<Vec<usize>>());
    gir::export::dot::to_dot(&mut ::std::io::stdout(), &g).unwrap();
    let mut f = File::create("target/html/foo.html").unwrap();
    gir::export::cytoscape::to_cytoscape_html(&mut f, &g).unwrap();
    Ok(())
}

fn main() {
    test().unwrap();
}
