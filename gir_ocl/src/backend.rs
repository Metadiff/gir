use gir_core::primitives::*;
use gir_core::graph::*;
use gir_core::backend::*;
use function::*;

use ocl::{Platform, Device, Context, Queue, Buffer};
use ocl::core::DeviceInfo;
use ocl::flags::{MEM_ALLOC_HOST_PTR, MEM_READ_WRITE, CommandQueueProperties};
use std::io;
use std::collections::HashMap;
use tera::Tera;


/// For now this will support only single device
#[derive(Debug, Clone)]
pub struct OpenCLBackend {
    pub platform: Platform,
    pub device: Device,
    pub context: Context,
    pub precisions: BackendPrecisions,
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
        OpenCLBackend {
            platform: platform,
            device: device,
            context: context,
            precisions: BackendPrecisions::default()
        }
    }
}

impl Backend<OpenCLFunction> for OpenCLBackend {
    fn make_function(&self, gf: GraphFunction)
                     -> OpenCLFunction {
        let sym_input_shapes = gf.inputs.iter()
            .map(|&id| gf.graph.nodes[id].shape.clone()).collect();
        let flags = Some(MEM_READ_WRITE | MEM_ALLOC_HOST_PTR);
        let mut kernel_map = HashMap::new();
        let mut tera = compile_templates!("templates/kernels");
        for &i in &gf.graph.order {
            let mut context = ::tera::Context::new();
            let s = type_to_string(gf.graph.nodes[i].data_type, &self.precisions);
            context.add("b_type", &s);
            let s: String = "size_t".into();
            context.add("c_type", &s);
            kernel_map.insert(i, tera.render("store.tera", context).unwrap());
        }
        let queue = Queue::new(&self.context, self.device).unwrap();
        let memory_map = build_memory_map(&gf);
        OpenCLFunction {
            initialized: false,
            precisions: self.precisions,
            gf: gf,
            memory_map: memory_map,
            current_size: 0,
            sym_input_shapes: sym_input_shapes,
            last_shapes: Vec::new(),
            last_deduced: HashMap::new(),
            buffer: Buffer::<u8>::new(queue.clone(), flags, [1] , None).unwrap(),
            buffer_map: HashMap::new(),
            kernel_map: kernel_map,
            queue: queue
        }
    }

    fn get_precisions(&self) -> &BackendPrecisions {
        &self.precisions
    }
    fn set_precisions(&mut self, precisions: BackendPrecisions){
        self.precisions = precisions;
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

pub fn type_to_string(_type: FundamentalType, precisions: &BackendPrecisions) -> String {
    match _type {
        FundamentalType::Boolean => "bool".into(),
        FundamentalType::UnsignedInt => match precisions.integer_precision {
            Precision::P8 => unimplemented!(),
            Precision::P16 => "uint_16".into(),
            Precision::P32 => "uint_32".into(),
            Precision::P64 => "uint_64".into(),
        },
        FundamentalType::SignedInt => match precisions.integer_precision {
            Precision::P8 => unimplemented!(),
            Precision::P16 => "int_16".into(),
            Precision::P32 => "int_32".into(),
            Precision::P64 => "int_64".into(),
        },
        FundamentalType::Float => match precisions.float_precision {
            Precision::P8 => unimplemented!(),
            Precision::P16 => "float_16".into(),
            Precision::P32 => "float_32".into(),
            Precision::P64 => "float_64".into(),
        },
        FundamentalType::UnsignedInt |  FundamentalType::Complex => match precisions.complex_precision {
            Precision::P8 => unimplemented!(),
            Precision::P16 => unimplemented!(),
            Precision::P32 => unimplemented!(),
            Precision::P64 => unimplemented!(),
        }
    }
}
