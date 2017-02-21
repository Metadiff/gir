use gir_core::primitives::*;
use gir_core::graph::*;
use gir_core::backend::*;
use gir_core::errors::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Neg;

use arrayfire as af;

#[derive(Clone)]
pub struct AfFunction {
    pub initialized: bool,
    pub precisions: BackendPrecisions,
    pub gf: GraphFunction,
    pub parameters: Rc<RefCell<HashMap<String, af::Array>>>,
    pub sym_input_shapes: Vec<Shape>,
    pub last_shapes: Vec<[usize; 4]>,
    pub last_deduced: HashMap<String, i64>,
    pub expr_map: HashMap<usize, af::Array>,
}

impl AfFunction {
    pub fn internal_eval(&mut self, inputs: &[&af::Array]) {
        for (&id, input) in self.gf.inputs.iter().zip(inputs) {
            self.expr_map.insert(id, (*input).clone());
        }
        for (name, &id) in self.gf.parameters.iter() {
            self.expr_map.insert(id, self.parameters.borrow().get(name).unwrap().clone());
//            let v = self.expr_map.get(&id).unwrap();
//            println!("Id: {}", id);
//            af_print!("Value:",v);
        }
        let order = self.gf.graph.order.clone();
        for &id in &order {
            self.compute_node(id);
        }
    }
}

impl CompiledFunction<af::Array, af::Array> for AfFunction {
    fn eval(&mut self, inputs: &[&af::Array]) -> Result<Vec<af::Array>> {
        // Check correct number of inputs are provided
        if inputs.len() != self.gf.inputs.len() {
            return Err(ErrorKind::Msg(format!("Incorrect number of inputs. \
            Expected: {}, actual: {}.", self.gf.inputs.len(), inputs.len())).into());
        }
        let input_shapes: Vec<[usize;4]> = inputs.iter().map(|x| {
            let mut dims = [1, 1, 1, 1];
            for (i, &d) in x.dims().get().iter().enumerate() {
                dims[i] = d as usize;
            }
            dims
        }).collect();
        // Check shapes are correct and if they have changed
        match verify_shapes(&input_shapes, &self.last_shapes, &self.sym_input_shapes)? {
            Some(deduced) => {
                self.last_shapes = input_shapes;
                self.last_deduced = deduced;
            },
            None => {}
        }
        self.internal_eval(inputs);
        let mut result = Vec::new();
        for i in &self.gf.outputs {
            result.push(self.expr_map.remove(i).unwrap());
        }
//        let output = self.gf.outputs.iter().map(|x| ).collect();
        Ok(result)
    }

    fn initialized(&self) -> bool {
        self.initialized
    }
    fn free_memory(&mut self) {}
}

