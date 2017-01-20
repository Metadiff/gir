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
    pub op_map: HashMap<String, Vec<usize>>,
}

impl GraphData {
    pub fn get_descendants(&self, roots: &Vec<Expr>) -> Vec<bool> {
        let mut mask : Vec<bool> = vec![false; self.nodes.len()];

        let mut fifo = VecDeque::with_capacity(roots.len());
        for r in roots {
            mask[r.id] = true;
            fifo.push_back(r.id);
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

    pub fn get_ancestors(&self, leafs: &Vec<Expr>) -> Vec<bool> {
        let mut mask : Vec<bool> = vec![false; self.nodes.len()];

        let mut fifo = VecDeque::with_capacity(leafs.len());
        for l in leafs {
            mask[l.id] = true;
            fifo.push_back(l.id);
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

    pub fn add_node(&mut self, mut data: ExprData) -> usize {
        data.id = self.nodes.len();
        data.name = "Derived".into();
        data.scope = self.scope.clone();
        for &a in &data.ancestors {
            self.nodes[a].children.push(data.id)
        }
        self.nodes.push(data);
        self.order.push(self.nodes.len() - 1);
        self.nodes.len() - 1
    }

    pub fn get_flow(&self, roots: &Vec<Expr>, leafs: &Vec<Expr>) -> Vec<bool> {
        let descendants = self.get_descendants(roots);
        let ancestors = self.get_ancestors(leafs);
        descendants.into_iter()
            .zip(ancestors.into_iter())
            .map(|(r, l)| r && l).collect()
    }

    pub fn constant_scalar(&mut self, value: f64, data_type: FundamentalType) -> usize {
        let op = Box::new(ConstantScalar{
            value: value,
            data_type: data_type,
        });

        // Maybe make these wi
        let mut new_expr = op.apply_null();
        let id = self.nodes.len();
        new_expr.id = id;
        new_expr.name = "Constant".into();
        new_expr.scope = self.scope.clone();
        self.nodes.push(new_expr);
        self.order.push(id);
        id
    }

    pub fn input(&mut self, data_type: FundamentalType, shape: Shape, name: String) -> usize {
        let op = Box::new(Input{
            data_type: data_type,
            shape: shape
        });
        let mut new_expr = op.apply_null();
        let id = self.nodes.len();
        new_expr.id = id;
        new_expr.name = self.scope.clone() + &self.props.scope_delimiter + &name;
        new_expr.scope = self.scope.clone();
        self.nodes.push(new_expr);
        self.order.push(id);
        id
    }

    pub fn parameter(&mut self,  data_type: FundamentalType, shape: Shape, name: String) -> usize {
        let op = Box::new(Parameter{
            param_name: self.scope.clone() + &self.props.scope_delimiter + &name,
            data_type: data_type,
            shape: shape
        });
        let mut new_expr = op.apply_null();
        let id = self.nodes.len();
        new_expr.id = id;
        new_expr.name = self.scope.clone() + &self.props.scope_delimiter + &name;
        new_expr.scope = self.scope.clone();
        self.nodes.push(new_expr);
        self.order.push(id);
        id
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
            rc: Rc::new(RefCell::new(GraphData {
                nodes: Vec::new(),
                order: Vec::new(),
                props: GraphProperties::default(),
                grad_level: 0,
                updates: Vec::new(),
                scope: "".into(),
                scope_map: HashMap::new(),
                op_map: HashMap::new(),
            })),
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

    pub fn apply_op(&self, op: Box<Operator>, exprs: &Vec<usize>) -> Result<usize> {
        let data = op.apply(self, exprs)?;
        Ok(self.get_mut().add_node(data))
    }

    pub fn apply_op_expr(&self, op: Box<Operator>, exprs: &Vec<Expr>) -> Result<Expr> {
        Ok(Expr{
            graph: self.clone(),
            id: self.apply_op(op, &exprs.iter().map(|x| x.id).collect())?
        })
    }

    pub fn constant_scalar(&self, value: f64, data_type: FundamentalType) -> Expr {
        Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().constant_scalar(value, data_type)
        }
    }

    pub fn input(&self, data_type: FundamentalType, shape: Shape, name: Option<String>) -> Expr {
        Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().input(data_type, shape, name.unwrap_or("Input".into()))
        }
    }

    pub fn parameter(&self, data_type: FundamentalType, shape: Shape, name: String) -> Expr {
        Expr {
            graph: self.clone(),
            id: self.rc.borrow_mut().parameter(data_type, shape, name)
        }
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
}