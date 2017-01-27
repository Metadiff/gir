use primitives::*;
//use props::*;
use errors::*;
use graph::*;
use ops::interface::default::*;
use api;

use std::collections::HashMap;
use std::borrow::Borrow;

/// Calculates the gradient of the expression **f** with respect to all of the
/// expressions in **x** using reverse mode automatic differentiation.
pub fn gradient<T1: Borrow<Expr>, T2: Borrow<Expr>>(f: T1, x: &Vec<T2>) -> Result<Vec<Expr>> {
    let f = f.borrow();
    let ref graph = f.graph;
    if f.get()?.shape.order() != 0 {
        let err = ErrorKind::Msg("Requested gradient of a non scalar function.".into()).into();
        error!(graph.log, format!("[derivative] {}", err));
        Err(err)
    } else {
        // Verify all nodes are from the same graph
        for &ref expr in x {
            let e: &Expr = expr.borrow();
            if !::std::ptr::eq(&*graph.rc, &*e.graph.rc) {
                return Err(ErrorKind::NotFromTheSameGraph.into())
            }
        }
        let x_id = x.iter().map(|e| e.borrow().id).collect();
        let data_type = f.get()?.data_type;
        let u = graph.constant_scalar(1.0, data_type).id;
        reverse_diff(graph, &vec![f.id], &x_id, &vec![u])
    }
}

pub fn reverse_diff_expr(f: &Vec<&Expr>, x: &Vec<&Expr>, u: &Vec<&Expr>) -> Result<Vec<Expr>> {
    let combined: Vec<&Expr> = f.iter().cloned()
        .chain(x.iter().cloned())
        .chain(u.iter().cloned()).collect();
    if combined.len() == 0 {
        Ok(Vec::new())
    } else {
        // Verify all nodes are from the same graph
        same_graph(&combined)?;
        let f = f.iter().map(|e| e.id).collect();
        let x = x.iter().map(|e| e.id).collect();
        let u = u.iter().map(|e| e.id).collect();
        reverse_diff(&combined[0].graph, &f, &x, &u)
    }
}

