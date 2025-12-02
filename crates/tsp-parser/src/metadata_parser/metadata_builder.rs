use thiserror::Error;
use tsp_core::{
    instance::InstanceMetadata,
    tsp_lib_spec::{
        DisplayDataType, EdgeDataFormat, EdgeWeightFormat, EdgeWeightType, NodeCoordType,
        ProblemType,
    },
};

use crate::metadata_parser::MetaDataParseError;

#[derive(Error, Debug)]
pub struct InstanceMetadataBuilderError(&'static str);

impl std::fmt::Display for InstanceMetadataBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceMetadataBuilderError: {}", self.0)
    }
}

pub struct InstanceMetadataBuilder {
    name: Option<String>,
    problem_type: Option<ProblemType>,
    comment: Option<String>,
    dimension: Option<u32>,
    capacity: Option<u32>,
    edge_weight_type: Option<EdgeWeightType>,
    edge_weight_format: Option<EdgeWeightFormat>,
    edge_data_format: Option<EdgeDataFormat>,
    node_coord_type: Option<NodeCoordType>,
    display_data_type: Option<DisplayDataType>,
}

impl Default for InstanceMetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceMetadataBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            problem_type: None,
            comment: None,
            dimension: None,
            capacity: None,
            edge_weight_type: None,
            edge_weight_format: None,
            edge_data_format: None,
            node_coord_type: None,
            display_data_type: None,
        }
    }

    pub fn build(self) -> Result<InstanceMetadata, MetaDataParseError> {
        Ok(InstanceMetadata {
            name: self
                .name
                .ok_or(InstanceMetadataBuilderError("name is required"))?,
            problem_type: self
                .problem_type
                .ok_or(InstanceMetadataBuilderError("problem_type is required"))?,
            comment: self.comment,
            dimension: self
                .dimension
                .ok_or(InstanceMetadataBuilderError("dimension is required"))?,
            capacity: self.capacity,
            edge_weight_type: self
                .edge_weight_type
                .ok_or(InstanceMetadataBuilderError("edge_weight_type is required"))?,
            edge_weight_format: self.edge_weight_format,
            edge_data_format: self.edge_data_format,
            node_coord_type: self.node_coord_type.unwrap_or(NodeCoordType::NO_COORDS),
            display_data_type: self.display_data_type,
        })
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn name_mut(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn problem_type(mut self, problem_type: ProblemType) -> Self {
        self.problem_type = Some(problem_type);
        self
    }

    pub fn problem_type_mut(&mut self, problem_type: ProblemType) -> &mut Self {
        self.problem_type = Some(problem_type);
        self
    }

    pub fn comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }

    pub fn comment_mut(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn dimension(mut self, dimension: u32) -> Self {
        self.dimension = Some(dimension);
        self
    }

    pub fn dimension_mut(&mut self, dimension: u32) -> &mut Self {
        self.dimension = Some(dimension);
        self
    }

    pub fn capacity(mut self, capacity: u32) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn capacity_mut(&mut self, capacity: u32) -> &mut Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn edge_weight_type(mut self, edge_weight_type: EdgeWeightType) -> Self {
        self.edge_weight_type = Some(edge_weight_type);
        self
    }

    pub fn edge_weight_type_mut(&mut self, edge_weight_type: EdgeWeightType) -> &mut Self {
        self.edge_weight_type = Some(edge_weight_type);
        self
    }

    pub fn edge_weight_format(mut self, edge_weight_format: EdgeWeightFormat) -> Self {
        self.edge_weight_format = Some(edge_weight_format);
        self
    }

    pub fn edge_weight_format_mut(&mut self, edge_weight_format: EdgeWeightFormat) -> &mut Self {
        self.edge_weight_format = Some(edge_weight_format);
        self
    }

    pub fn edge_data_format(mut self, edge_data_format: EdgeDataFormat) -> Self {
        self.edge_data_format = Some(edge_data_format);
        self
    }

    pub fn edge_data_format_mut(&mut self, edge_data_format: EdgeDataFormat) -> &mut Self {
        self.edge_data_format = Some(edge_data_format);
        self
    }

    pub fn node_coord_type(mut self, node_coord_type: NodeCoordType) -> Self {
        self.node_coord_type = Some(node_coord_type);
        self
    }

    pub fn node_coord_type_mut(&mut self, node_coord_type: NodeCoordType) -> &mut Self {
        self.node_coord_type = Some(node_coord_type);
        self
    }

    pub fn display_data_type(mut self, display_data_type: DisplayDataType) -> Self {
        self.display_data_type = Some(display_data_type);
        self
    }

    pub fn display_data_type_mut(&mut self, display_data_type: DisplayDataType) -> &mut Self {
        self.display_data_type = Some(display_data_type);
        self
    }
}
