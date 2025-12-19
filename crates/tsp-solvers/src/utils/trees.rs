use std::collections::BinaryHeap;

use tsp_core::instance::{
    distance::{DistanceMatrix, DistanceMatrixSymmetric},
    edge::{InvWeightUnEdge, UnEdge},
    node::Node,
};

use crate::CustomBitVec;

/// Compute a minimum 1-tree with given node penalties
fn min_one_tree(distances: DistanceMatrixSymmetric, penalties: &[i32]) {
    let distances_restricted_to_0_to_n_minus_1 =
        distances.restrict_to_first_n(distances.dimension - 1);
    let tree = min_spanning_tree(&distances_restricted_to_0_to_n_minus_1);
}

/// Compute a minimum spanning tree for given nodes and edges using prim's algorithm.
/// We assume that the distance matrix is valid, i.e., it models a complete graph.
///
/// Returns a vector of number of nodes - 1 edges representing the minimum spanning tree.
///
/// For more details, see https://en.wikipedia.org/wiki/Prim%27s_algorithm
fn min_spanning_tree(distance_matrix: &impl DistanceMatrix) -> Vec<UnEdge> {
    // TODO: Check if kruskal's algorithm might be faster
    let number_of_nodes = distance_matrix.dimension();

    // Track which nodes have been selected into the MST
    let mut selected = CustomBitVec::with_capacity(number_of_nodes);
    selected.resize(number_of_nodes, false);

    // Min-heap of edges to explore next (actually a max-heap with u32::MAX - cost as weight)
    let mut next_edges: BinaryHeap<InvWeightUnEdge> = BinaryHeap::with_capacity(number_of_nodes);

    // The resulting tree edges in no particular order
    let mut tree = Vec::with_capacity(number_of_nodes - 1);

    // Start from node 0
    selected.set(0, true);

    for to in 1..number_of_nodes {
        let cost = distance_matrix.get_distance_to_bigger(Node(0), Node(to));
        next_edges.push(InvWeightUnEdge {
            cost,
            from: Node(0),
            to: Node(to),
        });
    }

    for _ in 0..(number_of_nodes - 1) {
        let weighted_edge = {
            loop {
                // TODO: Check how much performance we gain by using unwrap_unchecked here
                let weighted_edge = next_edges.pop().expect(
                    "There should always be edges to explore. Otherwise the graph is disconnected.",
                );
                if !selected[weighted_edge.to.0] {
                    break weighted_edge;
                }
            }
        };

        selected.set(weighted_edge.to.0, true);
        tree.push(weighted_edge.to_edge());

        for to in 1..number_of_nodes {
            if selected[to] {
                continue;
            }
            let cost = distance_matrix.get_distance(weighted_edge.to, Node(to));
            next_edges.push(InvWeightUnEdge {
                cost,
                from: weighted_edge.to,
                to: Node(to),
            });
        }
    }

    tree
}

#[cfg(test)]
mod tests {
    use tsp_core::instance::distance::{Distance, DistanceMatrixSymmetric};

    use super::*;

    #[test]
    fn test_min_spanning_tree() {
        let distance_matrix =
            DistanceMatrixSymmetric::slow_new_from_distance_function(10, |from, to| {
                if from.0 + 1 == to.0 || from.0 == to.0 + 1 {
                    Distance(0)
                } else {
                    Distance(1)
                }
            });

        let mst = min_spanning_tree(&distance_matrix);
        assert_eq!(mst.len(), 9);
        let expected = (0..9)
            .map(|i| UnEdge {
                from: Node(i),
                to: Node(i + 1),
            })
            .collect::<Vec<_>>();
        mst.iter().for_each(|edge| {
            assert!(
                expected.contains(edge),
                "Edge {:?} not in expected MST",
                edge
            );
        });
    }
}
