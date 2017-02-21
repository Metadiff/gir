extern crate gir_core;
extern crate ocl;
#[macro_use]
extern crate tera;

pub mod function;
pub mod backend;

pub use self::backend::*;
pub use self::function::*;