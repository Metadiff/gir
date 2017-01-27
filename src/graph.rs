use primitives::*;
use props::*;
use ops::*;
use errors::*;

use std::boxed::Box;
use std::collections::{HashMap, VecDeque};
use std::cell::{Ref, RefMut, RefCell};
use slog::{Logger, DrainExt};
use slog_term;
use std::rc::Rc;


#[derive(Debug, Clone)]
pub struct ExprData{
    pub id: usize,
    pub name: String,
    pub ancestors: Vec<usize>,
    pub children: Vec<usize>,
    pub op: Box<Operator>,
    pub data_type: FundamentalType,
    pub shape: Shape,
    pub is_input_dependent: bool,
    pub is_differentiable: bool,
    pub matrix_positivity:  MatrixPositivity,
    pub matrix_symmetry: MatrixSymmetry,
    pub matrix_fill: MatrixFill,
    pub grad_level: usize,
    pub scope: String,
    pub sym_int: Option<SymInt>
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub graph: Graph,
    pub id: usize
}

impl Expr {
    pub fn get(&self) -> Result<Ref<ExprData>> {
        self.graph.get_node(self.id)
        //        if self.graph.get().nodes.get(self.id).is_none() {
        //            Err(ErrorKind::InvalidExprAccess(self.id).into())
        //        }  else {
        //            Ok(Ref::map(self.graph.get(), |d|
        //                d.nodes.get(self.id).unwrap()))
        //        }
    }

    pub fn get_mut(&self) -> Result<RefMut<ExprData>> {
        self.graph.get_node_mut(self.id)
        //        if self.graph.rc.borrow().nodes.get(self.id).is_none() {
        //            Err(ErrorKind::InvalidExprAccess(self.id).into())
        //        }  else {
        //            Ok(RefMut::map(self.graph.get_mut(), |d|
        //                d.nodes.get_mut(self.id).unwrap()))
        //        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphData {
    pub nodes: Vec<ExprData>,
    pub order: Vec<usize>,
    pub props: GraphProperties,
    pub grad_level: usize,
    pub updates: Vec<(usize, usize)>,
    pub scope: String,
    pub scope_map: HashMap<String, Vec<usize>>,
    pub op_map: HashMap<String, Vec<usize>>
}

impl GraphData {
    pub fn get_descendants(&self, roots: &Vec<usize>) -> Vec<bool> {
        let mut mask : Vec<bool> = vec![false; self.nodes.len()];

        let mut fifo = VecDeque::with_capacity(roots.len());
        for &r in roots {
            mask[r] = true;
            fifo.push_back(r);
        }

        while !fifo.is_empty() {
            let id = fifo.pop_front().unwrap();
            for &child in self.nodes[id].children.iter() {
                if !mask[child] {
                    mask[child] = true;
                    fifo.push_back(child);
                }
            }
        }

        return mask;
    }

    pub fn get_ancestors(&self, leafs: &Vec<usize>) -> Vec<bool> {
        let mut mask : Vec<bool> = vec![false; self.nodes.len()];

        let mut fifo = VecDeque::with_capacity(leafs.len());
        for &l in leafs {
            mask[l] = true;
            fifo.push_back(l);
        }

        while !fifo.is_empty() {
            let id = fifo.pop_front().unwrap();
            for &ancestor in self.nodes[id].ancestors.iter() {
                if !mask[ancestor] {
                    mask[ancestor] = true;
                    fifo.push_back(ancestor);
                }
            }
        }

        return mask;
    }

    pub fn add_node(&mut self, mut data: ExprData) -> Result<usize> {
        if let Some(id) = self.equivalent_node(&data)? {
            Ok(id)
        } else {
            data.id = self.nodes.len();
            data.scope = self.scope.clone();
            //            println!("Adding node {:?}", data);
            for &a in &data.ancestors {
                self.nodes[a].children.push(data.id)
            }
            // Insert into op_map
            if !self.op_map.contains_key(data.op.get_meta().name) {
                self.op_map.insert(String::new() + data.op.get_meta().name, vec![data.id]);
            } else {
                self.op_map.get_mut(data.op.get_meta().name).unwrap().push(data.id);
            }
            // Insert into scope_map
            if !self.op_map.contains_key(&self.scope) {
                self.op_map.insert(self.scope.clone(), vec![data.id]);
            } else {
                self.op_map.get_mut(&self.scope).unwrap().push(data.id);
            }
            // Insert into order
            self.order.push(data.id);
            // Insert into nodes
            self.nodes.push(data);
            Ok(self.nodes.len() - 1)
        }
    }

