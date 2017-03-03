extern crate arrayfire as af;
extern crate gir_core;
extern crate gir_af;

use gir_core::{GraphFunction, Backend, CompiledFunction};
use gir_core::errors::Result;
use std::time::Instant;

fn main() {
    let f = gir_core::make_example_graph().unwrap();
    compile_and_run_af(f).unwrap();
}

pub fn initialize_params(backend: &mut gir_af::AfBackend) -> Result<()> {
    let alpha = af::constant::<f32>(0.001, af::Dim4::new(&[1, 1, 1, 1]));
    backend.set_param_value("alpha", alpha)?;
    let w = af::randn::<f32>(af::Dim4::new(&[1024, 784, 1, 1])) / 30.0f32;
    backend.set_param_value("1::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[1024, 1, 1, 1]));
    backend.set_param_value("1::b", b)?;
    let w = af::randn::<f32>(af::Dim4::new(&[512, 1024, 1, 1])) / 30.0f32;
    backend.set_param_value("2::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[512, 1, 1, 1]));
    backend.set_param_value("2::b", b)?;
    let w = af::randn::<f32>(af::Dim4::new(&[1024, 512, 1, 1])) / 30.0f32;
    backend.set_param_value("3::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[1024, 1, 1, 1]));
    backend.set_param_value("3::b", b)?;
    let w = af::randn::<f32>(af::Dim4::new(&[512, 1024, 1, 1])) / 30.0f32;
    backend.set_param_value("4::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[512, 1, 1, 1]));
    backend.set_param_value("4::b", b)?;
    let w = af::randn::<f32>(af::Dim4::new(&[1024, 512, 1, 1])) / 30.0f32;
    backend.set_param_value("5::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[1024, 1, 1, 1]));
    backend.set_param_value("5::b", b)?;
    let w = af::randn::<f32>(af::Dim4::new(&[10, 1024, 1, 1])) / 30.0f32;
    backend.set_param_value("6::w", w)?;
    let b = af::constant(0f32, af::Dim4::new(&[10, 1, 1, 1]));
    backend.set_param_value("6::b", b)?;
    Ok(())
}

#[allow(unused_variables, unused_mut)]
pub fn compile_and_run_af(func: GraphFunction) -> Result<()> {
    // Initialize backend
    let mut backend = gir_af::AfBackend::default();
    backend.print_general_info().unwrap();
    backend.print_info().unwrap();
    // Initialize parameters
    initialize_params(&mut backend)?;
    // Make inputs
    let input = af::randu::<f32>(af::Dim4::new(&[784, 2000, 1, 1]));
    let target = af::randu::<f32>(af::Dim4::new(&[10, 2000, 1, 1])) * 2.0f32 - 1.0f32;
    let ins = &vec![&input, &target];
    // Compile function
    let mut f = backend.make_function(func);
    // Run 100 iterations
    let mut result = [0.0f32];
    let start = Instant::now();
    for i in 0..1000 {
        f.eval(ins).unwrap().pop().unwrap().host(&mut result);
        println!("Iteration {}: {:.5e}", i, result[0]);
    }
    let duration = start.elapsed();
    println!("1000 iterations took {}.{:>09}s", duration.as_secs(), duration.subsec_nanos());
    Ok(())
}
