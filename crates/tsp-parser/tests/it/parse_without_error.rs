use tsp_core::instance::{
    distance::Distance,
    matrix::{Matrix, MatrixSym},
};
use tsp_macros::test_fn_on_all_instances;

fn parse_instance_symmetric(path: &str) {
    let parsing_result = tsp_parser::parse_tsp_instance::<MatrixSym<Distance>>(path.to_owned());
    assert!(parsing_result.is_ok());
}

fn parse_instance_non_symmetric(path: &str) {
    let parsing_result = tsp_parser::parse_tsp_instance::<Matrix<Distance>>(path.to_owned());
    assert!(parsing_result.is_ok());
}

#[rust_analyzer::skip]
test_fn_on_all_instances!(parse_instance_symmetric, symmetric);
#[rust_analyzer::skip]
test_fn_on_all_instances!(parse_instance_non_symmetric, non_symmetric);
