use graph::*;
use std::io::{Write, Result};
use export::GraphExporter;

#[derive(Debug, Clone)]
pub struct DotExporter {

}

impl DotExporter {
    pub fn new() -> Self {
        DotExporter{}
    }
}

impl GraphExporter for DotExporter {
    fn export(&self, io: &mut Write, graph: &Graph) -> Result<()> {
        let mut edges: Vec<(usize, usize, usize)> = Vec::new();
        writeln!(io, "digraph g {{")?;
        for expr in &graph.nodes {
            self.export_expr(io, expr, &graph.props.scope_delimiter)?;
            for (i, &anc) in expr.ancestors.iter().enumerate() {
                edges.push((anc, expr.id, i));
            }
        }
        for e in edges {
            writeln!(io, "\tN{} -> N{} [label=\"{}\"];", e.0, e.1, e.2)?;
        }
        writeln!(io, "}}")
    }

    fn export_expr(&self, io: &mut Write, expr: &ExprData, sep: &str) -> ::std::io::Result<()>  {
        let scope = if expr.scope.len() == 0 {
            "base".into()
        } else {
            expr.scope.join(sep)
        };
        let op = expr.op.get_meta();
        let color = match op.name {
            "Input" => "orange",
            "Parameter" => "green",
            "Scalar" => "yellow",
            _ => "blue"
        };
        writeln!(io, "subgraph cluster_{} {{\n\
    label = \"{}\";\n\
    \tN{} [shape=rectangle, style=filled, fillcolor={},\n\
    label=\"\
    {}{:?}\\n\
    id:{}\\n\
    shape:{}\\n\
    \"];\n}}",
                 scope,
                 scope,
                 expr.id,
                 color,
                 expr.op.get_meta().name,
                 expr.ancestors,
                 expr.id,
                 expr.shape,
        )
    }
}