    pub fn equivalent_node(&self, data: &ExprData) -> Result<Option<usize>> {
        if data.ancestors.len() == 0 {
            let meta = data.op.get_meta();
            // Only input operators have no parents
            match meta.name {
                "Parameter" => {
                    // Check if parameter already exists
                    if let Some(v) = self.op_map.get("Parameter") {
                        let (_, _, name) = *data.op.get_args().unwrap()
                            .downcast::<(FundamentalType, Shape, String)>().unwrap();
                        for &id in v {
                            let (_, _, v_name) = *self.nodes[id].op.get_args().unwrap()
                                .downcast::<(FundamentalType, Shape, String)>().unwrap();
                            if name == v_name {
                                return Err(ErrorKind::ParameterAlreadyExists(name).into())
                            }
                        }
                    }
                },
                "Scalar" => {
                    if let Some(v) = self.op_map.get("Scalar") {
                        let (value, data_type) = *data.op.get_args().unwrap()
                            .downcast::<(f64, FundamentalType)>().unwrap();
                        for &id in v {
                            let (v_value, v_data_type) = *self.nodes[id].op.get_args().unwrap()
                                .downcast::<(f64, FundamentalType)>().unwrap();
                            if value == v_value && data_type == v_data_type {
                                return Ok(Some(id));
                            }
                        }
                    }
                },
                "SymIntInput" => {
                    if let Some(v) = self.op_map.get("SymIntInput") {
                        let sym_int_id = data.op.get_args().unwrap()
                            .downcast::<String>().unwrap();
                        for &id in v {
                            let v_id = self.nodes[id].op.get_args().unwrap()
                                .downcast::<String>().unwrap();
                            if sym_int_id == v_id {
                                return Ok(Some(id));
                            }
                        }
                    }
                },
                _ => {}
            }
            return Ok(None)
        }
        // For reductions if they have not changed the shape its a pointless exercise
        if data.op.get_meta().reduction && data.shape == self.nodes[data.ancestors[0]].shape {
            return Ok(Some(data.ancestors[0]))
        }
        // Candidates are all children of the ancestor
        for &a in &data.ancestors {
            for &c in &self.nodes.get(a).unwrap().children {
                let node = self.nodes.get(c).unwrap();
                if data.op.get_meta() == node.op.get_meta() {
                    let ordered_parents = data.op.get_meta().ordered_parents;
                    if ordered_parents && data.ancestors == node.ancestors {
                        return Ok(Some(c));
                    } else if !ordered_parents && data.ancestors.len() == node.ancestors.len() {
                        let mut v1_sorted = data.ancestors.clone();
                        v1_sorted.sort();
                        let mut v2_sorted = node.ancestors.clone();
                        v2_sorted.sort();
                        if v1_sorted == v2_sorted {
                            return Ok(Some(c));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn get_flow(&self, roots: &Vec<usize>, leafs: &Vec<usize>) -> Vec<bool> {
        let descendants = self.get_descendants(roots);
        let ancestors = self.get_ancestors(leafs);
        descendants.into_iter()
            .zip(ancestors.into_iter())
            .map(|(r, l)| r && l).collect()
    }

    pub fn constant_scalar(&mut self, value: f64, data_type: FundamentalType) -> Result<usize> {
        let op = Box::new(Scalar{
            value: value,
            data_type: data_type,
        });

        // Maybe make these wi
        let mut new_expr = op.apply_null();
        new_expr.name = "Scalar".into();
        self.add_node(new_expr)
    }

    pub fn input(&mut self, data_type: FundamentalType, shape: Shape, name: String) -> Result<usize> {
        let op = Box::new(Input{
            data_type: data_type,
            shape: shape
        });
        let mut new_expr = op.apply_null();
        new_expr.name = self.scope.clone() + &self.props.scope_delimiter + &name;
        self.add_node(new_expr)
    }

    pub fn parameter(&mut self,  data_type: FundamentalType, shape: Shape, name: String) -> Result<usize> {
        let op = Box::new(Parameter{
            param_name: self.scope.clone() + &self.props.scope_delimiter + &name,
            data_type: data_type,
            shape: shape
        });
        let mut new_expr = op.apply_null();
        new_expr.name = self.scope.clone() + &self.props.scope_delimiter + &name;
        self.add_node(new_expr)
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub rc: Rc<RefCell<GraphData>>,
    pub log: Rc<::slog::Logger>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new(Logger::root(slog_term::streamer().full().build().fuse(), o!()))
    }
}

impl Graph {
    pub fn new(logger: Logger) -> Self {
        Graph {
            rc: Rc::new(RefCell::new(GraphData::default())),
            log: Rc::new(logger),
        }
    }

    pub fn get(&self) -> Ref<GraphData> {
        self.rc.borrow()
    }

    pub fn get_mut(&self) -> RefMut<GraphData> {
        self.rc.borrow_mut()
    }

    pub fn get_node(&self, id: usize) -> Result<Ref<ExprData>> {
        if self.rc.borrow().nodes.get(id).is_some() {
            Ok(Ref::map(self.rc.borrow(), |d| d.nodes.get(id).unwrap()))
        } else {
            Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn get_node_mut(&self, id: usize) -> Result<RefMut<ExprData>> {
        if self.rc.borrow().nodes.get(id).is_some() {
            Ok(RefMut::map(self.rc.borrow_mut(), |d| d.nodes.get_mut(id).unwrap()))
        } else {
            Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn to_expr(&self, id: usize) -> Result<Expr> {
        match self.rc.borrow().nodes.get(id) {
            Some(_) => Ok(Expr{graph: self.clone(), id:id }),
            None => Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn to_exprs(&self, ids: &Vec<usize>) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();
        for &id in ids {
            match self.rc.borrow().nodes.get(id) {
                Some(_) => {exprs.push(Expr{graph: self.clone(), id:id });},
                None => return Err(ErrorKind::InvalidExprAccess(id).into())
            }
        }
        Ok(exprs)
    }

    pub fn apply_op(&self, op: Box<Operator>, exprs: Vec<usize>) -> Result<usize> {
        let data = op.apply(self, exprs)?;
        Ok(self.get_mut().add_node(data)?)
    }

    pub fn apply_op_expr(&self, op: Box<Operator>, exprs: &Vec<Expr>) -> Result<Expr> {
        Ok(Expr{
            graph: self.clone(),
            id: self.apply_op(op, exprs.iter().map(|x| x.id).collect())?
        })
    }

    pub fn constant_scalar(&self, value: f64, data_type: FundamentalType) -> Expr {
        // This can not fail
        Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().constant_scalar(value, data_type).unwrap()
        }
    }

    pub fn input(&self, data_type: FundamentalType, shape: Shape, name: Option<String>) -> Expr {
        // This can not fail
        Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().input(data_type, shape, name.unwrap_or("Input".into())).unwrap()
        }
    }

    pub fn parameter(&self, data_type: FundamentalType, shape: Shape, name: String) -> Result<Expr> {
        Ok(Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().parameter(data_type, shape, name)?
        })
    }

    pub fn b_scalar(&self, name: Option<String>) -> Expr {
        self.input(FundamentalType::Boolean, Shape::scalar_shape(), name)
    }

    pub fn u_scalar(&self, name: Option<String>) -> Expr {
        self.input(FundamentalType::UnsignedInt, Shape::scalar_shape(), name)
    }

    pub fn i_scalar(&self, name: Option<String>) -> Expr {
        self.input(FundamentalType::SignedInt, Shape::scalar_shape(), name)
    }

    pub fn f_scalar(&self, name: Option<String>) -> Expr {
        self.input(FundamentalType::Float, Shape::scalar_shape(), name)
    }

    pub fn b_vector(&self, dim0: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Boolean, Shape::vector_shape(dim0), name)
    }

    pub fn u_vector(&self, dim0: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::UnsignedInt, Shape::vector_shape(dim0), name)
    }

    pub fn i_vector(&self, dim0: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::SignedInt, Shape::vector_shape(dim0), name)
    }

    pub fn f_vector(&self, dim0: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Float, Shape::vector_shape(dim0), name)
    }

    pub fn b_matrix(&self, dim0: Dim, dim1: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Boolean, Shape::matrix_shape(dim0, dim1), name)
    }

    pub fn u_matrix(&self, dim0: Dim, dim1: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::UnsignedInt, Shape::matrix_shape(dim0, dim1), name)
    }

    pub fn i_matrix(&self, dim0: Dim, dim1: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::SignedInt, Shape::matrix_shape(dim0, dim1), name)
    }

    pub fn f_matrix(&self, dim0: Dim, dim1: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Float, Shape::matrix_shape(dim0, dim1), name)
    }

