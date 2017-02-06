#![feature(ptr_eq)]
extern crate symbolic_polynomials;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate tera;
extern crate ocl;

pub mod primitives;
pub mod errors;
pub mod props;
pub mod graph;
//pub mod function;
pub mod ops;
pub mod api;
pub mod derivative;
pub mod utils;
pub mod export;
pub mod backend;

pub use primitives::*;
pub use graph::*;
pub use backend::*;
