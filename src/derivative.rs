use primitives::*;
//use props::*;
use errors::*;
use graph::*;
use api;

use std::collections::HashMap;

pub fn gradient(f: &Expr, x: &Vec<Expr>) -> Result<Vec<Expr>> {
    let ref g = f.graph;
    if f.get()?.shape.order() != 0 {
        let err = ErrorKind::Msg("Requested gradient of a non scalar function.".into()).into();
        error!(g.log, format!("[derivative] {}", err));
        Err(err)
    } else {
        let data_type = f.get()?.data_type;
        let u = vec![g.constant_scalar(1.0, data_type)];
        reverse_diff(&vec![f.clone()], &x, &u)
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
pub fn reverse_diff(f: &Vec<Expr>, x: &Vec<Expr>, u: &Vec<Expr>)
                    -> Result<Vec<Expr>> {
    if x.is_empty() {
        // If no parameters then no derivatives
        return Ok(Vec::new())
    }
    let g = x[0].graph.clone();
    if f.len() == 0 {
        // At least one function needed
        let err = ErrorKind::Msg("No functions provided to reverse_diff.".into()).into();
        error!(g.log, format!("[derivative] {}", err));
        Err(err)
    } else if f.len() != u.len() {
        // Same number of 'f' and 'u'
        let err = ErrorKind::InvalidNumberOfProjectionTensors(f.len(), u.len());
        error!(g.log, format!("[derivative] {}", err));
        Err(err.into())
    } else {
        // Verify shapes of 'f[i]' and 'u[i]'
        for (ef, eu) in f.iter().zip(u.iter()) {
            if ef.get()?.shape != eu.get()?.shape {
                let err = ErrorKind::InvalidShapes(
                    format!("{}", ef.get()?.shape),
                    format!("{}", eu.get()?.shape));
                error!(g.log, format!("[derivative] {}", err));
                return Err(err.into())
            }
        }
        // Verify all nodes are from the same graph
        for expr in x.iter().chain(f.iter()).chain(u.iter()) {
            if !::std::ptr::eq(&*expr.graph.rc, &*g.rc) {
                let err = ErrorKind::NotFromTheSameGraph;
                error!(g.log, format!("[derivative] {}", err));
                return Err(err.into())
            }
        }

        trace!(g.log, "[derivative] Starting reverse_diff.");
        // Flow tree
        let flow_tree = g.rc.borrow().get_flow(x, f);
        // Derivative messages
        let mut derivatives: HashMap<usize, Vec<usize>> = HashMap::new();
        let init_grad_level = g.get().grad_level;
        let mut grad_level = init_grad_level;
        let (mut min_index, mut max_index) = (0, g.get().nodes.len());

        for (i, (fe, ue)) in f.iter().zip(u.iter()).enumerate() {
            debug!(g.log, "[derivative] Initial derivative at index {} for {} is {}.",
            i, fe.id, ue.id);
            derivatives.insert(fe.id, vec![ue.id]);
            min_index = ::std::cmp::min(min_index, g.get().order.iter()
                .position(|x| *x == fe.id).unwrap());
            max_index = ::std::cmp::max(max_index, g.get().order.iter()
                .position(|x| *x == fe.id).unwrap());
            grad_level = ::std::cmp::max(grad_level, g.get().nodes[fe.id].grad_level);
        }
        for xe in x {
            min_index = ::std::cmp::min(min_index, g.get().order.iter()
                .position(|x| *x == xe.id).unwrap());
        }

        // Send derivative message in reverse mode
        let traversal: Vec<usize> = g.get().order[min_index..max_index]
            .iter().cloned().collect();
        for i in traversal.into_iter().rev().filter(|&x| flow_tree[x]) {
            for (a, df_da) in reverse_diff_op(&g, i, derivatives.entry(i).or_insert(Vec::new()), &flow_tree)? {
                derivatives.entry(a).or_insert(Vec::new()).push(df_da);
            }
        }

        // Reset the grad_level
        g.get_mut().grad_level = init_grad_level;

        let id = g.get().props.policies.independent_derivative;
        let mut ut_jf = Vec::with_capacity(x.len());
        for (i, ref x_i) in x.iter().enumerate()  {
            let mut v = derivatives.remove(&x_i.id).unwrap_or(Vec::new());
            match v.len() {
                0 => match id {
                    Policy::Quite => {
                        let data_type = x_i.get()?.data_type;
                        let di = g.constant_scalar(0.0, data_type);
                        let old_name = di.get()?.name.clone();
                        di.get_mut()?.name = format!("{}|rd[{}]", old_name, x_i.id);
                        ut_jf.push(di);
                    },
                    Policy::Warn => {
                        let err = ErrorKind::IndependentDerivative(i);
                        warn!(g.log, format!("[derivative] {}", err));
                        let data_type = x_i.get()?.data_type;
                        let di = g.constant_scalar(0.0, data_type);
                        let old_name = di.get()?.name.clone();
                        di.get_mut()?.name = format!("{}|rd[{}]", old_name, x_i.id);
                        ut_jf.push(di);
                    },
                    Policy::Raise => {
                        let err = ErrorKind::IndependentDerivative(i);
                        error!(g.log, format!("[derivative] {}", err));
                        return Err(err.into())
                    },
                },
                1 => {
                    let id = v[0];
                    let old_name = g.get_node(id)?.name.clone();
                    g.get_node_mut(id)?.name = format!("{}|rd[{}]", old_name, x_i.id);
                    ut_jf.push(Expr{
                        graph: g.clone(),
                        id: id
                    });
                },
                _ => {
                    let id = api::ids::add(&g, &v)?;
                    let old_name = g.get_node(id)?.name.clone();
                    g.get_node_mut(id)?.name = format!("{}|rd[{}]", old_name, x_i.id);
                    ut_jf.push(Expr{
                        graph: g.clone(),
                        id: id
                    });
                }
            }
        };
        trace!(g.log, "[derivative] Fished reverse_diff.");
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
pub fn reverse_diff_op<'a>(g: &Graph, x: usize, dx: &Vec<usize>, flow_tree: &Vec<bool>)
                           -> Result<Vec<(usize, usize)>> {
    // The derivative of the node is 0 so no messages to parents
    if dx.len() == 0 {
        return Ok(Vec::new())
    }

    // This is needed because operators like Add and Mul can have any number of parents
    let diff_parents = {
        let ref n = g.get().nodes[x];
        match n.op.get_meta().differential_parents {
            ::std::usize::MAX => n.ancestors.len(),
            v => v
        }
    };

    if diff_parents == 0 {
        return Ok(Vec::new())
    }

    // If more than one derivative incoming the total derivative is the sum
    let dx = match dx.len() {
        1 => dx[0],
        _ => api::ids::add(g, dx)?
    };
    debug!(g.log, "[derivative] Derivative of {} is {}.", x, dx);

    let init_scope = g.get().scope.clone();
    let x_scope = g.get().nodes[x].scope.clone();
    g.get_mut().scope = x_scope;

    let old_name = g.get().nodes[dx].name.clone();
    g.get_mut().nodes[dx].name = format!("{}|rd[{}]", old_name, x);

    let op = g.get().nodes[x].op.clone();

    let parent_derivatives = op.reverse_diff(g, x, dx, flow_tree)?;
    for &(ref p, ref pd) in &parent_derivatives {
        let old_name = g.get().nodes[*pd].name.clone();
        g.get_mut().nodes[*pd].name = format!("{}|rd[{}->{}]", old_name, x, p);
        debug!(g.log, "[derivative] Sending rd {} from {} to {}.", pd, x, p);
    }
    g.get_mut().scope = init_scope;

    Ok(parent_derivatives)
}