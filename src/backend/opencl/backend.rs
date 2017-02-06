use graph::*;
use backend::*;
use backend::opencl::function::*;

use ocl::{Platform, Device, Context, Queue};
use ocl::core::DeviceInfo;
use std::io;
use std::collections::HashMap;


/// For now this will support only single device
#[derive(Debug, Clone)]
pub struct OpenCLBackend {
    pub platform: Platform,
    pub device: Device,
    pub context: Context,
    pub queue: Queue,
}


impl Default for OpenCLBackend {
    fn default() -> Self {
        // Todo similar to GraphProps this should be loaded from system file
        // and environmental variables
        let platform = Platform::list().pop().unwrap();
        let device = Device::list_all(&platform).unwrap()[0];
        let context = Context::builder()
            .platform(platform)
            .devices(device)
            .build()
            .unwrap();
        let queue = Queue::new(&context, device).unwrap();
        OpenCLBackend {
            platform: platform,
            device: device,
            context: context,
            queue: queue,
        }
    }
}

impl Backend<OpenCLFunction> for OpenCLBackend {
    fn make_function(&self, gf: GraphFunction)
                     -> OpenCLFunction {
        let sym_input_shapes = gf.inputs.iter()
            .map(|&id| gf.graph.nodes[id].shape.clone()).collect();
        OpenCLFunction {
            memory_map: MemoryMap::default(),
            gf: gf,
            initialized: false,
            sym_input_shapes: sym_input_shapes,
            last_shapes: Vec::new(),
            last_deduced: HashMap::new(),
        }
    }

    fn info(&self, f:&mut io::Write) -> io::Result<()> {
        writeln!(f, "OpenCL Backend Information:")?;
        // Todo: when String.repeat() becomes stable exchange
        // writeln!(f, "{}", "=".repeat(50))?;
        writeln!(f, "==================================================")?;
        writeln!(f, "Platform:\n\
            {t}Profile: {}\n\
            {t}Version: {}\n\
            {t}Name: {}\n\
            {t}Vendor: {}",
                 self.platform.profile(),
                 self.platform.version(),
                 self.platform.name(),
                 self.platform.vendor(),
                 t = "\t")?;
        writeln!(f, "{t}Device: \n\
                {t}{t}Name: {}\n\
                {t}{t}Type: {}\n\
                {t}{t}Vendor: {}[{}]\n\
                {t}{t}MaxComputeUnits: {}\n\
                {t}{t}MaxWorkItemDimensions: {}\n\
                {t}{t}MaxWorkGroupSize: {}\n\
                {t}{t}MaxWorkItemSizes: {}",
                 self.device.name(),
                 self.device.info(DeviceInfo::Type),
                 self.device.info(DeviceInfo::Vendor),
                 self.device.info(DeviceInfo::VendorId),
                 self.device.info(DeviceInfo::MaxComputeUnits),
                 self.device.info(DeviceInfo::MaxWorkItemDimensions),
                 self.device.info(DeviceInfo::MaxWorkGroupSize),
                 self.device.info(DeviceInfo::MaxWorkItemSizes),
                 t = "\t")?;
        // Todo: when String.repeat() becomes stable exchange
        // writeln!(f, "{}", "=".repeat(50))?;
        writeln!(f, "==================================================")?;
        Ok(())
    }

    fn general_info(&self, f: &mut io::Write) -> io::Result<()> {
        writeln!(f, "OpenCL Backend General Information:")?;
        writeln!(f, "==================================================")?;
        for (p_id, platform) in Platform::list().iter().enumerate() {
            let devices = Device::list_all(platform).unwrap();
            writeln!(f, "Platform[{}]:\n\
            {t}Profile: {}\n\
            {t}Version: {}\n\
            {t}Name: {}\n\
            {t}Vendor: {}\n\
            {t}Total Device Count: {}",
                     p_id,
                     platform.profile(),
                     platform.version(),
                     platform.name(),
                     platform.vendor(),
                     devices.len(),
                     t = "\t")?;
            for (d_id, device) in devices.iter().enumerate() {
                writeln!(f, "{t}Device[{}]: \n\
                {t}{t}Name: {}\n\
                {t}{t}Type: {}\n\
                {t}{t}Vendor: {}[{}]\n\
                {t}{t}MaxComputeUnits: {}\n\
                {t}{t}MaxWorkItemDimensions: {}\n\
                {t}{t}MaxWorkGroupSize: {}\n\
                {t}{t}MaxWorkItemSizes: {}",
                         d_id,
                         device.name(),
                         device.info(DeviceInfo::Type),
                         device.info(DeviceInfo::Vendor),
                         device.info(DeviceInfo::VendorId),
                         device.info(DeviceInfo::MaxComputeUnits),
                         device.info(DeviceInfo::MaxWorkItemDimensions),
                         device.info(DeviceInfo::MaxWorkGroupSize),
                         device.info(DeviceInfo::MaxWorkItemSizes),
                         t = "\t",
                )?;
            }
            writeln!(f, "==================================================")?;
        }
        Ok(())
    }
}

//impl OpenCLBackend {
//    pub fn process_graph(&mut self, graph: &Graph) {
//        let mut kernel_map = HashMap::new();
//        let mut kernels: Vec<String> = Vec::new();
//        for &id in &graph.order {
//            let ref node = graph.nodes[id];
//            let meta = node.op.get_meta();
//            match meta.name {
//                "Input" | "Parameter" | "Scalar" => {},
//                "Add" => {
//                    match node.ancestors.len() {
//                        2 => {
//                            let name = "add_2_float_32";
//                            if kernel_map.get(name).is_none() {
//                                let kernel = format!(
//                                    "__kernel void multiply(__global float* out,
//                                    __global float* in1,
//                                    __global float* in2){{
//                                        auto id = get_global_id(0);
//                                        out[id] = in1[id] + in2[id];
//                                    }}");
//                                kernel_map.insert(name, kernel);
//                            }
//                        },
//                        _ => {}
//                    }
//                }
//                _ => {}
//            }
//        }
//    }
//
//
//    pub fn make_program(&mut self, source: &str) {
//        self.program = Some(Program::builder()
//            .src(source)
//            .devices(self.device)
//            .build(&self.context).unwrap());
//    }
//
//    pub fn execute_kernel(&self, kernel_name: &str) {
//        let dims = &[64];
//        let buffer = Buffer::<f32>::new(self.queue.clone(), None, dims, None).unwrap();
//        let kernel = Kernel::new(kernel_name, self.program.as_ref().unwrap(), &self.queue).unwrap()
//            .gws(&[10])
//            .arg_buf(&buffer)
//            .arg_scl(10.0f32);
//
//        let mut event_list = EventList::new();
//
//        let mut result = vec![1.0f32; dims[0]];
//        let mut event = Event::empty();
//        buffer.cmd().write(&result).enq().unwrap();
//        kernel.cmd().enq().unwrap();
//        buffer.cmd().read(&mut result).enew(&mut event).enq().unwrap();
//        event_list.wait().unwrap();
//        println!("{:?}", result);
//    }
//
//
//}