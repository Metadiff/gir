use graph::*;
use std::io::{Write, Result};
use std::fs::File;

pub trait GraphExporter {
    fn export(&self, io: &mut Write, graph: &Graph) -> Result<()>;

    fn export_expr(&self, io: &mut Write, expr: &ExprData, sep: &str) -> Result<()>;

    fn export_to_file(&self, path: &str, graph: &Graph) -> Result<()> {
        let mut file = File::create(path)?;
        self.export(&mut file, graph)
    }
}

pub mod dot;
pub use self::dot::DotExporter;
