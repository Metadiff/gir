use gir_core::primitives::*;
use gir_core::graph::*;
use gir_core::backend::*;
use gir_core::errors::*;
use ocl::{Buffer, Queue};
use ocl::flags::MemFlags;
use ocl::core::{create_sub_buffer, BufferRegion, Mem};

use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct OpenCLContainer {
    pub mem: Vec<f32>,
    pub dims: [usize; 4]
}

#[derive(Debug, Clone)]
pub struct OpenCLFunction {
    pub initialized: bool,
    pub precisions: BackendPrecisions,
    pub gf: GraphFunction,
    pub memory_map: AbstractMemoryMap,
    pub current_size: usize,
    pub sym_input_shapes: Vec<Shape>,
    pub last_shapes: Vec<[usize; 4]>,
    pub last_deduced: HashMap<String, i64>,
    pub buffer: Buffer<u8>,
    pub buffer_map: HashMap<usize, Mem>,
    pub kernel_map: HashMap<usize, String>,
    pub queue: Queue
}

impl OpenCLFunction {
    fn allocate(&mut self) {
        // Calculate memory for each type
//        let size_b = self.memory_map.abstract_size.0.eval(&self.last_deduced).unwrap();
//        let size_i = self.memory_map.abstract_size.1
//            .eval(&self.last_deduced).unwrap() *
//            self.precisions.integer_precision as i64;
//        let size_f = self.memory_map.abstract_size.2
//            .eval(&self.last_deduced).unwrap() *
//            self.precisions.float_precision as i64;
//        let size_c = self.memory_map.abstract_size.3
//            .eval(&self.last_deduced).unwrap() * 2 *
//            self.precisions.complex_precision as i64;
//        // Full size of allocation
//        self.current_size = (size_b + size_i + size_f + size_c) as usize;
//        // Allocate full memory
//        let flags = Some(MemFlags::alloc_host_ptr() | MemFlags::read_write());
//        self.buffer = Buffer::<u8>::new(self.queue.clone(), flags, self.current_size, None).unwrap();
//        if self.current_size > 1024 * 1024 {
//            println!("Allocating {:.2} MB of memory.", self.current_size as f64 / (1024.0 * 1024.0));
//        } else if self.current_size > 1024 {
//            println!("Allocating {:.2} KB of memory.", self.current_size as f64 / 1024.0);
//        } else {
//            println!("Allocating {:.2} B of memory.", self.current_size);
//        }
//        let mut map = HashMap::new();
//        // Create buffers for each node
//        for (&id, &(offset, size)) in self.memory_map.abstract_map.iter() {
//            let offset = offset.eval(&self.last_deduced).unwrap();
//            let size = size.eval(&self.last_deduced).unwrap();
//            let sub =  match self.gf.graph.nodes[id].data_type {
//                FundamentalType::Boolean => {
//                    create_sub_buffer::<bool>(&self.buffer,
//                                              MemFlags::read_write(),
//                                              &BufferRegion::new(offset, size)).unwrap();
//                },
//                FundamentalType::UnsignedInt => match self.precisions.integer_precision {
//                    Precision::P8 => unimplemented!(),
//                    Precision::P16 => {
//                        let offset = offset / 2;
//                        create_sub_buffer::<u16>(&self.buffer,
//                                                 MemFlags::read_write(),
//                                                 &BufferRegion::new(offset, size)).unwrap();
//                    }
//                    Precision::P32 => unimplemented!(),
//                    Precision::P64 => unimplemented!(),
//                    create_sub_buffer::<u32>(&self.buffer, MemFlags::read_write(), &BufferRegion::new(offset, size)).unwrap();
//                },
//                FundamentalType::SignedInt => {
//                    create_sub_buffer::<i32>(&self.buffer, MemFlags::read_write(), &BufferRegion::new(offset, size)).unwrap();
//                },
//                _ => {
//                    create_sub_buffer::<f32>(&self.buffer, MemFlags::read_write(), &BufferRegion::new(offset, size)).unwrap();
//                },
//            };
//            map.insert(id, sub);
//        }
        // Create sub buffers for each node
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
    fn eval(&mut self, inputs: &[&OpenCLContainer]) -> Result<Vec<OpenCLContainer>> {
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
                self.allocate();
            },
            None => {}
        }
        // Todo copy inputs data to the inputs ocl::Buffers
        self.internal_eval();
        // Todo copy outputs ocl::Buffer to OpenCLContainers
        Ok(Vec::new())
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
    fn free_memory(&mut self) {
//        self.buffer = Buffer::<u8>::new(self.queue.clone(), None, 1, None).unwrap();
    }
}
