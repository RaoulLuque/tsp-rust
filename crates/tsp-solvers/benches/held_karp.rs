use concorde_rs::solver::tsp_hk;
use criterion::{Criterion, criterion_group, criterion_main};
use tsp_parser::parse_tsp_instance;

fn held_karp_benchmark(c: &mut Criterion) {
    let tsp_instance = parse_tsp_instance("../../instances/bench/berlin52.tsp").unwrap();
    let lower_distance_matrix = concorde_rs::LowerDistanceMatrix {
        num_nodes: tsp_instance.metadata().dimension as u32,
        values: tsp_instance
            .raw_distances()
            .iter()
            .map(|&d| d.0 as u32)
            .collect(),
    };

    c.bench_function("Held Karp using concorde_rs", |b| {
        b.iter(|| tsp_hk(&lower_distance_matrix).unwrap())
    });
}

criterion_group!(held_karp_og, held_karp_benchmark);
criterion_main!(held_karp_og);
