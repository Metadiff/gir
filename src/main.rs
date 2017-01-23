extern crate graph_ir;
extern crate slog_term;


use graph_ir as g;

fn main() {
    let g = g::graph::Graph::default();
    let n0 = &g.f_scalar(None);
    let n1 = &g.f_scalar(None);
    let n2 = &g.f_scalar(None);
    let n3 = &g::api::add_2(n0, n1).unwrap();
    let n4 = &g::api::mul_2(n2, n3).unwrap();
    let n6 = &g::api::sub(n4, n0).unwrap();
//    println!("{}, {}, {}", n1.id, n2.id, n3.id);
//    println!("{:?}", n1.graph);
    let r = g::derivative::gradient(n6, &vec![n0.clone()]).unwrap();
    graph_ir::to_dot(&mut ::std::io::stdout(), &g);
    println!("{:?}", g.get().order);
//    println!("{:?}", r[0].get().unwrap());
//    println!("Hello, world! - {:?}", r);
}
