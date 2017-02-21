extern crate gir_core;
extern crate gir_ocl;

use gir_core::{GraphFunction, Backend, CompiledFunction};
use gir_core::errors::Result;

fn main() {
    let f = gir_core::make_example_graph().unwrap();
    compile_and_run_af(f).unwrap();
}

#[allow(unused_variables, unused_mut)]
pub fn compile_and_run_af(func: GraphFunction) -> Result<()> {
    // Initialize backend
    let mut backend = gir_ocl::OpenCLBackend::default();
    backend.print_general_info().unwrap();
    backend.print_info().unwrap();
    // Initialize parameters
//    let alpha = af::constant::<f32>(0.001, af::Dim4::new(&[1, 1, 1, 1]));
//    backend.set_param_value("alpha", alpha)?;
//    let w1 = af::randn::<f32>(af::Dim4::new(&[1, 784, 1, 1])) / 100.0f32;
//    backend.set_param_value("w1", w1)?;
//    let b1 = af::randn::<f32>(af::Dim4::new(&[1, 1, 1, 1])) / 100.0f32;
//    backend.set_param_value("b1", b1)?;
//    // Make inputs
//    let input = af::randu::<f32>(af::Dim4::new(&[784, 2000, 1, 1]));
//    let target = af::randu::<f32>(af::Dim4::new(&[10, 2000, 1, 1]));
//    let ins = &vec![&input, &target];
//    // Compile function
//    let mut f = backend.make_function(func);
//    // Run 100 iterations
//    let mut result = [0.0f32];
//    for i in 0..100 {
//        f.eval(ins).unwrap().pop().unwrap().host(&mut result);
//        println!("Iteration {}: {:.5e}", i, result[0]);
//    }
    Ok(())
}
