use graph::*;
use std::io::Write;

pub fn to_dot(io: &mut Write, graph: &Graph) -> ::std::io::Result<()>  {
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    let g = graph.get();
    writeln!(io, "digraph g {{")?;
    for expr in &g.nodes {
        expr_to_dot(io, expr)?;
        for (i, &anc) in expr.ancestors.iter().enumerate() {
            edges.push((anc, expr.id, i));
        }
    }
    for e in edges {
        writeln!(io, "\tN{} -> N{} [label=\"{}\"];", e.0, e.1, e.2)?;
    }
    writeln!(io, "}}")
}

pub fn expr_to_dot(io: &mut Write, expr: &ExprData) -> ::std::io::Result<()>  {
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
    writeln!(io, "\tN{} [label=\"\
    {}{:?}\\n\
    id:{},shape:{}\\n\
    \"];",
             expr.id,
             expr.op.get_meta().name,
             expr.ancestors,
             expr.id,
             expr.shape,
    )
}