impl AfFunction {
    fn compute_node(&mut self, id: usize) {
        let ref node = self.gf.graph.nodes[id];
        let expr_map = &mut self.expr_map;
        let op_meta = node.op.get_meta();
        match op_meta.name {
            "Input" | "Parameter" => {},
            "Scalar" => {
                let (value, _) = *node.op.get_args().unwrap()
                    .downcast::<(f64, FundamentalType)>().unwrap();
                let result = af::constant(value as f32, af::Dim4::new(&[1, 1, 1, 1]));
                expr_map.insert(node.id, result);
            },
            "Add" => {
                let result = match node.ancestors.len() {
                    2 => af::add(expr_map.get(&node.ancestors[0]).unwrap(),
                                 expr_map.get(&node.ancestors[1]).unwrap(), true),
                    _ => unimplemented!()
                };
                expr_map.insert(node.id, result);
            },
            "Mul" => {
                let result = match node.ancestors.len() {
                    2 => af::mul(expr_map.get(&node.ancestors[0]).unwrap(),
                                 expr_map.get(&node.ancestors[1]).unwrap(), true),
                    _ => unimplemented!()
                };
                expr_map.insert(node.id, result);
            },
            "MatMul" => {
                //            println!("{:?} vs {:?}", expr_map.get(&node.ancestors[0]).unwrap().get_type(),
                //                     expr_map.get(&node.ancestors[1]).unwrap().get_type());
                //            println!("{:?} vs {:?}", expr_map.get(&node.ancestors[0]).unwrap().dims(),
                //                     expr_map.get(&node.ancestors[1]).unwrap().dims());
                //            println!("{:?}", node.ancestors);
                let result = match node.ancestors.len() {
                    2 => af::matmul(expr_map.get(&node.ancestors[0]).unwrap(),
                                    expr_map.get(&node.ancestors[1]).unwrap(),
                                    af::MatProp::NONE, af::MatProp::NONE),
                    _ => unimplemented!()
                };
                expr_map.insert(node.id, result);
            },
            "Reorder" => {
                let order = *node.op.get_args().unwrap()
                    .downcast::<[Axis; 4]>().unwrap();
                let result = if order == [Axis::Axis1, Axis::Axis0, Axis::Axis2, Axis::Axis3] {
                    af::transpose(expr_map.get(&node.ancestors[0]).unwrap(), false)
                } else {
                    let dims = af::Dim4::new(&[order[0] as u64,
                        order[1] as u64,
                        order[2] as u64,
                        order[3] as u64]);
                    af::reorder(expr_map.get(&node.ancestors[0]).unwrap(), dims)
                };
                expr_map.insert(node.id, result);
            },
            "Tanh" => {
                let result = af::tanh(expr_map.get(&node.ancestors[0]).unwrap());
                expr_map.insert(node.id, result);
            },
            "Neg" => {
                let result = expr_map.get(&node.ancestors[0]).unwrap().clone().neg();
                expr_map.insert(node.id, result);
            },
            "Sum" => {
                let axis = *node.op.get_args().unwrap()
                    .downcast::<[bool; 4]>().unwrap();
                let mut result = None;
                {
                    let initial = expr_map.get(&node.ancestors[0]).unwrap();
                    for i in 0..4 {
                        if axis[i] {
                            if result.is_none() {
                                result = Some(af::sum(initial, i as i32));
                            } else {
                                result = Some(af::sum(&result.unwrap(), i as i32));
                            }
                        }
                    }
                }
                expr_map.insert(node.id, result.unwrap());
            },
            "TensorShape" => {
                let axis = *node.op.get_args().unwrap()
                    .downcast::<Axis>().unwrap();
                let result = {
                    let parent = expr_map.get(&node.ancestors[0]).unwrap();
                    af::constant(parent.dims()[axis as usize] as f32, af::Dim4::new(&[1, 1, 1, 1]))
                };
                expr_map.insert(node.id, result);
            },
            "Div" => {
                let result = {
                    let parent = expr_map.get(&node.ancestors[0]).unwrap();
                    let one = af::constant(1.0f32, af::Dim4::new(&[1, 1, 1, 1]));
                    af::div(&one, parent, true)
                };
                expr_map.insert(node.id, result);
            },
            "Broadcast" => {
                let result = expr_map.get(&node.ancestors[0]).unwrap().clone();
                expr_map.insert(node.id, result);
            },
            "Update" => {
                let name = self.gf.graph.nodes[node.ancestors[0]].name.clone();
//                {
//                    let x = self.parameters.borrow();
//                    let p_old = x.get(&name).unwrap();
//                    let p_new = expr_map.get(&node.ancestors[1]).unwrap();
//                    println!("[{}]{:?} vs [{}]{:?}",
//                             node.ancestors[0], p_old.dims(),
//                             node.ancestors[1], p_new.dims());
//                    if p_old.dims()[1] == 1 {
//                        af_print!("Value: ", p_old);
//                        af_print!("Value: ", p_new);
//                    }
//                }
                let upd = expr_map.get(&node.ancestors[1]).unwrap().clone();
                upd.eval();
                self.parameters.borrow_mut().insert(name, upd);
            },
            name => {
                panic!("Operator {} not implemented.", name)
            }
        }
//        println!("{} - {:?}", id, expr_map.get(&id).map(|x| x.dims()));
    }
}
