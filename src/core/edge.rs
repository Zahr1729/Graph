use std::{collections::HashMap, fmt::{self, Debug}};

use serde::{Deserialize, Serialize};

use crate::core::{error::GraphError, node::{NodeId}};


#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EdgeId(pub usize);


impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

/// Anything that behaves like a node, which is everything.
pub trait Edge: Default + Ord + Debug {
    fn contains_node(&self, node_id: &NodeId) -> bool;
    fn get_first(&self) -> &NodeId;
    fn get_second(&self) -> &NodeId;
    fn set_first(&mut self, node_id: NodeId);
    fn set_second(&mut self, node_id: NodeId);
    fn get_nodes(&self) -> (&NodeId, &NodeId) {
        (self.get_first(), self.get_second())
    }
    fn set_nodes(&mut self, first: NodeId, second: NodeId) {
        self.set_first(first);
        self.set_second(second);
    }
    fn new(first: NodeId, second: NodeId) -> Self {
        let mut edge = Self::default();
        edge.set_nodes(first, second);
        edge
    }
    fn dbg(&self) -> impl Debug;
}


/// Structure to deal with storing nodes in a graph.
#[derive(Serialize, Deserialize)]
pub struct EdgeMap<E: Edge>{
    edge_counter: usize,
    pub edge_map: HashMap<EdgeId, E>,
}



impl<E: Edge> EdgeMap<E> {
    pub fn new() -> Self {
        Self {
            edge_counter: 0,
            edge_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.edge_map.len()
    }

    /// Add edge without checking if such an edge is well definied.
    pub fn add(&mut self, edge: E) -> EdgeId {
        let return_value = self.edge_counter;
        self.edge_map.insert(EdgeId(self.edge_counter), edge);
        self.edge_counter += 1;
        EdgeId(return_value)
    }

    /// Add edge from_nodes
    pub fn add_from_nodes(&mut self, first: NodeId, second: NodeId) -> EdgeId {
        let mut edge = E::default();
        edge.set_first(first);
        edge.set_second(second);
        self.add(edge)
    }

    /// Get edge corresponding to edge id
    pub fn get(&self, edge_id: &EdgeId) -> Result<&E, GraphError> {
        match self.edge_map.get(edge_id) {
            None => return Err(GraphError::EdgeNotFoundError { id: edge_id.clone() }),
            Some(edge) => Ok(&edge),
        }
    }

    /// Attempt to remove edge from graph
    pub fn remove(&mut self, edge_id: &EdgeId) -> Option<E> {
        self.edge_map.remove(edge_id)
    }

    /// Remove all edges starting or stopping at given node
    pub fn remove_edges_dependent_on_node(&mut self, node_id: &NodeId) {
        for edge_id in self.edge_map.keys()
            .filter(|&id| {
                match self.get(id) {
                    Err(_) => false,
                    Ok(edge) => edge.contains_node(&node_id),
            }})
            .map(|edge_id| edge_id.clone())
            .collect::<Vec<_>>() {
            // Can be unsafe given we start from the keys.
            self.remove(&edge_id);
        }
    } 

    /// Insert edge
    pub fn insert(&mut self, edge_id: EdgeId, edge: E) -> Option<E> {
        self.edge_map.insert(edge_id, edge)
    }

    /// Verify edge is well defined
    pub fn verify(&self, edge_id: &EdgeId) -> Result<(), GraphError> {
        if self.edge_map.contains_key(edge_id) { Ok(()) }
        else { Err(GraphError::EdgeNotFoundError { id: edge_id.clone() })}
    }
}

impl<E: Edge> fmt::Debug for EdgeMap<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut edge_vec = self.edge_map.values().into_iter().collect::<Vec<_>>();
        edge_vec.sort();
        write!(f, "Edges: {:?}", &edge_vec)?;
        Ok(())
    }
}


pub(crate) mod edge_tests {
    use crate::core::edge::{Edge, EdgeId, EdgeMap, NodeId};

    pub fn get_3_default_edge_map<E: Edge>() -> EdgeMap<E> {
        let mut node_map = EdgeMap::<E>::new();
        for _ in 0..3 {
            node_map.add(E::default());
        }
        return node_map;
    }

    pub fn get_example_default_edge_map<E: Edge>() -> EdgeMap<E> {
        let mut node_map = EdgeMap::<E>::new();
        for _ in 0..5 {
            node_map.add(E::default());
        }
        node_map.remove(&EdgeId(2)).unwrap();
        node_map.remove(&EdgeId(3)).unwrap();
        node_map
    }

