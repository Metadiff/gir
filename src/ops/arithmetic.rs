use ops::interface::*;
use primitives::*;
use graph::*;
use errors::*;
use api::ids;

#[derive(Debug, Clone)]
pub struct Add {}


impl Operator for Add {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        Ok(g.get_node(x)?.ancestors.iter()
            .filter(|id| flow_tree[**id])
            .map(|id| (*id, dx)).collect())
    }

    fn verify_args(&self, g: &Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        Ok(default::broadcast_shapes(g, meta.name, args)?)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static ADD: OperatorMetaData = OperatorMetaData{
            name: "Add",
            arity: Arity::Nary,
            num_outputs: 1,
            differential_parents: ::std::usize::MAX,
            ordered_parents: false,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &ADD
    }
}

#[derive(Debug, Clone)]
pub struct Neg {}


impl Operator for Neg {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            Ok(vec![(ancestor, ids::neg(g, dx)?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn verify_args(&self, g: &Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        if g.get_node(args[0])?.data_type == FundamentalType::Boolean {
            return Err(ErrorKind::InvalidArguments(
                String::new() + meta.name, args,
                "Not applicable to boolean expressions.".into()).into())
        }
        Ok(args)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static NEG: OperatorMetaData = OperatorMetaData{
            name: "Neg",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
            elementwise: true,
            type_preserving: true,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &NEG
    }
}

#[derive(Debug, Clone)]
pub struct Mul {}

impl Operator for Mul {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let n = g.get_node(x)?.ancestors.len();
        match n {
            0|1 => panic!("Reverse diff encountered a Mul node with 0 or 1 ancestors."),
            2 => {
                let mut result = Vec::new();
                let ids = g.get_node(x)?.ancestors.clone();
                if flow_tree[ids[0]] {
                    let dp = ids::mul(g, vec![dx, ids[1]])?;
                    result.push((ids[0], dp));
                }
                if flow_tree[ids[1]] {
                    let dp = ids::mul(g, vec![dx, ids[0]])?;
                    result.push((ids[1], dp));
                }
                Ok(result)
            },
            _ => unimplemented!()
        }
    }

    fn verify_args(&self, g: &Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        Ok(default::broadcast_shapes(g, meta.name, args)?)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static MUL: OperatorMetaData = OperatorMetaData{
            name: "Mul",
            arity: Arity::Nary,
            num_outputs: 1,
            differential_parents: ::std::usize::MAX,
            ordered_parents: false,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &MUL
    }
}

#[derive(Debug, Clone)]
pub struct Div {}

impl Operator for Div {
    #[allow(unused_variables, unused_mut)]
    fn reverse_diff(&self, g: &Graph, x: usize, dx: usize, flow_tree: &Vec<bool>)
                    -> Result<Vec<(usize, usize)>> {
        let ancestor = g.get_node(x)?.ancestors[0];
        if flow_tree[ancestor] {
            let minus_one = g.constant_scalar(-1.0, g.get_node(x)?.data_type).id;
            Ok(vec![(ancestor, ids::mul(g, vec![dx, x, x, minus_one])?)])
        } else {
            Ok(Vec::new())
        }
    }

    fn verify_args(&self, g: &Graph, args: Vec<usize>) -> Result<Vec<usize>> {
        let meta = self.get_meta();
        let args = default::verify_args(meta, g, args)?;
        if g.get_node(args[0])?.data_type == FundamentalType::Boolean {
            return Err(ErrorKind::InvalidArguments(
                String::new() + meta.name, args,
                "Not applicable to boolean expressions.".into()).into())
        }
        Ok(args)
    }

    fn clone_box(&self) -> Box<Operator> {
        Box::new(self.clone())
    }

    fn get_meta(&self) -> &OperatorMetaData {
        static DIV: OperatorMetaData = OperatorMetaData{
            name: "Div",
            arity: Arity::Unary,
            num_outputs: 1,
            differential_parents: 1,
            ordered_parents: true,
            elementwise: true,
            type_preserving: false,
            reduction: false,
            differentiable: true,
            scalar_output: false,
            shape_operator: false,
            fixed_output_type: None,
        };
        &DIV
    }
}
