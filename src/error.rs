use std::path::Path;

use thiserror::Error;

use crate::edge::{EdgeId};
use crate::node::NodeId;


#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Node {:?} not found.", id)]
    NodeNotFoundError {id: NodeId},
    #[error("Edge {:?} not found.", id)]
    EdgeNotFoundError {id: EdgeId},
}

#[derive(Error, Debug)]
pub enum IoError<'a> {
    #[error("File {:?} could not be read.", path)]
    ReadError {path: &'a Path},
    #[error("File {:?} could not be written to.", path)]
    WriteError {path: &'a Path},
}