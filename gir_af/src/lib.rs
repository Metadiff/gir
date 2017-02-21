extern crate gir_core;
extern crate arrayfire;

pub mod backend;
pub mod function;

pub use self::backend::*;
pub use self::function::*;