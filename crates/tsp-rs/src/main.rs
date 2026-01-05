use std::ops::Add;

use tsp_core::instance::{
    distance::Distance,
    matrix::{Matrix, MatrixSym},
};
use tsp_solvers::held_karp;

fn main() {
    // env_logger::init();

    let tsp_instance =
        tsp_parser::parse_tsp_instance::<Matrix<Distance>>("instances/tsp_rust/12.tsp").unwrap();
    println!("Parsed TSP instance: {}", tsp_instance);
    let tsp_instance =
        tsp_parser::parse_tsp_instance::<MatrixSym<Distance>>("instances/tsp_rust/12.tsp")
            .unwrap();
    println!("Parsed TSP instance: {}", tsp_instance);
    // println!("Parsed TSP instance: {:?}", tsp_instance.raw_distances());
    // let distances_non_symmetric = tsp_instance.distance_matrix().to_edge_data_matrix();
    // let best_tour = held_karp(&distances_non_symmetric);
    // if let Some(best_tour) = &best_tour {
    //     println!("Best tour found: {:?}", best_tour.cost.0);
    // }/*  */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
