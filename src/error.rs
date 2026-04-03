use thiserror::Error;

use crate::edge::{EdgeId};
use crate::node::NodeId;


#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Node {:?} not found.", id)]
    NodeNotFoundError {id: NodeId},
    #[error("Edge {:?} not found.", id)]
    EdgeNotFoundError {id: EdgeId},
    #[error("Key Error")]
    KeyError,
    #[error("Verification Error")]
    VerificationError,
}