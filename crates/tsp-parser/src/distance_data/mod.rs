/// Module for parsing distance data sections from TSP instance files.
///
/// According to TSPLIB95, distance data can be represented in various formats,
/// such as explicit distance matrices or coordinate-based representations.
///
/// Distance values are required to be non-negative integers. Computations are expected to be
/// carried out in double precision arithmetic, i.e. `f64` in Rust.
use std::{
    fs::File,
    io::{BufReader, Lines},
};

use tsp_core::{
    instance::{InstanceMetadata, distances::DistancesSymmetric},
    tsp_lib_spec::TSPDataKeyword,
};

pub fn parse_data_sections(
    input: &mut Lines<BufReader<File>>,
    data_keyword: TSPDataKeyword,
    metadata: &InstanceMetadata,
) -> DistancesSymmetric {
    match data_keyword {
        TSPDataKeyword::NODE_COORD_SECTION => {
            retrieve_distance_data_from_node_coord_section(input, metadata)
        }
        _ => todo!("Other data sections are not yet implemented"),
    }
}

fn retrieve_distance_data_from_node_coord_section(
    input: &mut Lines<BufReader<File>>,
    metadata: &InstanceMetadata,
) -> DistancesSymmetric {
    let node_data = retrieve_node_data_from_node_coord_section(input, metadata);

    match metadata.edge_weight_type {
        tsp_core::tsp_lib_spec::EdgeWeightType::EUC_2D => {
            compute_distances_euclidean(node_data, metadata.dimension)
        }
        _ => unimplemented!(
            "Edge weight type {:?} is not yet implemented",
            metadata.edge_weight_type
        ),
    }
}

fn retrieve_node_data_from_node_coord_section(
    input: &mut Lines<BufReader<File>>,
    metadata: &InstanceMetadata,
) -> Vec<(f64, f64)> {
    let mut point_data: Vec<(f64, f64)> = Vec::with_capacity(metadata.dimension);

    for line in input {
        let line = line.expect("Failed to read line from input");
        if line.trim() == "EOF" {
            break;
        }

        let mut parts = line.split_whitespace();
        let _node_index: usize = parts
            .next()
            .expect("Missing node index")
            .parse()
            .expect("Failed to parse node index");
        let x: f64 = parts
            .next()
            .expect("Missing x coordinate")
            .parse()
            .expect("Failed to parse x coordinate");
        let y: f64 = parts
            .next()
            .expect("Missing y coordinate")
            .parse()
            .expect("Failed to parse y coordinate");

        point_data.push((x, y));
    }

    point_data
}

fn compute_distances_euclidean(
    point_data: Vec<(f64, f64)>,
    dimension: usize,
) -> DistancesSymmetric {
    // TODO: Handle symmetric / asymmetric cases
    let mut distance_data = vec![0; dimension * dimension];

    for i in 0..dimension {
        for j in 0..dimension {
            let distance = compute_euclidean_distance(&point_data[i], &point_data[j]);
            distance_data[i * dimension + j] = distance;
        }
    }

    DistancesSymmetric::new_from_data(distance_data, dimension)
}

/// Computes the Euclidean distance between two points as defined in TSPLIB95.
fn compute_euclidean_distance(point_a: &(f64, f64), point_b: &(f64, f64)) -> u32 {
    nint(((point_a.0 - point_b.0).powi(2) + (point_a.1 - point_b.1).powi(2)).sqrt())
}

/// Nearest integer function as defined in TSPLIB95.
///
/// Expects a non-negative float input.
fn nint(x: f64) -> u32 {
    (x + 0.5) as u32
}
