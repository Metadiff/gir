extern crate gir_core;
use gir_core::export::*;

fn main() {
    let f = gir_core::make_example_graph().unwrap();
    let mut args = ::std::env::args();
    if let Some(file_name) = args.nth(1) {
        println!("Writing graphviz to file: {}.", file_name);
        DotExporter::new().export_to_file(&file_name, &f.graph).unwrap();
    }
}
