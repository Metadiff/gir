use primitives::*;
use props::*;
use ops::*;
use errors::*;
use api::ids;

use std::boxed::Box;
use std::collections::{HashMap, VecDeque};
use std::cell::{Ref, RefMut, RefCell};
use slog::{Logger, DrainExt};
use slog_term;
use std::rc::Rc;
use std::convert::AsRef;


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
    pub scope: Vec<String>,
    pub sym_int: Option<SymInt>
}


#[derive(Debug, Clone)]
pub struct Expr {
    pub wrapper: GraphWrapper,
    pub id: usize
}

impl AsRef<Expr> for Expr{
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Expr {
    pub fn get(&self) -> Result<Ref<ExprData>> {
        self.wrapper.get_node(self.id)
    }

    pub fn get_mut(&self) -> Result<RefMut<ExprData>> {
        self.wrapper.get_node_mut(self.id)
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<ExprData>,
    pub order: Vec<usize>,
    pub props: GraphProperties,
    pub grad_level: usize,
    pub scope: Vec<String>,
    pub op_map: HashMap<String, Vec<usize>>,
    pub updates: HashMap<usize, usize>,
    pub log: Logger,
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new(Logger::root(slog_term::streamer().full().build().fuse(), o!()))
    }
}

impl Graph {
    pub fn new(log: Logger) -> Self {
        Graph {
            nodes: Vec::new(),
            order: Vec::new(),
            props: GraphProperties::default(),
            grad_level: 0,
            scope: Vec::new(),
            op_map: HashMap::new(),
            updates: HashMap::new(),
            log: log
        }
    }

    pub fn scope_str(&self) -> String {
        let ref sep = self.props.scope_delimiter;
        self.scope.join(&sep)
    }

    pub fn name_in_scope(&self, name: &str) -> String {
        let ref sep = self.props.scope_delimiter;
        let joined = self.scope.join(&sep);
        format!("{}{}{}", joined, sep, name)
    }

    pub fn get_node(&self, index: usize) -> Result<&ExprData> {
        self.nodes.get(index).ok_or(ErrorKind::InvalidExprAccess(index).into())
    }

    pub fn get_node_mut(&mut self, index: usize) -> Result<&mut ExprData> {
        self.nodes.get_mut(index).ok_or(ErrorKind::InvalidExprAccess(index).into())
    }

