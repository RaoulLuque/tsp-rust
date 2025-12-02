use crate::tsp_lib_spec::{
    DisplayDataType, EdgeDataFormat, EdgeWeightFormat, EdgeWeightType, NodeCoordType, ProblemType,
};

pub struct TSPInstance {
    metadata: InstanceMetadata,
    /// Flattened distance matrix
    ///
    /// Row major order, i.e. distance from node i to node j is at index (i * num_nodes + j).
    /// Node indexing starts at 0.
    distances: Vec<f32>,
}

impl TSPInstance {
    pub fn new_from_distances(metadata: InstanceMetadata, distances: Vec<f32>) -> Self {
        Self {
            metadata,
            distances,
        }
    }
}

pub struct InstanceMetadata {
    pub name: String,
    pub problem_type: ProblemType,
    pub comment: Option<String>,
    pub dimension: u32,
    pub capacity: Option<u32>,
    pub edge_weight_type: EdgeWeightType,
    pub edge_weight_format: Option<EdgeWeightFormat>,
    pub edge_data_format: Option<EdgeDataFormat>,
    /// Defaults to NO_COORDS
    pub node_coord_type: NodeCoordType,
    pub display_data_type: Option<DisplayDataType>,
}
