use tsp_solvers::held_karp::held_karp;

fn main() {
    let tsp_instance = tsp_parser::parse_tsp_instance("instances/bench/berlin52.tsp").unwrap();
    // println!("Parsed TSP instance: {:?}", tsp_instance.raw_distances());
    let best_tour = held_karp(tsp_instance.distances());
    println!("Best tour found: {:?}", best_tour);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
