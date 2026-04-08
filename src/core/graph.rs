use std::{collections::{HashMap}, fmt};

use serde::{Deserialize, Serialize};

use crate::core::error::GraphError;
use crate::core::node::{Node, NodeId, NodeMap};
use crate::core::edge::{Edge, EdgeId, EdgeMap};

// /// For a generic graph let us have a map which takes node/edge id (stored as an int) and returns


/// Generic Graph structure.
# [derive(Serialize, Deserialize)]
pub struct Graph<N: Node, E: Edge> {
    pub(crate) node_map: NodeMap<N>,
    pub(crate) edge_map: EdgeMap<E>,
}

impl<N: Node, E: Edge> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            node_map: NodeMap::<N>::new(),
            edge_map: EdgeMap::<E>::new()
        }
    }

    pub fn get_node_ids(&self) -> Vec<&NodeId> {
        self.node_map.node_map.keys().into_iter().collect::<Vec<_>>()
    }

    pub fn get_edge_ids(&self) -> Vec<&EdgeId> {
        self.edge_map.edge_map.keys().into_iter().collect::<Vec<_>>()
    }

    /// Add node
    pub fn add_node(&mut self, node: N) -> NodeId {
        self.node_map.add(node)
    }

    /// Add edge without checking if it is valid
    fn unsafe_add_edge(&mut self, edge: E) -> EdgeId {
        self.edge_map.add(edge)
    }

    /// Add edge
    pub fn add_edge(&mut self, edge: E) -> Result<EdgeId, GraphError>{
        self.verify_edge(&edge)?;
        Ok(self.unsafe_add_edge(edge))
    }

    /// Add edge from nodes only if nodes already exist in graph
    pub fn add_edge_from_nodes(&mut self, first: NodeId, second: NodeId) -> Result<EdgeId, GraphError>{
        self.verify_node(&first)?;
        self.verify_node(&second)?;
        Ok(self.edge_map.add_from_nodes(first, second))
    }

    /// Get node corresponding to node id
    pub fn get_node(&self, node_id: &NodeId) -> Result<&N, GraphError> {
        self.node_map.get(node_id)
    }

    /// Get edge corresponding to edge id
    pub fn get_edge(&self, edge_id: &EdgeId) -> Result<&E, GraphError> {
        self.edge_map.get(edge_id)
    }

    /// Insert node
    pub fn insert_node(&mut self, node_id: NodeId, node: N) -> Option<N> {
        self.node_map.insert(node_id, node)
    }

    /// Insert edge
    pub fn insert_edge(&mut self, edge_id: EdgeId, edge: E) -> Option<E> {
        self.edge_map.insert(edge_id, edge)
    }

    /// Remove node from graph
    pub fn remove_node(&mut self, node_id: &NodeId) -> Option<N> {
        self.edge_map.remove_edges_dependent_on_node(node_id);
        self.node_map.remove(node_id)
    }

    /// Remove edge from graph
    pub fn remove_edge(&mut self, edge_id: &EdgeId) -> Option<E> {
        self.edge_map.remove(edge_id)
    }

    /// Verify node is in graph
    pub fn verify_node(&self, node_id: &NodeId) -> Result<(), GraphError> {
        self.node_map.verify_node(node_id)
    }

    /// Verify edge is in graph and well defined.
    pub fn verify_edge_id(&self, edge_id: &EdgeId) -> Result<(), GraphError> {
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

impl<N: Node, E: Edge> fmt::Debug for Graph<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph {{ {:?}, {:?} }}", self.node_map, self.edge_map)?;
        Ok(())
    }
}

// impl<N: Node + Serialize, E: Edge + Serialize> Serialize for Graph<N, E> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         // serializer.serialize_map(len)
//     }
// }


pub(crate) mod graph_tests {
    use std::path::Path;

    use serde::{Deserialize, Serialize};

    use crate::core::{edge::EdgeMap, graph::{Edge, EdgeId, Graph, Node, NodeId}, node::NodeMap};
    use crate::utils::saveload::{load_from_json, save_to_json};

    pub fn get_example_graph<N: Node, E: Edge>() -> Graph<N, E> {
        let mut graph = Graph { node_map: NodeMap::<N>::new(), edge_map: EdgeMap::<E>::new() };
        for _ in 0..7 { graph.add_node(N::new()); }
        graph.remove_node(&NodeId(3));
        let _ = graph.add_edge(E::new(NodeId(0), NodeId(0)));
        let _ = graph.add_edge(E::new(NodeId(0), NodeId(1)));
        let _ = graph.add_edge(E::new(NodeId(0), NodeId(2)));
        let id = graph.add_edge(E::new(NodeId(0), NodeId(2)));
        graph.remove_edge(&id.unwrap());
        let _ = graph.add_edge(E::new(NodeId(2), NodeId(5)));
        let _ = graph.add_edge(E::new(NodeId(5), NodeId(1)));
        let _ = graph.add_edge(E::new(NodeId(4), NodeId(6)));
        let _ = graph.add_edge(E::new(NodeId(1), NodeId(4)));
        graph
    }

    // Add

    pub fn test_add_node_helper<N: Node + Default, E: Edge>() {
        let mut graph = Graph::<N, E>::new();
        let node_id0 = graph.add_node(N::default());
        let node_id1 = graph.add_node(N::default());
        assert_eq!(node_id0, NodeId(0));
        assert_eq!(node_id1, NodeId(1));

        assert!(graph.verify_node(&NodeId(0)).is_ok());
        assert!(graph.verify_node(&NodeId(1)).is_ok());
        assert!(graph.verify_node(&NodeId(2)).is_err());
    }