    pub fn b_tensor3(&self, dim0: Dim, dim1: Dim, dim2: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Boolean, Shape::tensor3_shape(dim0, dim1, dim2), name)
    }

    pub fn u_tensor3(&self, dim0: Dim, dim1: Dim, dim2: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::UnsignedInt, Shape::tensor3_shape(dim0, dim1, dim2), name)
    }

    pub fn i_tensor3(&self, dim0: Dim, dim1: Dim, dim2: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::SignedInt, Shape::tensor3_shape(dim0, dim1, dim2), name)
    }

    pub fn f_tensor3(&self, dim0: Dim, dim1: Dim, dim2: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Float, Shape::tensor3_shape(dim0, dim1, dim2), name)
    }

    pub fn b_tensor4(&self, dim0: Dim, dim1: Dim, dim2: Dim, dim3: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Boolean, Shape::tensor4_shape(dim0, dim1, dim2, dim3), name)
    }

    pub fn u_tensor4(&self, dim0: Dim, dim1: Dim, dim2: Dim, dim3: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::UnsignedInt, Shape::tensor4_shape(dim0, dim1, dim2, dim3), name)
    }

    pub fn i_tensor4(&self, dim0: Dim, dim1: Dim, dim2: Dim, dim3: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::SignedInt, Shape::tensor4_shape(dim0, dim1, dim2, dim3), name)
    }

    pub fn f_tensor4(&self, dim0: Dim, dim1: Dim, dim2: Dim, dim3: Dim, name: Option<String>) -> Expr {
        self.input(FundamentalType::Float, Shape::tensor4_shape(dim0, dim1, dim2, dim3), name)
    }
}