use std::fmt;

use serde::{Deserialize, Serialize};

use crate::core::node::Node;
use crate::core::{edge::Edge, node::NodeId};
use crate::core::weight::{Weight, Weighted};

/// An Edge at minimum needs a reference to two nodes.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WeightedEdge {
    first: NodeId,
    second: NodeId,
    weight: Weight,
}

impl WeightedEdge {
    pub fn new(first: NodeId, second: NodeId, weight: Weight) -> Self {
        Self {first, second, weight}
    }
}

impl fmt::Debug for WeightedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]: {:?}", self.first, self.second, self.weight)?;
        Ok(())
    }
}

impl Edge for WeightedEdge {
    fn contains_node(&self, node_id: &NodeId) -> bool {
        return node_id == &self.first || node_id == &self.second
    }
    
    fn get_first(&self) -> &NodeId { &self.first }
    fn get_second(&self) -> &NodeId { &self.second }
    fn set_first(&mut self, node_id: NodeId) { self.first = node_id }
    fn set_second(&mut self, node_id: NodeId) { self.second = node_id }
    
    fn dbg(&self) -> impl std::fmt::Debug {
        self
    }
}

impl Weighted for WeightedEdge {
    fn get_weight(&self) -> &Weight { &self.weight }
    fn set_weight(&mut self, weight: Weight) { self.weight = weight; }
}

#[cfg(test)]
mod weighted_edge_tests {
    use crate::core::node::NodeId;
    use crate::core::weight::{Weight, Weighted};
    use crate::graphs::weights::WeightedEdge;

    use crate::core::edge::{Edge, EdgeMap, edge_tests::*};

    fn get_weighted_edge_map<E: Edge + Weighted>() -> EdgeMap<E> {
        let mut edge_map = EdgeMap::<E>::new();
        for w in 0..3 {
            let mut e = E::default();
            e.set_weight(Weight(w));
            edge_map.add(e);
        }
        return edge_map;
    }
    
    #[test]
    fn test_weights() {
        let mut edge = WeightedEdge::new(NodeId(0), NodeId(1), Weight(2));
        assert_eq!(edge.get_weight(), &Weight(2));
        edge.set_weight(Weight(9));
        assert_eq!(edge.get_weight(), &Weight(9));
    }

    #[test]
    fn test_add() {
        test_add_helper::<WeightedEdge>();
    }

    #[test]
    fn test_add_from_nodes() {
        test_add_from_nodes_helper::<WeightedEdge>();
    }

    #[test]
    fn test_get() {
        test_get_helper::<WeightedEdge>();
    }
    #[test]
    fn test_remove() {
        test_remove_helper::<WeightedEdge>();
    }

    #[test]
    fn test_remove_edges_dependent_on_node() {
        test_remove_edges_dependent_on_node_helper::<WeightedEdge>();
    }

    #[test]
    fn test_insert() {
        test_insert_helper::<WeightedEdge>();
    }

    #[test]
    fn test_debug() {
        let edge_map = get_weighted_edge_map::<WeightedEdge>();
        assert_eq!(format!("{edge_map:?}"), "Edges: [[0, 0]: 0, [0, 0]: 1, [0, 0]: 2]");
    }
}

mod weighted_graph_tests {
    use crate::core::graph::graph_tests::*;

    use crate::graphs::{basic::BasicNode, weights::WeightedEdge};

    #[test]
    fn test_add_node() {
        test_add_node_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_add_edge() {
        test_add_edge_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_add_edge_from_nodes() {
        test_add_edge_from_nodes_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_add_edge_with_invalid_nodes() {
        test_add_edge_with_invalid_nodes_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_get_node() {
        test_get_node_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_get_edge() {
        test_get_edge_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_insert_node() {
        test_insert_node_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_insert_edge() {
        test_insert_edge_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_remove_node() {
        test_remove_node_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_remove_edge() {
        test_remove_edge_helper::<BasicNode, WeightedEdge>();
    }

    #[test]
    fn test_debug() {
        let graph = get_example_graph::<BasicNode, WeightedEdge>();
        assert_eq!(format!("{graph:?}"),
        "Graph { Nodes: [0, 1, 2, 4, 5, 6], Edges: [[0, 0]: 0, [0, 1]: 0, [0, 2]: 0, [1, 4]: 0, [2, 5]: 0, [4, 6]: 0, [5, 1]: 0] }");
    }

    #[test]
    fn test_serde() {
        test_serde_helper::<BasicNode, WeightedEdge>();
    }
}