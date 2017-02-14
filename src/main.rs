#[macro_use]
extern crate gir;
use gir::api::*;
use std::fs::File;

//use gir::backend::opencl::*;
use gir::backend::af::*;

fn main() {
    let f = make_graph().unwrap();
    compile_and_run_af(f);
}

#[allow(unused_variables, unused_mut)]
fn make_graph() -> gir::errors::Result<gir::GraphFunction> {
    // Make the graph
    let g = gir::GraphWrapper::default();
    // Learning rate
    let alpha = &f_param!(g, (), "alpha")?;
    // Dummy
//    let beta = &f_var!(g, ());
    // Input
    let x = &f_var!(g, (784, "n"), "input");
    // Targets
    let y = &f_var!(g, (10, "n"), "target");
//    let y_t = &transpose(y)?;
    // Parameters
    let w1 = &f_param!(g, (10, 784), "w1")?;
    let b1 = &f_param!(g, (10), "b1")?;
    // Calculate outputs
    let h1 = &tanh((mat_mul(w1, x)? + b1))?;
    // Error
    let error = sum_all((h1 - y) * (h1 - y))? / dim1(y)?;
    let params = &vec![w1, b1];
    // Calculate gradients
    let grads = gir::derivative::gradient(&error, &params)?;
    // Generate SGD updates
    g.get_mut().scope.push("updates".into());
    let updates: Vec<(gir::Expr, gir::Expr)> = params.iter().zip(grads.iter())
        .map(|(&& ref p, ref g)| (p.clone(), p - alpha * g)).collect();
    g.get_mut().scope.clear();
    let mut file = File::create("target/html/foo.dot").unwrap();
    gir::export::dot::to_dot(&mut file, &x.wrapper.get()).unwrap();
    let mut file = File::create("target/html/foo.html").unwrap();
    gir::export::cytoscape::to_cytoscape_html(&mut file, &x.wrapper.get()).unwrap();
    // Compile function
    let f = gir::GraphFunction::new_from_expr(&[x.clone(), y.clone()], &[error],
                                              false, &updates[..], Some("test_func".into()))?;
    Ok(f)
}

//static SRC: &'static str = r#"
//    __kernel void multiply(__global float* buffer, float coeff) {
//        buffer[get_global_id(0)] += coeff;
//    }
//"#;

#[macro_use(af_print)]
extern crate arrayfire as af;
use af::print_gen;

#[allow(unused_variables, unused_mut)]
pub fn compile_and_run_af(func: gir::GraphFunction) {
    // Initialize backend
    let mut backend = AfBackend::default();
    backend.print_general_info().unwrap();
    // Initialize parameters
    let alpha = af::constant::<f32>(0.001, af::Dim4::new(&[1, 1, 1, 1]));
    backend.set_param_value("alpha", alpha);
    let w1 = af::randn::<f32>(af::Dim4::new(&[1, 784, 1, 1])) / 100.0f32;
    backend.set_param_value("w1", w1);
    let b1 = af::randn::<f32>(af::Dim4::new(&[1, 1, 1, 1])) / 100.0f32;
    backend.set_param_value("b1", b1);
    // Make inputs
    let input = af::randu::<f32>(af::Dim4::new(&[784, 2000, 1, 1]));
    let target = af::randu::<f32>(af::Dim4::new(&[10, 2000, 1, 1]));
    let ins = &vec![&input, &target];
    // Compile function
    let mut f = backend.make_function(func);
    // Run 100 iterations
    let mut result = [0.0f32];
    for i in 0..100 {
        f.eval(ins).unwrap().pop().unwrap().host(&mut result);
        println!("Iteration {}: {:.5e}", i, result[0]);
    }
}