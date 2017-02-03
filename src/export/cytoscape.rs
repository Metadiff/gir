use graph::*;
use std::io::Write as W;
use tera::Tera;

pub fn to_cytoscape_html(io: &mut W, g: &Graph) -> ::std::io::Result<()>  {
    let mut t = compile_templates!("templates/*");
    t.autoescape_on(vec!["html", ".sql"]);
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    let mut nodes: Vec<String> = Vec::new();
    let ref sep = g.props.scope_delimiter;
    for ref expr in &g.nodes {
        let mut s = String::new();
        expr_to_cytoscape(&mut s, &expr, sep).unwrap();
        nodes.push(s);
        for (i, &anc) in expr.ancestors.iter().enumerate() {
            edges.push((anc, expr.id, i));
        }
    }
    let mut context = ::tera::Context::new();
    context.add("nodes", &nodes);
    context.add("edges", &edges);

    match t.render("cytoscape.html.tera", context) {
        Ok(s) => {
            writeln!(io, "{}", s)?;
        }
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
    Ok(())
}

use std::fmt::Write;
pub fn expr_to_cytoscape(io: &mut String, expr: &ExprData, sep: &str) -> ::std::fmt::Result  {
    write!(io, "id: 'n{}',\n\t\t\t\t\t\t\
    label: '{}[{}]',\n\t\t\t\t\t\t\
    parent: '{}',\n\t\t\t\t\t\t\
    shape: 'ellipse',\n\t\t\t\t\t\t\
    expanded: 'false',\n\t\t\t\t\t\t\
    group: 'false',\n\t\t\t\t\t\t\
    Name: '{}[{}]',\n\t\t\t\t\t\t\
    Data: '{}',\n\t\t\t\t\t\t\
    Shape: '{}',\n\t\t\t\t\t\t\
    Parents: '{:?}',\n\t\t\t\t\t\t\
    Children: '{:?}'",
           expr.id,
           expr.op.get_meta().name, expr.id,
           expr.scope.join(sep),
           expr.name, expr.id,
           expr.data_type,
           expr.shape,
           expr.ancestors,
           expr.children
    )
}