    pub fn test_add_edge_helper<N: Node + Default, E: Edge>() {
        let mut graph = Graph::<N, E>::new();
        graph.add_node(N::default());
        graph.add_node(N::default());
        let mut edge = E::default();
        edge.set_first(NodeId(0));
        edge.set_first(NodeId(1));
        graph.add_edge(edge).unwrap();
        assert!(graph.add_edge_from_nodes(NodeId(0), NodeId(1)).is_ok());
    }

    pub fn test_add_edge_from_nodes_helper<N: Node + Default, E: Edge>() {
        let mut graph = Graph::<N, E>::new();
        assert!(graph.add_edge_from_nodes(NodeId(0), NodeId(1)).is_err());
        graph.add_node(N::default());
        graph.add_node(N::default());
        assert!(graph.add_edge_from_nodes(NodeId(0), NodeId(1)).is_ok());
    }

    
    pub fn test_add_edge_with_invalid_nodes_helper<N: Node, E: Edge>() {
        let mut graph = Graph::<N, E>::new();
        let edge = E::default();
        let edge2 = E::default();
        println!("{edge:?}");
        graph.unsafe_add_edge(edge);
        assert!(graph.verify().is_err());
        assert!(graph.add_edge(edge2).is_err());
    }

    // Get

    pub fn test_get_node_helper<N: Node, E: Edge>() {
        let graph = get_example_graph::<N, E>();
        assert!(graph.get_node(&NodeId(3)).is_err());
        assert!(graph.get_node(&NodeId(7)).is_err());
        assert!(graph.get_node(&NodeId(0)).is_ok())
    }

    pub fn test_get_edge_helper<N: Node, E: Edge>() {
        let graph = get_example_graph::<N, E>();
        assert!(graph.get_edge(&EdgeId(3)).is_err());
        assert!(graph.get_edge(&EdgeId(8)).is_err());
        assert!(graph.get_edge(&EdgeId(7)).is_ok());
        assert!(graph.get_edge(&EdgeId(0)).is_ok())
    }

    // Insert

    pub fn test_insert_node_helper<N: Node, E: Edge>() {
        let mut graph = get_example_graph::<N, E>();
        assert!(graph.insert_node(NodeId(0), N::new()).is_some());
        assert!(graph.insert_node(NodeId(3), N::new()).is_none());
        assert!(graph.insert_node(NodeId(7), N::new()).is_none());
    }

    pub fn test_insert_edge_helper<N: Node, E: Edge>() {
        let mut graph = get_example_graph::<N, E>();
        assert!(graph.insert_edge(EdgeId(0), E::default()).is_some());
        assert!(graph.insert_edge(EdgeId(3), E::default()).is_none());
        assert!(graph.insert_edge(EdgeId(7), E::default()).is_some());
        assert!(graph.insert_edge(EdgeId(8), E::default()).is_none());
    }

    // Test Remove

    pub fn test_remove_node_helper<N: Node, E: Edge>() {
        let mut graph = get_example_graph::<N, E>();
        assert!(graph.remove_node(&NodeId(3)).is_none());
        assert!(graph.remove_node(&NodeId(7)).is_none());
        assert_eq!(graph.get_node_ids().len(), 6);
        assert_eq!(graph.get_edge_ids().len(), 7);
        assert!(graph.remove_node(&NodeId(0)).is_some());
        assert_eq!(graph.get_node_ids().len(), 5);
        assert_eq!(graph.get_edge_ids().len(), 4);

        assert!(graph.remove_node(&NodeId(1)).is_some());
        assert_eq!(graph.get_node_ids().len(), 4);
        assert_eq!(graph.get_edge_ids().len(), 2);
    }

    pub fn test_remove_edge_helper<N: Node, E: Edge>() {
        let mut graph = get_example_graph::<N, E>();
        assert!(graph.remove_edge(&EdgeId(0)).is_some());
        assert!(graph.remove_edge(&EdgeId(3)).is_none());
        assert!(graph.remove_edge(&EdgeId(7)).is_some());
        assert!(graph.remove_edge(&EdgeId(8)).is_none());
    }

    // Test debug

    pub fn test_debug_helper<N: Node, E: Edge>() {
        let node_map = get_example_graph::<N, E>();
        assert_eq!(format!("{node_map:?}"),
        "Graph { Nodes: [0, 1, 2, 4, 5, 6], Edges: [[0, 0], [0, 1], [0, 2], [1, 4], [2, 5], [4, 6], [5, 1]] }");
    }

    // test serde

    pub fn test_serde_helper<N: Node + Serialize + for<'a> Deserialize<'a>, E: Edge + Serialize + for<'a> Deserialize<'a>>() {
        let path = Path::new("./tests/data/save-load.json");
        let g = get_example_graph::<N, E>();
        let initial_debug = format!("{:?}", g);

        // Save
        assert!(save_to_json(path, &g).is_ok());

        // load
        let g_save_load = load_from_json::<Graph<N, E>>(&path);
        assert!(g_save_load.is_ok());

        assert_eq!(initial_debug, format!("{:?}", g_save_load.unwrap()))
    }
}
