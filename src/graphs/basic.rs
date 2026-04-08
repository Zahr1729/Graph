use std::fmt::{self, Debug};

use serde::{Serialize, Deserialize};

use crate::core::{edge::Edge, node::{Node, NodeId}};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BasicNode();

impl Node for BasicNode {
    fn new() -> Self {
        Self()
    }
}



/// An Edge at minimum needs a reference to two nodes.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BasicEdge {
    first: NodeId,
    second: NodeId,
}

impl fmt::Debug for BasicEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.first, self.second)?;
        Ok(())
    }
}

impl Edge for BasicEdge {
    fn contains_node(&self, node_id: &NodeId) -> bool {
        return node_id == &self.first || node_id == &self.second
    }
    
    fn get_first(&self) -> &NodeId { &self.first }
    fn get_second(&self) -> &NodeId { &self.second }
    fn set_first(&mut self, node_id: NodeId) { self.first = node_id }
    fn set_second(&mut self, node_id: NodeId) { self.second = node_id }
    
    fn dbg(&self) -> impl Debug {
        BasicEdge{first:self.get_first().clone(), second:self.get_second().clone()};
    }
}

#[cfg(test)]
mod basic_node_tests {
    use crate::graphs::basic::BasicNode;

    use crate::core::node::node_tests::*;

    #[test]
    fn test_add() {
        test_add_helper::<BasicNode>();
    }

    #[test]
    fn test_get() {
        test_get_helper::<BasicNode>();
    }
    #[test]
    fn test_remove() {
        test_remove_helper::<BasicNode>();
    }

    #[test]
    fn test_insert() {
        test_insert_helper::<BasicNode>();
    }

    #[test]
    fn test_debug() {
        test_debug_helper::<BasicNode>();
    }
}

#[cfg(test)]
mod basic_edge_tests {
    use crate::graphs::basic::BasicEdge;

    use crate::core::edge::edge_tests::*;

    #[test]
    fn test_add() {
        test_add_helper::<BasicEdge>();
    }

    #[test]
    fn test_add_from_nodes() {
        test_add_from_nodes_helper::<BasicEdge>();
    }

    #[test]
    fn test_get() {
        test_get_helper::<BasicEdge>();
    }
    #[test]
    fn test_remove() {
        test_remove_helper::<BasicEdge>();
    }

    #[test]
    fn test_remove_edges_dependent_on_node() {
        test_remove_edges_dependent_on_node_helper::<BasicEdge>();
    }

    #[test]
    fn test_insert() {
        test_insert_helper::<BasicEdge>();
    }

    #[test]
    fn test_debug() {
        test_debug_helper::<BasicEdge>();
    }
}

mod basic_graph_tests {
    use crate::graphs::basic::{BasicNode, BasicEdge};

    use crate::core::graph::graph_tests::*;

    #[test]
    fn test_add_node() {
        test_add_node_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_add_edge() {
        test_add_edge_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_add_edge_from_nodes() {
        test_add_edge_from_nodes_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_add_edge_with_invalid_nodes() {
        test_add_edge_with_invalid_nodes_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_get_node() {
        test_get_node_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_get_edge() {
        test_get_edge_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_insert_node() {
        test_insert_node_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_insert_edge() {
        test_insert_edge_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_remove_node() {
        test_remove_node_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_remove_edge() {
        test_remove_edge_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_debug() {
        test_debug_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_serde() {
        test_serde_helper::<BasicNode, BasicEdge>();
    }
}