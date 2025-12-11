fn main() {
    let tsp_instance = tsp_parser::parse_tsp_instance("instances/bench/d493.tsp").unwrap();
    println!("Parsed TSP instance: {:?}", tsp_instance.raw_distances());
}
