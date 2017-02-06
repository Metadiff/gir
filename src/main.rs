#[macro_use]
extern crate gir;
extern crate slog_term;

use gir::api::*;
use gir::backend::opencl::*;
use gir::backend::*;
use std::fs::File;

fn main() {
    let f = make_graph().unwrap();
    compile_and_run(f);
}

#[allow(unused_variables, unused_mut)]
fn make_graph() -> gir::errors::Result<gir::GraphFunction> {
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
    let f = gir::GraphFunction::new_from_expr(&[x.clone(), y.clone()], &[error],
                                                        false, &updates[..], Some("test_func".into()))?;
    println!("{} - {}", g.get().nodes.len(), f.graph.nodes.len());
    let mut file = File::create("target/html/foo.dot").unwrap();
    gir::export::dot::to_dot(&mut file, &f.graph).unwrap();
    let mut file = File::create("target/html/foo.html").unwrap();
    gir::export::cytoscape::to_cytoscape_html(&mut file, &f.graph).unwrap();
    Ok(f)
}

//static SRC: &'static str = r#"
//    __kernel void multiply(__global float* buffer, float coeff) {
//        buffer[get_global_id(0)] += coeff;
//    }
//"#;

#[allow(unused_variables, unused_mut)]
pub fn compile_and_run(func: gir::GraphFunction) {
    let backend = OpenCLBackend::default();
    backend.print_general_info().unwrap();
    backend.print_info().unwrap();
    let mut f = backend.make_function(func);
    let x = OpenCLContainer {
        mem: vec![1.0; 784*10],
        dims: [784, 10, 1, 1]
    };
    let y = OpenCLContainer {
        mem: vec![0.5; 10],
        dims: [10, 1, 1, 1]
    };
    let y_wrong = OpenCLContainer {
        mem: vec![0.5; 12],
        dims: [12, 1, 1, 1]
    };
    // Not correct number of inputs
    println!("{:?}", f.eval(&vec![]));
    // Incorrect size, e.g. x.dim1 = 10, y_wrong.dim0 = 12
    println!("{:?}", f.eval(&vec![x.clone(), y_wrong]));
    // Correct
    println!("{:?}", f.eval(&vec![x, y]));
    for (ref sym, ref ls) in f.sym_input_shapes.iter().zip(f.last_shapes.iter()) {
        println!("{} - {:?}", sym, ls);
    }
    println!("{:?}", f.last_deduced)
}
