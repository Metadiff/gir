use graph::*;
use std::io::Write;

pub fn to_dot(io: &mut Write, g: &Graph) -> ::std::io::Result<()>  {
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    writeln!(io, "digraph g {{")?;
    for expr in &g.nodes {
        expr_to_dot(io, expr, &g.props.scope_delimiter)?;
        for (i, &anc) in expr.ancestors.iter().enumerate() {
            edges.push((anc, expr.id, i));
        }
    }
    for e in edges {
        writeln!(io, "\tN{} -> N{} [label=\"{}\"];", e.0, e.1, e.2)?;
    }
    writeln!(io, "}}")
}

pub fn expr_to_dot(io: &mut Write, expr: &ExprData, sep: &str) -> ::std::io::Result<()>  {
    let scope = if expr.scope.len() == 0 {
        "0".into()
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
    \tN{} [shape=rectangle, style=filled, fillcolor={},\n\
    label=\"\
    {}{:?}\\n\
    id:{}\\n\
    shape:{}\\n\
    \"];\n}}",
             scope,
             expr.id,
             color,
             expr.op.get_meta().name,
             expr.ancestors,
             expr.id,
             expr.shape,
    )
//    writeln!(io, "\tN{} [label=\"\
//    {}{:?}\\n\
//    id:{}\\n\
//    name:{}\\n\
//    data_type:{}\\n\
//    shape:{}\\n\
//    scope:{}\\n\
//    children:{:?}\\n\
//    \"];",
//             expr.id,
//             expr.op.get_meta().name,
//             expr.ancestors,
//             expr.id,
//             expr.name,
//             expr.data_type,
//             expr.shape,
//             expr.scope,
//             expr.children
//    )
}