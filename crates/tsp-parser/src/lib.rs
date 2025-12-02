use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use thiserror::Error;
use tsp_core::instance::TSPInstance;

use crate::metadata::{MetaDataParseError, parse_metadata};

pub mod distance_data;
pub mod metadata;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    MetaDataParsing(#[from] MetaDataParseError),
}

pub fn parse_tsp_instance<P: AsRef<Path>>(instance_path: P) -> Result<TSPInstance, ParserError> {
    let mut lines = BufReader::new(File::open(instance_path)?).lines();

    let (metadata, data_keyword, input) = parse_metadata(&mut lines)?;

    parse_data_sections(input, data_keyword, metadata);

    todo!()
}
