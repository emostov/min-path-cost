use std::{cell::RefCell, rc::Rc};

type NodePointer = Rc<RefCell<Node>>;
type Input = Vec<Vec<NodePointer>>;

/// Unidirectional, weighted edge to a `Node`.
pub struct Edge {
	/// Weight of the edge.
	weight: usize, // Assume no negative values, so we are using unsigned integers.
	/// Node the edge leads to.
	destination: NodePointer,
}

impl Edge {
	pub fn new(weight: usize, destination: NodePointer) -> Self {
		Edge { weight, destination }
	}
}

/// Node for min path cost problem.
#[derive(Default)]
pub struct Node {
	/// Edges to destination node.
	edges: Vec<Edge>,
	/// Weight of the "lightest" path to get to this node.
	maybe_min_path: Option<usize>,
}

impl Node {
	pub fn new(edges: Vec<Edge>) -> Self {
		Node { edges, maybe_min_path: None }
	}
}



// Given an NxN matrix of connected “nodes” with weighted edges, where a node in
// row i can only connect to nodes in row i+1, find the least cost path from row
// 0 to row N-16
// Inputs: 2d matrix of elements of type `Node`
// Output: ~~integer~~ `Option<usize>` where `None` denotes no possible path
//
// Assumes inputs are validated
pub fn min_path_cost(input: Input) -> Option<usize> {
	let mut final_min_path = None;
	let last_row = input.len() - 1;

	for (row_idx, row) in input.iter().enumerate() {
		for node in row.iter() {
			let node = node.borrow_mut();
			if row_idx != 0 && node.maybe_min_path.is_none() {
				// We are at an inaccessible node.
				continue;
			} else if row_idx == last_row {
				// We are on the last row we look for the min path to get here.
				if let Some(min_path) = node.maybe_min_path {
					if final_min_path.unwrap_or(usize::MAX) > min_path {
						final_min_path = Some(min_path)
					}
				}
			} else {
				// We are at a non-terminal node.
				for edge in node.edges.iter() {
					let weight_to_dest = if row_idx == 0 {
						// This is a starting node, so the path only consists of 1 edge.
						edge.weight
					} else if let Some(src_min_path) = node.maybe_min_path {
						edge.weight + src_min_path
					} else {
						// We already skipped non-accessible nodes, thus `node.maybe_min_path`
						// is always `Some` so we should never reach here.
						continue;
					};

					// Potentially update the destination node's min path.
					let dest_maybe_min_path = edge.destination.borrow().maybe_min_path;
					match dest_maybe_min_path {
						None => edge.destination.borrow_mut().maybe_min_path = Some(weight_to_dest),
						Some(dest_min_path) if dest_min_path > weight_to_dest => {
							edge.destination.borrow_mut().maybe_min_path = Some(weight_to_dest)
						}
						_ => (),
					};
				}
			}
		}
	}

	return final_min_path;
}

#[cfg(test)]
mod tests {
	use super::*;

	fn node_pointer(edges: Vec<Edge>) -> NodePointer {
		let node = Node::new(edges);
		Rc::new(RefCell::new(node))
	}

	#[test]
	fn it_works() {
		let node_pointer_default = Rc::new(RefCell::new(Node::default()));
		let sanity_input = vec![
			vec![node_pointer_default]
		];
		assert_eq!(min_path_cost(sanity_input), None);

		let r2c0 = node_pointer(vec![]);
		let r2c1 = node_pointer(vec![]);

		let r1c0 = node_pointer(vec![Edge::new(6, r2c0.clone())]);
		let r1c1 = node_pointer(vec![Edge::new(4, r2c0.clone()), Edge::new(5, r2c1.clone())]);

		let r0c0 = node_pointer(vec![Edge::new(2, r1c0.clone()), Edge::new(3, r1c1.clone())]);
		let r0c1 = node_pointer(vec![Edge::new(0, r1c0.clone()), Edge::new(1, r1c1.clone())]);

		let simple_input = vec![
			vec![r0c0, r0c1],
			vec![r1c0, r1c1],
			vec![r2c0, r2c1]
		];
		assert_eq!(min_path_cost(simple_input), Some(5));
	}
}
