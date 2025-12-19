use std::collections::BinaryHeap;

use tsp_core::instance::{distance::DistanceMatrix, edge::WeightedEdge};

use crate::CustomBitVec;

/// Compute a minimum 1-tree with given node penalties
fn min_one_tree() {}

/// Compute a minimum spanning tree for given nodes and edges using prim's algorithm.
///
/// For more details, see https://en.wikipedia.org/wiki/Prim%27s_algorithm
fn min_spanning_tree(distance_matrix: impl DistanceMatrix) {
    let number_of_nodes = distance_matrix.dimension();

    // Track which nodes have been selected into the MST
    let mut selected = CustomBitVec::with_capacity(number_of_nodes);
    selected.resize(number_of_nodes, false);

    // Min-heap of edges to explore next (actually a max-heap with u32::MAX - cost as weight)
    let mut next_edges: BinaryHeap<WeightedEdge> = BinaryHeap::with_capacity(number_of_nodes);

    // The resulting tree edges in no particular order
    let mut tree = Vec::with_capacity(number_of_nodes - 1);

    // Start from node 0
    selected.set(0, true);

    for to in 1..number_of_nodes {
        let cost = distance_matrix.get_distance(0, to);
        next_edges.push(WeightedEdge {
            cost: convert_weight(cost),
            from: 0,
            to,
        });
    }

    while selected.count_ones() < number_of_nodes {
        let weighted_edge = next_edges
            .pop()
            .expect("There should be edges left, else the graph is disconnected");
        if selected[weighted_edge.to] {
            continue;
        }

        selected.set(weighted_edge.to, true);
        tree.push(weighted_edge.to_edge());

        for to in 1..number_of_nodes {
            if selected[to] {
                continue;
            }
            let cost = distance_matrix.get_distance(weighted_edge.to, to);
            next_edges.push(WeightedEdge {
                cost: convert_weight(cost),
                from: weighted_edge.to,
                to,
            });
        }
    }
}

/// Convert a weight to use it in a max-heap as if it were a min-heap
fn convert_weight(weight: u32) -> u32 {
    // SAFETY: Overflow is impossible here as u32::MAX >= weight since weight is u32
    unsafe { u32::MAX.unchecked_sub(weight) }
}
