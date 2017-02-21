use primitives::*;
use ops::*;
use graph::*;
use errors::*;

pub fn sum(graph: &mut Graph, arg: usize, mut axes: [bool; 4]) -> Result<usize> {
    // Eliminate any unit shapes
    for &axis in Axis::iter() {
        if axes[axis as usize] && *graph.get_node(arg)?.shape.get(axis) == 1 {
            axes[axis as usize] = false;
        }
    }
    graph.apply_op(Box::new(Sum {axes: axes}), vec![arg])
}
