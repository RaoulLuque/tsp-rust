use tsp_core::instance::{
    UnTour,
    edge::{UnEdge, distance::Distance},
    node::Node,
};
use tsp_solvers::held_karp;

#[test]
fn test_held_karp_on_12() {
    let tsp_instance = tsp_parser::parse_tsp_instance("../../instances/bench/12.tsp").unwrap();
    let distances_non_symmetric = tsp_instance.distances().to_non_symmetric();
    let best_tour = held_karp(&distances_non_symmetric).unwrap();
    let mut edges = Vec::with_capacity(12);
    for i in 0..12 {
        edges.push(UnEdge {
            from: Node(i),
            to: Node((i + 1) % 12),
        });
    }
    let expected_tour = UnTour {
        edges,
        cost: Distance(1200),
    };
    assert_eq!(best_tour, expected_tour);
}