/// Performs reverse mode automatic differentiation.
///
/// Mathematically the procedure computes **u^T J<sub>f</sub>**, where **J<sub>f</sub>**
/// is the Jacobian of **f** with respect to **x**.
///
/// Note that if **u** does not depend on **x** then this is equivalent to the gradient of
/// **u^T f** with respect to **x**.
///
/// The above mathematical formulas are valid even when **x**, **f** and **u** are lists of
/// tensors, by flattening and stacking them in single vectors.
///
/// # Arguments
///
/// * `x` - a vector of symbolic expressions defining what the Jacobian will be with respect to.
/// * `f` - a vector of symbolic expressions defining all of the functions of which the Jacobian
/// will be
/// * `u` - a vector of symbolic expressions defining projection vector which is multiplied
/// with the columns of the Jacobian.
///
/// ## Constraints
///
/// All of the expressions in the three vectors must be part of the same graph. Additionally,
/// it is required that `f` and `u` have the same length and each individual entries of
/// the two vectors have the same shape or are broadcastable to each other.
pub fn reverse_diff(graph: &Graph, f: &Vec<usize>, x: &Vec<usize>, u: &Vec<usize>)
                    -> Result<Vec<Expr>> {
    if x.is_empty() {
        // If no parameters then no derivatives
        return Ok(Vec::new())
    }
    if f.len() == 0 {
        // At least one function needed
        let err = ErrorKind::Msg("No functions provided to reverse_diff.".into()).into();
        error!(graph.log, format!("[derivative] {}", err));
        Err(err)
    } else if f.len() != u.len() {
        // Same number of 'f' and 'u'
        let err = ErrorKind::InvalidNumberOfProjectionTensors(f.len(), u.len());
        error!(graph.log, format!("[derivative] {}", err));
        Err(err.into())
    } else {
        // Verify shapes of 'f[i]' and 'u[i]'
        for (&fi, &ui) in f.iter().zip(u.iter()) {
//        for (ef, eu) in f.iter().zip(u.iter()) {
            if graph.get_node(fi)?.shape != graph.get_node(ui)?.shape {
                let err = ErrorKind::InvalidShapes(
                    format!("reverse_diff"),
                    format!("{}", graph.get_node(fi)?.shape),
                    format!("{}", graph.get_node(ui)?.shape));
                error!(graph.log, format!("[derivative] {}", err));
                return Err(err.into())
            }
        }

        trace!(graph.log, "[derivative] Starting reverse_diff.");
        // Flow tree
        let flow_tree = graph.get().get_flow(x, f);
        // Derivative messages
        let mut derivatives: HashMap<usize, Vec<usize>> = HashMap::new();
        let init_grad_level = graph.get().grad_level;
        let mut grad_level = 0;
        let (mut min_index, mut max_index) = (0, graph.get().nodes.len());

        for (i, (&fi, &ui)) in f.iter().zip(u.iter()).enumerate() {
            debug!(graph.log, "[derivative] Initial derivative at index {} for {} is {}.", i, fi, ui);
            derivatives.insert(fi, vec![ui]);
            min_index = ::std::cmp::min(min_index, graph.get().order.iter()
                .position(|&x| x == fi).unwrap());
            max_index = ::std::cmp::max(max_index, graph.get().order.iter()
                .position(|&x| x == fi).unwrap());
            grad_level = ::std::cmp::max(grad_level, graph.get().nodes[fi].grad_level + 1);
        }
        for &xi in x {
            min_index = ::std::cmp::min(min_index, graph.get().order.iter()
                .position(|&x| x == xi).unwrap());
        }

        let init_scope = graph.get().scope.clone();
        graph.get_mut().grad_level = grad_level;
        graph.get_mut().scope = format!("rd{}", grad_level);

        // Send derivative message in reverse mode
        let traversal: Vec<usize> = graph.get().order[min_index..max_index]
            .iter().cloned().collect();
        for i in traversal.into_iter().rev().filter(|&x| flow_tree[x]) {
            let pd = derivatives.remove(&i).unwrap_or(Vec::new());
            for &x_i in x {
                if x_i == i {
                    derivatives.insert(i, pd.clone());
                    break;
                }
            }
            for (a, df_da) in reverse_diff_op(graph, i, pd, &flow_tree)? {
                derivatives.entry(a).or_insert(Vec::new()).push(df_da);
//                println!("{}: {:?}", a, derivatives.get(&a));
//                println!("************************");
//                println!("{}: {:?}", 3, derivatives.get(&3));
//                println!("{}: {:?}", 4, derivatives.get(&4));
//                println!("{}: {:?}", 3, derivatives.get(&3));
//                println!("{}: {:?}", 4, derivatives.get(&4));
//                println!("************************");
            }
        }

        // Reset the grad_level
        graph.get_mut().grad_level = init_grad_level;

        let id = graph.get().props.policies.independent_derivative;
        let mut ut_jf = Vec::with_capacity(x.len());
        for (i, &xi) in x.iter().enumerate()  {
            println!("{}: {:?}", xi, derivatives.get(&xi));
            let v = derivatives.remove(&xi).unwrap_or(Vec::new());
            match v.len() {
                0 => match id {
                    Policy::Quite => {
                        let data_type = graph.get_node(xi)?.data_type;
                        let di = graph.constant_scalar(0.0, data_type);
                        let old_name = di.get()?.name.clone();
                        di.get_mut()?.name = format!("{}|rd[{}]", old_name, xi);
                        ut_jf.push(di);
                    },
                    Policy::Warn => {
                        let err = ErrorKind::IndependentDerivative(i);
                        warn!(graph.log, format!("[derivative] {}", err));
                        let data_type = graph.get_node(xi)?.data_type;
                        let di = graph.constant_scalar(0.0, data_type);
                        let old_name = di.get()?.name.clone();
                        di.get_mut()?.name = format!("{}|rd[{}]", old_name, xi);
                        ut_jf.push(di);
                    },
                    Policy::Raise => {
                        let err = ErrorKind::IndependentDerivative(i);
                        error!(graph.log, format!("[derivative] {}", err));
                        return Err(err.into())
                    },
                },
                1 => {
                    let id = v[0];
                    let old_name = graph.get_node(id)?.name.clone();
                    graph.get_node_mut(id)?.name = format!("{}|rd[{}]", old_name, xi);
                    ut_jf.push(Expr{
                        graph: graph.clone(),
                        id: id
                    });
                },
                _ => {
                    let id = api::ids::add(graph, v)?;
                    let old_name = graph.get_node(id)?.name.clone();
                    graph.get_node_mut(id)?.name = format!("{}|rd[{}]", old_name, xi);
                    ut_jf.push(Expr{
                        graph: graph.clone(),
                        id: id
                    });
                }
            }
        };
        graph.get_mut().scope = init_scope;
        trace!(graph.log, "[derivative] Fished reverse_diff.");
        Ok(ut_jf)
    }
}

