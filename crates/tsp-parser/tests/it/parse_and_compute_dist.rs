use std::{
    self,
    fs::File,
    io::{BufRead, BufReader},
};

use tsp_core::instance::distance::Distance;

fn check_input_file_against_golden_file(file_name: &str) {
    let input_instance =
        tsp_parser::parse_tsp_instance("../../instances/".to_owned() + file_name + ".tsp").unwrap();
    let golden_distance_data = BufReader::new(
        File::open(
            "tests/test_assets/distances/".to_owned()
                + file_name.split("/").last().unwrap()
                + ".txt",
        )
        .unwrap(),
    )
    .lines()
    .map(|line| {
        let line = line.unwrap();
        line.split(",")
            .map(|entry| Distance(entry.trim().parse::<i32>().unwrap()))
            .collect::<Vec<Distance>>()
            .into_iter()
    })
    .flatten()
    .collect::<Vec<Distance>>();

    assert_eq!(
        golden_distance_data.len(),
        input_instance.raw_distances().len()
    );
    for (i, &distance) in golden_distance_data.iter().enumerate() {
        assert_eq!(
            distance,
            input_instance.raw_distances()[i],
            "Distance data mismatch at index {} with values {:?} (expected) vs {:?} (actual)",
            i,
            distance,
            input_instance.raw_distances()[i]
        );
    }
    assert_eq!(input_instance.raw_distances(), golden_distance_data);
}

#[test]
fn test_a280() {
    check_input_file_against_golden_file("tsplib_symmetric/a280");
}

#[test]
fn test_d198() {
    check_input_file_against_golden_file("tsplib_symmetric/d198");
}

#[test]
fn test_d493() {
    check_input_file_against_golden_file("tsplib_symmetric/d493");
}
