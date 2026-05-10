use crate::core::{edge::Edge, node::{Node, NodeId}};

pub(crate) trait NNew : Node + Default {
    fn new() -> Self;
}

pub(crate) trait ENew : Edge + Default {
    fn new(first: NodeId, second: NodeId) -> Self;
}