/// Returns the a vector of pairs of parent ids and their corresponding derivatives
/// arising from their dependence on **x**.
///
/// Mathematically this computes **df/dx * dx/da**, where **a** is an immediate
/// ancestor of **x**.
///
/// If the `flow_tree` is `false` for any of the ancestors, this implies that no
/// derivative should be taken with respect to that ancestor.
///
/// # Arguments
///
/// * `x` - a symbolic expression data of the "current" tensor variable
/// * `dx` - a vector of symbolic expressions defining all derivatives of **f** coming
/// from the children of **x**
/// * `flow_tree` - a boolean mask specifying which nodes will be needed
pub fn reverse_diff_op<'a>(graph: &Graph, x: usize, dx: Vec<usize>, flow_tree: &Vec<bool>)
                           -> Result<Vec<(usize, usize)>> {
    // The derivative of the node is 0 so no messages to parents
    if dx.len() == 0 {
        return Ok(Vec::new())
    }

    // This is needed because operators like Add and Mul can have any number of parents
    let diff_parents = {
        let ref n = graph.get().nodes[x];
        match n.op.get_meta().differential_parents {
            ::std::usize::MAX => n.ancestors.len(),
            v => v
        }
    };

    if diff_parents == 0 {
        return Ok(Vec::new())
    }

    let init_scope = graph.get().scope.clone();
    let new_scope = if graph.get().nodes[x].scope == "" {
        format!("rd{}", graph.get().grad_level)
    } else {
        format!("rd{}{}{}",
                graph.get().grad_level,
                graph.get().props.scope_delimiter,
                graph.get().nodes[x].scope)
    };
    graph.get_mut().scope = new_scope;

    // If more than one derivative incoming the total derivative is the sum
    let dx = match dx.len() {
        1 => dx[0],
        _ => api::ids::add(graph, dx)?
    };
    debug!(graph.log, "[derivative] Derivative of {} is {}.", x, dx);

    let old_name = graph.get().nodes[dx].name.clone();
    graph.get_mut().nodes[dx].name = format!("{}|rd[{}]", old_name, x);

    let op = graph.get().nodes[x].op.clone();

    let parent_derivatives = op.reverse_diff(graph, x, dx, flow_tree)?;
    for &(ref p, ref pd) in &parent_derivatives {
        let old_name = graph.get().nodes[*pd].name.clone();
        graph.get_mut().nodes[*pd].name = format!("{}|rd[{}->{}]", old_name, x, p);
        debug!(graph.log, "[derivative] Sending rd {} from {} to {}.", pd, x, p);
    }
    graph.get_mut().scope = init_scope;

    Ok(parent_derivatives)
}