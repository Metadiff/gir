use primitives::*;
use graph::*;
use backend::*;
use errors::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OpenCLContainer {
    pub mem: Vec<f32>,
    pub dims: [usize; 4]
}

#[derive(Debug, Clone)]
pub struct OpenCLFunction {
    pub memory_map: MemoryMap,
    pub gf: GraphFunction,
    pub initialized: bool,
    pub last_shapes: Vec<[usize;4]>,
    pub sym_input_shapes: Vec<Shape>,
    pub last_deduced: HashMap<String, i64>
}

impl OpenCLFunction {
    fn allocate_buffer(&mut self) {
        // Todo
        // Based on self.last_deduced should evaluate all of the memory needed
        // and allocate ocl::Buffer accordingly
    }

    /// This could not fail (in theory)
    fn internal_eval(&self) {
        // Todo
        // Run all of the kernels
    }
}

impl CompiledFunction<OpenCLContainer, OpenCLContainer> for OpenCLFunction {
    fn initialized(&self) -> bool {
        self.initialized
    }

    fn free_memory(&mut self) {
        // Free all of the OpenCL Buffers
    }

    fn eval(&mut self, inputs: &[OpenCLContainer]) -> Result<Vec<OpenCLContainer>> {
        // Check correct number of inputs are provided
        if inputs.len() != self.gf.inputs.len() {
            return Err(ErrorKind::Msg(format!("Incorrect number of inputs. \
            Expected: {}, actual: {}.", self.gf.inputs.len(), inputs.len())).into());
        }
        let input_shapes: Vec<[usize;4]> = inputs.iter().map(|x| x.dims.clone()).collect();
        // Check shapes are correct and if they have changed
        match verify_shapes(&input_shapes, &self.last_shapes, &self.sym_input_shapes)? {
            Some(deduced) => {
                self.last_shapes = input_shapes;
                self.last_deduced = deduced;
                // Allocate memory as needed for the new exact shapes
                self.allocate_buffer();
            },
            None => {}
        }
        // Todo copy inputs data to the inputs ocl::Buffers
        self.internal_eval();
        // Todo copy outputs ocl::Buffer to OpenCLContainers
        Ok(Vec::new())
    }
}
