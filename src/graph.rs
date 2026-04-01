use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::error::GraphError;

/// For a generic graph let us have a map which takes node/edge id (stored as an int) and returns

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub struct NodeId(usize);
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub struct EdgeId(usize);

impl EdgeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl NodeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Copy, Hash, Default)]
pub struct Node{
}

/// An Edge at minimum needs a reference to two nodes.
#[derive(Clone, Copy, Hash, Default)]
pub struct Edge {
    first: NodeId,
    second: NodeId,
}

impl Edge {
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        return node_id == &self.first || node_id == &self.second
    }
}

/// Generic Graph structure.
pub struct Graph {
    node_counter: NodeId,
    edge_counter: EdgeId,

    node_map: HashMap<NodeId, Node>,
    edge_map: HashMap<EdgeId, Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            node_counter: NodeId(0),
            edge_counter: EdgeId(0),
            node_map: HashMap::new(),
            edge_map: HashMap::new()
        }
    }

    /// Add node
    pub fn add_node(&mut self, node: Node) {
        self.node_map.insert(self.node_counter, node);
        self.node_counter.increment();
    }

    /// Add new node
    pub fn add_new_node(&mut self) {
        let new_node = Node::default();
        self.add_node(new_node);
    }

    /// Add edge without checking if such an edge is well definied.
    fn unsafe_add_edge(&mut self, edge: Edge) {
        self.edge_map.insert(self.edge_counter, edge);
        self.edge_counter.increment();
    }

    /// Add edge, failing if the node id do not correspond to nodes.
    fn add_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        self.verify_node(edge.first)?;
        self.verify_node(edge.second)?;
        self.unsafe_add_edge(edge);
        Ok(())
    }

    /// Get node corresponding to node id
    pub fn get_node(&self, node_id: NodeId) -> Result<Node, GraphError> {
        match self.node_map.get(&node_id) {
            None => return Err(GraphError::NodeNotFoundError { id: node_id }),
            Some(node) => Ok(node.clone()),
        }
    }

    /// Get edge corresponding to edge id
    pub fn get_edge(&self, edge_id: EdgeId) -> Result<Edge, GraphError> {
        match self.edge_map.get(&edge_id) {
            None => return Err(GraphError::EdgeNotFoundError { id: edge_id }),
            Some(edge) => Ok(edge.clone()),
        }
    }

    // Attempt to remove node from graph
    /// 
    /// Only use this if node_id is confirmed to be a valid key
    fn unsafe_remove_node(&mut self, node_id: NodeId) {
        self.remove_edges_dependent_on_node(node_id);
        self.node_map.remove(&node_id);
    }

    /// Attempt to remove node from graph
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<(), GraphError> {
        self.verify_node(node_id)?;
        self.unsafe_remove_node(node_id);
        Ok(())
    }

    // Remove all edges which contain the given node.
    fn remove_edges_dependent_on_node(&mut self, node_id: NodeId) {
        for edge_id in self.edge_map.keys()
            .filter(|&&id| {
                match self.get_edge(id) {
                    Err(_) => false,
                    Ok(edge) => edge.contains_node(&node_id),
            }})
            .map(|edge_id| edge_id.clone())
            .collect::<Vec<_>>() {
            self.unsafe_remove_edge(edge_id);
        }
    }  

    /// Attempt to remove edge from graph
    /// 
    /// Only use this if edge_id is confirmed to be a valid key
    fn unsafe_remove_edge(&mut self, edge_id: EdgeId) {
        self.edge_map.remove(&edge_id);
    }

    /// Attempt to remove edge from graph
    pub fn remove_edge(&mut self, edge_id: EdgeId) -> Result<(), GraphError> {
        self.verify_edge(edge_id)?;
        self.unsafe_remove_edge(edge_id);
        Ok(())
    }
}

impl Graph {
    /// Verify node is well defined
    fn verify_node(&self, node_id: NodeId) -> Result<(), GraphError> {
        match self.get_node(node_id) {
            Err(e) => Err(e),
            Ok(_) => Ok(()),
        }
    }

