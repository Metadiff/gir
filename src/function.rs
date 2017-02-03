//use primitives::*;
//use props::*;
//use ops::*;
use errors::*;
use graph::*;
use api::ids;
use std::collections::HashSet;
use std::ops::DerefMut;

pub struct GraphFunction {
    pub name: String,
    pub graph: Graph,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub unique_symints: HashSet<String>,
}

impl GraphFunction {
    pub fn new(graph: &Graph,
               inputs: &[usize],
               outputs: &[usize],
               discard_updates: bool,
               extra_updates: &[(usize, usize)],
               name: Option<String>) -> Result<Self> {
        // Verify all inputs and outputs and extra_updates are valid
        for &id in inputs.iter().chain(outputs.iter()) {
            graph.get_node(id)?;
        }
        for &(x, upd_x) in extra_updates {
            graph.get_node(x)?;
            graph.get_node(upd_x)?;
        }
        let mut leafs = vec![0; outputs.len()];
        leafs.clone_from_slice(outputs);
        // Add updates from the graph
        if ! discard_updates {
            for (&var, &upd) in &graph.updates {
                leafs.push(var);
                leafs.push(upd);
            }
        }
        // Add extra updates
        for &(var, upd) in extra_updates {
            leafs.push(var);
            leafs.push(upd);
        }
        // Get the ancestral tree of the outputs
        let ancestors = graph.get_ancestors(&leafs);
        // Verify all inputs needed are provided
        for &id in graph.order.iter().filter(|&&x| ancestors[x] &&
            graph.get_node(x).unwrap().op.get_meta().name == "Input") {
            if inputs.iter().position(|&x| x == id).is_none() {
                let name = graph.get_node(id).unwrap().name.clone();
                return Err(ErrorKind::Msg(format!("The with name {} and id {} is \
                required but has not been provided when creating the function.", name, id))
                    .into())
            }
        }
        // Copy the relevant part of the graph
        let mut sub_graph = Graph::new(graph.log.clone());
        let mapping = graph.copy_into(&mut sub_graph, &ancestors, None, discard_updates)?;
        // Add any extra updates
        for &(ref var, ref upd) in extra_updates {
            ids::update(&mut sub_graph, *mapping.get(var).unwrap(), *mapping.get(upd).unwrap())?;
        }
        // Fill up the unique symbolic integers
        let mut unique = HashSet::new();
        for ref node in &sub_graph.nodes {
            node.shape.0.unique_identifiers(&mut unique);
            node.shape.1.unique_identifiers(&mut unique);
            node.shape.2.unique_identifiers(&mut unique);
            node.shape.3.unique_identifiers(&mut unique);
        }
        // Return the function created
        Ok(GraphFunction{
            name: name.unwrap_or("main".into()),
            graph: sub_graph,
            inputs: inputs.iter().map(|x| *mapping.get(x).unwrap()).collect(),
            outputs: outputs.iter().map(|x| *mapping.get(x).unwrap()).collect(),
            unique_symints: unique,
        })
    }

    pub fn new_from_expr(inputs: &[Expr],
                         outputs: &[Expr],
                         discard_updates: bool,
                         extra_updates: &[(Expr, Expr)],
                         name: Option<String>) -> Result<Self> {
        let wrapper = if inputs.len() > 0 {
            inputs[0].wrapper.clone()
        } else if outputs.len() > 0 {
            outputs[0].wrapper.clone()
        } else if extra_updates.len() > 0 {
            extra_updates[0].0.wrapper.clone()
        } else {
            return Err(ErrorKind::Msg("No inputs, no outputs, no updates = no graph."
                .into()).into())
        };
        // Verify all expressions are from the same graph
        for ref expr in inputs.iter().chain(outputs.iter()) {
            if !::std::ptr::eq(&*wrapper.graph, &*expr.wrapper.graph) {
                return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
            }
        }
        // Verify all updates are from the same graph
        for &(ref expr, ref upd) in extra_updates {
            if !::std::ptr::eq(&*wrapper.graph, &*expr.wrapper.graph) {
                return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
            } else if !::std::ptr::eq(&*wrapper.graph, &*upd.wrapper.graph) {
                return Err(ErrorKind::Msg("Trying to use expressions which are \
                    not from the same graph.".into()).into())
            }
        }
        let inputs: Vec<usize> = inputs.iter().map(|e| e.id).collect();
        let outputs: Vec<usize> = outputs.iter().map(|e| e.id).collect();
        let extra_updates: Vec<(usize, usize)> = extra_updates.iter()
            .map(|&(ref e, ref u)| (e.id, u.id)).collect();
        let mut g = wrapper.get_mut();
        Self::new(g.deref_mut(),
                  inputs.as_slice(), outputs.as_slice(), discard_updates,
                  extra_updates.as_slice(), name)
    }
}