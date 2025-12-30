use tsp_solvers::held_karp;

fn main() {
    env_logger::init();

    let tsp_instance =
        tsp_parser::parse_tsp_instance("instances/tsplib_symmetric/a280.tsp").unwrap();
    // println!("Parsed TSP instance: {:?}", tsp_instance.raw_distances());
    let distances_non_symmetric = tsp_instance.distances().to_non_symmetric();
    let best_tour = held_karp(&distances_non_symmetric);
    if let Some(best_tour) = &best_tour {
        println!("Best tour found: {:?}", best_tour.cost.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