    pub fn apply_op(&mut self, op: Box<Operator>, args: Vec<usize>) -> Result<usize> {
        let data = op.apply(self, args)?;
        Ok(self.add_node(data)?)
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
                                return Err(ErrorKind::Msg(
                                    format!("The parameter '{}' already exists \
                                    in the graph.", name)).into())
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

    pub fn get_flow(&self, roots: &Vec<usize>, leafs: &Vec<usize>) -> Vec<bool> {
        let descendants = self.get_descendants(roots);
        let ancestors = self.get_ancestors(leafs);
        descendants.into_iter()
            .zip(ancestors.into_iter())
            .map(|(r, l)| r && l).collect()
    }

    pub fn scalar(&mut self, value: f64, data_type: FundamentalType) -> usize {
        let op = Box::new(Scalar{
            value: value,
            data_type: data_type,
        });
        // This can not fail
        self.add_node(op.apply_null()).unwrap()
    }

    pub fn input(&mut self, data_type: FundamentalType, shape: Shape, name: Option<String>) -> usize {
        let op = Box::new(Input{
            data_type: data_type,
            shape: shape
        });
        let mut new_expr = op.apply_null();
        new_expr.name = self.name_in_scope(&name.unwrap_or("_anonymous_".into()));
        self.add_node(new_expr).unwrap()
    }

    pub fn parameter(&mut self,  data_type: FundamentalType, shape: Shape, name: String) -> Result<usize> {
        let op = Box::new(Parameter{
            param_name: self.name_in_scope(&name),
            data_type: data_type,
            shape: shape
        });
        self.add_node(op.apply_null())
    }

    pub fn copy_into(&self, graph: &mut Graph,
                     mask: &[bool],
                     provided: Option<HashMap<usize, usize>>,
                     discard_updates: bool)
                     -> Result<HashMap<usize, usize>> {
        let mut provided = provided.unwrap_or(HashMap::new());
        let init_scope = graph.scope.clone();
        for &id in self.order.iter().filter(|&&x| mask[x]) {
            if provided.get(&id).is_none() {
                let node: &ExprData = self.nodes.get(id).unwrap();
                let op = node.op.clone();
                graph.scope = self.nodes[id].scope.clone();
                let new_id = match op.get_meta().name {
                    "Input" | "Parameter" | "Scalar" => {
                        graph.add_node(op.apply_null())?
                    },
                    _ => {
                        let mut ancestors: Vec<usize> = Vec::with_capacity(node.ancestors.len());
                        for &a in &node.ancestors {
                            let &v = provided.get(&a).ok_or(ErrorKind::Msg(
                                format!("Unexpected error in copy_into for node {}.", a)))?;
                            ancestors.push(v);
                        }
                        let data = op.apply(graph, ancestors)?;
                        graph.add_node(data)?
                    }
                };
                provided.insert(id, new_id);
            }
        }
        graph.scope = init_scope;
        if ! discard_updates {
            for (a, u) in self.updates.iter() {
                let ap = provided.get(a).ok_or(ErrorKind::Msg(
                    format!("The argument {} needed for updates is not provided.", a)))?;
                let up = provided.get(a).ok_or(ErrorKind::Msg(
                    format!("The argument {} needed for updates is not provided.", u)))?;
                ids::update(graph, *ap, *up)?;
            }
        }
        Ok(provided)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphWrapper {
    pub graph: Rc<RefCell<Graph>>
}

pub type MutGraph<'a> = RefMut<'a, Graph>;

impl GraphWrapper {
    pub fn new(log: Logger) -> Self {
        GraphWrapper {
            graph: Rc::new(RefCell::new(Graph::new(log)))
        }
    }

    pub fn get(&self) -> Ref<Graph> {
        Ref::map(self.graph.borrow(), |x| x)
    }

    pub fn get_mut(&self) -> RefMut<Graph> {
        RefMut::map(self.graph.borrow_mut(), |d| d)
    }

    pub fn get_node(&self, id: usize) -> Result<Ref<ExprData>> {
        if self.graph.borrow().nodes.get(id).is_some() {
            Ok(Ref::map(self.graph.borrow(), |d| d.nodes.get(id).unwrap()))
        } else {
            Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn get_node_mut(&self, id: usize) -> Result<RefMut<ExprData>> {
        if self.graph.borrow().nodes.get(id).is_some() {
            Ok(RefMut::map(self.graph.borrow_mut(), |d| d.nodes.get_mut(id).unwrap()))
        } else {
            Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn as_expr(&self, id: usize) -> Result<Expr> {
        match self.get().nodes.get(id) {
            Some(_) => Ok(Expr{wrapper: self.clone(), id:id }),
            None => Err(ErrorKind::InvalidExprAccess(id).into())
        }
    }

    pub fn as_exprs(&self, ids: &[usize]) -> Result<Vec<Expr>> {
        let mut exprs = Vec::new();
        for &id in ids {
            match self.get().nodes.get(id) {
                Some(_) => {exprs.push(Expr{wrapper: self.clone(), id:id });},
                None => return Err(ErrorKind::InvalidExprAccess(id).into())
            }
        }
        Ok(exprs)
    }

    pub fn scalar(&self, value: f64, data_type: FundamentalType) -> Expr {
        // This can not fail
        let x = self.get_mut().scalar(value, data_type);
        self.as_expr(x).unwrap()
    }

    pub fn input(&self, data_type: FundamentalType, shape: Shape, name: Option<String>) -> Expr {
        // This can not fail
        let x = self.get_mut().input(data_type, shape, name);
        self.as_expr(x).unwrap()
    }

    pub fn parameter(&self, data_type: FundamentalType, shape: Shape, name: String) -> Result<Expr> {
        let x = self.get_mut().parameter(data_type, shape, name)?;
        self.as_expr(x)
    }
}