    /// Verify edge is well defined
    fn verify_edge(&self, edge_id: EdgeId) -> Result<(), GraphError> {
        match self.get_edge(edge_id) {
            Err(e) => return Err(e),
            Ok(edge) => {
                let err_1 = self.verify_node(edge.first);
                let err_2 = self.verify_node(edge.second);
                if err_1.is_err() { return err_1; }
                if err_2.is_err() { return err_2; }
            },
        };
        Ok(())
    }

    /// Verify the entire graph is well defined
    /// i.e. all the edges exist and map to existing nodes
    fn verify_graph(&self) -> Result<(), GraphError> {
        for &node_id in self.node_map.keys().clone() {
            self.verify_node(node_id)?;
        }

        for &edge_id in self.edge_map.keys().clone() {
            self.verify_edge(edge_id)?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod graph_tests {
    use crate::graph::{Edge, EdgeId, Graph, Node, NodeId};

    #[test]
    fn test_new() {
        let graph = Graph::new();
        assert_eq!(graph.node_counter, NodeId(0));
        assert_eq!(graph.edge_counter, EdgeId(0));
        assert_eq!(graph.node_map.keys().len(), 0);
        assert_eq!(graph.edge_map.keys().len(), 0);
        assert!(graph.verify_graph().is_ok())
    }
    #[test]
    fn test_simple_graph() {
        let mut graph = Graph::new();
        // Let us add three nodes with edges between 0, 1 and 1, 2
        for _ in 0..3 {
            graph.add_node(Node{});
        }
        let id_0 = NodeId(0);
        let id_1 = NodeId(1);
        let id_2 = NodeId(2);
        let _ = graph.unsafe_add_edge(Edge{first: id_0, second: id_1});
        let _ = graph.unsafe_add_edge(Edge{first: id_1, second: id_2});

        assert_eq!(graph.node_counter, NodeId(3));
        assert_eq!(graph.edge_counter, EdgeId(2));

        assert!(graph.verify_graph().is_ok());
    }
    #[test]
    fn test_add_edge_with_invalid_nodes() {
        let mut graph = Graph::new();
        let edge = Edge{first: NodeId(0), second: NodeId(1)};
        graph.unsafe_add_edge(edge.clone());
        assert!(graph.verify_graph().is_err());
        assert!(graph.add_edge(edge).is_err());
    }
    #[test]
    fn test_add_new_node() {
        let mut graph = Graph::new();
        graph.add_new_node();
        assert_eq!(graph.node_counter, NodeId(1));
        assert_eq!(graph.edge_counter, EdgeId(0));
    }
    #[test]
    fn test_remove_node() {
        let mut graph = Graph::new();
        graph.add_new_node();
        assert!(graph.remove_node(NodeId(0)).is_ok());
    }
    #[test]
    fn test_remove_node_fail() {
        let mut graph = Graph::new();
        assert!(graph.remove_node(NodeId(0)).is_err());
    }
    #[test]
    fn test_remove_node_with_edges() {
        let mut graph = Graph::new();
        graph.add_new_node();
        graph.add_new_node();
        graph.add_edge(Edge { first: NodeId(0), second: NodeId(1) }).unwrap();
        assert!(graph.remove_node(NodeId(0)).is_ok());
        assert_eq!(graph.node_map.len(), 1);
        assert_eq!(graph.edge_map.len(), 0);
    }
    #[test]
    fn test_remove_edges_for_node_that_does_not_exist() {
        let mut graph = Graph::new();
        graph.add_new_node();
        graph.add_new_node();
        graph.add_edge(Edge { first: NodeId(0), second: NodeId(1) }).unwrap();
        graph.remove_edges_dependent_on_node(NodeId(2));
        assert_eq!(graph.node_map.len(), 2);
        assert_eq!(graph.edge_map.len(), 1);
    }
    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::new();
        graph.add_new_node();
        graph.add_new_node();
        graph.add_edge(Edge { first: NodeId(0), second: NodeId(1) }).unwrap();
        assert!(graph.remove_edge(EdgeId(0)).is_ok());
        assert_eq!(graph.node_map.len(), 2);
        assert_eq!(graph.edge_map.len(), 0);
    }
}