use primitives::*;
use graph::*;
use backend::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::io;
use af::function::AfFunction;
use arrayfire as af;
use arrayfire::Array;


/// For now this will support only single device
#[derive(Clone)]
pub struct AfBackend {
    pub platform: ::arrayfire::Backend,
    pub device: i32,
    pub parameters: Rc<RefCell<HashMap<String, Array>>>,
    pub precisions: BackendPrecisions
}


impl Default for AfBackend {
    fn default() -> Self {
        // Todo similar to GraphProps this should be loaded from system file
        AfBackend {
            platform: ::arrayfire::Backend::DEFAULT,
            device: 0,
            parameters: Rc::new(RefCell::new(HashMap::new())),
            precisions: BackendPrecisions::default()
        }
    }
}

impl AfBackend {
    pub fn get_param_value(&self, name: &str) -> Ref<Array> {
        Ref::map(self.parameters.borrow(), |x| x.get(name).unwrap())
    }

    pub fn set_param_value(&mut self, name: &str, value:Array) -> Result<(), String> {
        if let Some(v) =  self.parameters.borrow().get(name) {
            if v.dims() != value.dims() {
                return Err(format!("The parameter {} has shape {}, \
                but {} was passed to set_param_value.", name, v.dims(), value.dims()))
            }
        }
        self.parameters.borrow_mut().insert(name.into(), value);
        Ok(())
    }
}

impl Backend<AfFunction> for AfBackend {
    fn make_function(&self, gf: GraphFunction)
                     -> AfFunction {
        let sym_input_shapes = gf.inputs.iter()
            .map(|&id| gf.graph.nodes[id].shape.clone()).collect();
        AfFunction {
            initialized: false,
            precisions: self.precisions,
            gf: gf,
            parameters: self.parameters.clone(),
            sym_input_shapes: sym_input_shapes,
            last_shapes: Vec::new(),
            last_deduced: HashMap::new(),
            expr_map: HashMap::new()
        }
    }

    fn get_precisions(&self) -> &BackendPrecisions {
        &self.precisions
    }
    fn set_precisions(&mut self, precisions: BackendPrecisions){
        self.precisions = precisions;
    }
    fn info(&self, f:&mut io::Write) -> io::Result<()> {
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "\tDevices: {}", af::device_count())
    }

    fn general_info(&self, f: &mut io::Write) -> io::Result<()> {
        let backend = af::get_active_backend();
        writeln!(f, "Arrayfire Backend General Information:")?;
        writeln!(f, "==================================================")?;
        for b in af::get_available_backends() {
            writeln!(f, "Platform: {}", b)?;
            writeln!(f, "\tDevices: {}", af::device_count())?;
            af::set_backend(b);
            af::info();
        }
        af::set_backend(backend);
        Ok(())
    }

    fn print_info(&self) -> io::Result<()> {
        Ok(::arrayfire::info())
    }
}
