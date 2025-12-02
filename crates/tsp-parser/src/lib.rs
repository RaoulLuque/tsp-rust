use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use tsp_core::{InstanceMetadata, TSPInstance};

/// Enumeration of all possible data section keywords in a .tsp file.
///
/// The Keywords are according to the TSPLIB 95 specification.
pub enum TSPDataKeyword {
    NODE_COORD_SECTION,
    DEPOT_SECTION,
    DEMAND_SECTION,
    EDGE_DATA_SECTION,
    FIXED_EDGES_SECTION,
    DISPLAY_DATA_SECTION,
    TOUR_SECTION,
    EDGE_WEIGHT_SECTION,
}

/// Enumeration of all possible keywords in the specification part
/// of a .tsp file.
///
/// The Keywords are according to the TSPLIB 95 specification.
pub enum TSPSpecificationKeyword<'InputFile> {
    NAME(&'InputFile str),
    TYPE(ProblemType),
    COMMENT(&'InputFile str),
    DIMENSION(u32),
    CAPACITY(u32),
    EDGE_WEIGHT_TYPE(EdgeWeightType),
    EDGE_WEIGHT_FORMAT(EdgeWeightFormat),
    EDGE_DATA_FORMAT(EdgeDataFormat),
    NODE_COORD_TYPE(NodeCoordType),
    DISPLAY_DATA_TYPE(DisplayDataType),
    EOF,
}

pub enum ProblemType {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR,
}

pub enum EdgeWeightType {
    EXPLICIT,
    EUC_2D,
    EUC_3D,
    MAX_2D,
    MAX_3D,
    MAN_2D,
    MAN_3D,
    CEIL_2D,
    GEO,
    ATT,
    XRAY1,
    XRAY2,
    SPECIAL,
}

pub enum EdgeWeightFormat {
    FUNCTION,
    FULL_MATRIX,
    UPPER_ROW,
    LOWER_ROW,
    UPPER_DIAG_ROW,
    LOWER_DIAG_ROW,
    UPPER_COL,
    LOWER_COL,
    UPPER_DIAG_COL,
    LOWER_DIAG_COL,
}

pub enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST,
}

pub enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS,
}

pub enum DisplayDataType {
    COORD_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY,
}

pub fn parse_tsp_instance<P: AsRef<Path>>(instance_path: P) -> Result<TSPInstance, std::io::Error> {
    let lines = BufReader::new(File::open(instance_path)?).lines();

    let (metadata, data) = parse_metadata(&lines);

    todo!()
}

pub fn parse_metadata(input: &Lines<BufReader<File>>) -> (InstanceMetadata, &str) {
    let mut metadata = InstanceMetadata {
        name: String::new(),
        num_nodes: 0,
    };

    while let Some(Ok(line)) = input.next() {}
}