    pub fn get_example_non_default_edge_map<E: Edge>() -> EdgeMap<E> {
        let mut node_map = EdgeMap::<E>::new();
        node_map.add_from_nodes(NodeId(0), NodeId(0));
        node_map.add_from_nodes(NodeId(0), NodeId(1));
        node_map.add_from_nodes(NodeId(2), NodeId(1));
        node_map.add_from_nodes(NodeId(2), NodeId(4));
        node_map.add_from_nodes(NodeId(4), NodeId(0));
        node_map.add_from_nodes(NodeId(4), NodeId(5));
        node_map
    }

    pub fn test_add_helper<E: Edge>() {
        let mut edge_map = EdgeMap::<E>::new();
        let id0 = edge_map.add(E::default());
        assert_eq!(id0, EdgeId(0));
        let edge_1 = E::default();
        let id1 = edge_map.add(edge_1);
        assert_eq!(id1, EdgeId(1));
        assert_eq!(edge_map.len(), 2);

        assert!(edge_map.verify(&EdgeId(0)).is_ok());
        assert!(edge_map.verify(&EdgeId(1)).is_ok());
        assert!(edge_map.verify(&EdgeId(2)).is_err());
    }

    pub fn test_add_from_nodes_helper<E: Edge>() {
        let mut edge_map = EdgeMap::<E>::new();
        // Add the same node multiple times
        let id0 = edge_map.add_from_nodes(NodeId(0), NodeId(2));
        let id1 = edge_map.add_from_nodes(NodeId(2), NodeId(1));
        let id2 = edge_map.add_from_nodes(NodeId(0), NodeId(2));
        assert_eq!(id0, EdgeId(0));
        assert_eq!(id1, EdgeId(1));
        assert_eq!(id2, EdgeId(2));
        assert_eq!(edge_map.len(), 3);
        
        assert!(edge_map.verify(&EdgeId(0)).is_ok());
        assert!(edge_map.verify(&EdgeId(1)).is_ok());
        assert!(edge_map.verify(&EdgeId(2)).is_ok());
        assert!(edge_map.verify(&EdgeId(3)).is_err());
    }

    pub fn test_get_helper<E: Edge>() {
        let edge_map = get_3_default_edge_map::<E>();
        assert!(edge_map.get(&EdgeId(1)).is_ok());
        assert!(edge_map.get(&EdgeId(3)).is_err());
    }

    pub fn test_remove_helper<E: Edge>() {
        let mut edge_map = get_3_default_edge_map::<E>();
        assert!(edge_map.remove(&EdgeId(3)).is_none());
        assert!(edge_map.remove(&EdgeId(2)).is_some());
        assert_eq!(edge_map.len(), 2);
        // Try to remove 2 again without checks.
        edge_map.remove(&EdgeId(2));
        assert_eq!(edge_map.len(), 2);
        // remove 0 without checks
        edge_map.remove(&EdgeId(0));
        assert_eq!(edge_map.len(), 1);
        
        assert!(edge_map.verify(&EdgeId(0)).is_err());
        assert!(edge_map.verify(&EdgeId(2)).is_err());
        assert!(edge_map.verify(&EdgeId(1)).is_ok());
    }

    pub fn test_remove_edges_dependent_on_node_helper<E: Edge>() {
        let mut edge_map = get_example_non_default_edge_map::<E>();
        assert_eq!(edge_map.len(), 6);
        edge_map.remove_edges_dependent_on_node(&NodeId(3));
        assert_eq!(edge_map.len(), 6);
        edge_map.remove_edges_dependent_on_node(&NodeId(1 ));
        assert_eq!(edge_map.len(), 4);

        assert!(edge_map.verify(&EdgeId(1)).is_err());
        assert!(edge_map.verify(&EdgeId(2)).is_err());
    }

    pub fn test_insert_helper<E: Edge>() {
        let mut edge_map = get_example_default_edge_map::<E>();
        let ret2 = edge_map.insert(EdgeId(2), E::default());
        assert!(ret2.is_none());
        assert_eq!(edge_map.len(), 4);
        let ret0 = edge_map.insert(EdgeId(0), E::default());
        assert!(ret0.is_some());
        assert_eq!(edge_map.len(), 4);
    }
}