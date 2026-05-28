use std::{collections::{HashSet}, fmt};

use serde::{Deserialize, Serialize};

use crate::{core::{error::GraphError, graph::Graph}, utils::new_trait::{ENew, NNew}};
use crate::core::node::{Node, NodeId, NodeMap};
use crate::core::edge::{Edge, EdgeId, EdgeMap};

/// Generic Graph structure.
# [derive(Serialize, Deserialize, Clone)]
pub struct HashGraph<N: Node, E: Edge> {
    pub(crate) node_map: NodeMap<N>,
    pub(crate) edge_map: EdgeMap<E>,
}

impl<N: Node, E: Edge> HashGraph<N, E> {
    pub fn new() -> Self {
        Self {
            node_map: NodeMap::<N>::new(),
            edge_map: EdgeMap::<E>::new()
        }
    }
}

impl<N: Node, E: Edge> Graph<N, E> for HashGraph<N, E> {
    fn get_node_ids(&self) -> Vec<&NodeId> {
        self.node_map.node_map.keys().into_iter().collect::<Vec<_>>()
    }

    fn get_edge_ids(&self) -> Vec<&EdgeId> {
        self.edge_map.edge_map.keys().into_iter().collect::<Vec<_>>()
    }

    /// Add node
    fn add_node(&mut self, node: N) -> NodeId {
        self.node_map.add(node)
    }

    /// Add edge without checking if it is valid
    fn unsafe_add_edge(&mut self, edge: E) -> EdgeId {
        self.edge_map.add(edge)
    }

    /// Get node corresponding to node id
    fn get_node(&self, node_id: &NodeId) -> Result<&N, GraphError> {
        self.node_map.get(node_id)
    }

    /// Get edge corresponding to edge id
    fn get_edge(&self, edge_id: &EdgeId) -> Result<&E, GraphError> {
        self.edge_map.get(edge_id)
    }

    /// Get node corresponding to node id
    fn get_mut_node(&mut self, node_id: &NodeId) -> Result<&mut N, GraphError> {
        self.node_map.get_mut(node_id)
    }

    /// Get edge corresponding to edge id
    fn get_mut_edge(&mut self, edge_id: &EdgeId) -> Result<&mut E, GraphError> {
        self.edge_map.get_mut(edge_id)
    }

    fn nodes(&self) -> Vec<(&NodeId, &N)> {
        self.node_map.node_map.iter().collect::<Vec<_>>()
    }

    fn edges(&self) -> Vec<(&EdgeId, &E)> {
        self.edge_map.edge_map.iter().collect::<Vec<_>>()
    }

    fn nodes_mut(&mut self) -> Vec<(&NodeId, &mut N)> {
        self.node_map.node_map.iter_mut().collect::<Vec<_>>()
    }

    fn edges_mut(&mut self) -> Vec<(&EdgeId, &mut E)> {
        self.edge_map.edge_map.iter_mut().collect::<Vec<_>>()
    }

    /// Insert node
    fn insert_node(&mut self, node_id: NodeId, node: N) -> Option<N> {
        self.node_map.insert(node_id, node)
    }

    /// Insert edge
    fn insert_edge(&mut self, edge_id: EdgeId, edge: E) -> Option<E> {
        self.edge_map.insert(edge_id, edge)
    }

    /// Remove node from graph
    fn remove_node(&mut self, node_id: &NodeId) -> Option<N> {
        self.edge_map.remove_edges_dependent_on_node(node_id);
        self.node_map.remove(node_id)
    }

    /// Remove edge from graph
    fn remove_edge(&mut self, edge_id: &EdgeId) -> Option<E> {
        self.edge_map.remove(edge_id)
    }

    /// Verify node is in graph
    fn verify_node(&self, node_id: &NodeId) -> Result<(), GraphError> {
        self.node_map.verify_node(node_id)
    }

    /// Verify edge is in graph and well defined.
    fn verify_edge_id(&self, edge_id: &EdgeId) -> Result<(), GraphError> {
        self.edge_map.verify(edge_id)?;
        let edge = self.get_edge(edge_id).unwrap();
        // Verify that the nodes in the edge are part of the graph.
        self.verify_node(edge.get_first())?;
        self.verify_node(edge.get_first())?;
        Ok(())
    }

    /// Verify edge is well defined in the graph
    fn verify_edge(&self, new_edge: &E) -> Result<(), GraphError> {
        self.verify_node(new_edge.get_first())?;
        self.verify_node(new_edge.get_second())?;
        Ok(())
    }

    fn verify(&self) -> Result<(), GraphError> {
        for edge in self.get_edge_ids().into_iter().map(|id| self.get_edge(id).unwrap()) {
            self.verify_edge(edge)?;
        }
        Ok(())
    }
}

impl<N: Node + NNew, E: Edge + ENew> HashGraph<N, E> {
    /// Add edge from nodes only if nodes already exist in graph
    pub fn add_edge_from_nodes(&mut self, first: NodeId, second: NodeId) -> Result<EdgeId, GraphError>{
        self.verify_node(&first)?;
        self.verify_node(&second)?;
        Ok(self.edge_map.add_from_nodes(first, second))
    }
}

impl<N: Node, E: Edge> fmt::Debug for HashGraph<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph {{ {:?}, {:?} }}", self.node_map, self.edge_map)?;
        Ok(())
    }
}


mod hash_graph_tests {
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
    fn test_nodes() {
        test_nodes_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_edges() {
        test_edges_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_get_edges() {
        test_get_edges_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_get_neighbors() {
        test_get_neighbors_helper::<BasicNode, BasicEdge>();
    }

    #[test]
    fn test_get_directed_neighbors() {
        test_get_directed_neighbors_helper::<BasicNode, BasicEdge>();
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
        let graph = get_example_graph::<BasicNode, BasicEdge>();
        assert_eq!(format!("{graph:?}"),
        "Graph { Nodes: [(0, {}), (1, {}), (2, {}), (4, {}), (5, {}), (6, {})], Edges: [(0, [0, 0]), (1, [0, 1]), (2, [0, 2]), (4, [2, 5]), (5, [5, 1]), (6, [4, 6]), (7, [1, 4])] }");
    }

    #[test]
    fn test_serde() {
        test_serde_helper::<BasicNode, BasicEdge>("./tests/data/save_load/basic.json");
    }
}