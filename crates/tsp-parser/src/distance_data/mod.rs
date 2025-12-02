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

use tsp_core::{instance::InstanceMetadata, tsp_lib_spec::TSPDataKeyword};

pub fn parse_data_sections(
    input: &Lines<BufReader<File>>,
    data_keyword: TSPDataKeyword,
    metadata: InstanceMetadata,
) -> Vec<f32> {
    let distance_data: Vec<f32> = match data_keyword {
        TSPDataKeyword::NODE_COORD_SECTION => {
            retrieve_distance_data_from_node_coord_section(input, &metadata)
        }
        _ => todo!("Other data sections are not yet implemented"),
    };

    distance_data
}

fn retrieve_distance_data_from_node_coord_section(
    input: &Lines<BufReader<File>>,
    metadata: &InstanceMetadata,
) -> Vec<u32> {
    let distance_data: Vec<u32> =
        Vec::with_capacity(metadata.dimension as usize * metadata.dimension as usize);
    let point_data: Vec<(f64, f64)> = Vec::with_capacity(metadata.dimension as usize);

    distance_data
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
