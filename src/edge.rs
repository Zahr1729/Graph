use std::{collections::HashMap, fmt};

use crate::{error::GraphError, node::NodeId};


#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, PartialOrd, Ord)]
pub struct EdgeId(pub usize);


impl fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl EdgeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Anything that behaves like a node, which is everything.
pub trait Edge: Default + Clone {
    fn contains_node(&self, node_id: &NodeId) -> bool;
    fn first(&self) -> &NodeId;
    fn second(&self) -> &NodeId;
    fn first_mut(&mut self) -> &NodeId;
    fn second_mut(&mut self) -> &NodeId;
}

/// An Edge at minimum needs a reference to two nodes.
#[derive(Clone, Default)]
pub struct BasicEdge {
    first: NodeId,
    second: NodeId,
}

impl Edge for BasicEdge {
    fn contains_node(&self, node_id: &NodeId) -> bool {
        return node_id == &self.first || node_id == &self.second
    }
    
    fn first(&self) -> &NodeId {
        &self.first
    }
    
    fn second(&self) -> &NodeId {
        &self.second
    }
    
    fn first_mut(&mut self) -> &NodeId {
        &self.first
    }
    
    fn second_mut(&mut self) -> &NodeId {
        &self.second
    }
}

impl fmt::Debug for BasicEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.first, self.second)?;
        Ok(())
    }
}

/// Structure to deal with storing nodes in a graph.
pub struct EdgeMap<T: Edge>{
    edge_counter: usize,
    edge_map: HashMap<EdgeId, T>,
}



impl<T: Edge> EdgeMap<T> {
    pub fn new() -> Self {
        Self {
            edge_counter: 0,
            edge_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.edge_map.len()
    }

    /// Add T
    pub fn add_edge(&mut self, edge: T) ->  EdgeId {
        let return_value = self.edge_counter;
        self.edge_map.insert(EdgeId(self.edge_counter), edge);
        self.edge_counter += 1;
        EdgeId(return_value)
    }

    /// Add new edge
    pub fn add_default_edge(&mut self) -> EdgeId {
        let new_edge = T::default();
        self.add_edge(new_edge)
    }

    /// Get edge corresponding to edge id
    pub fn get_edge(&self, edge_id: &EdgeId) -> Result<&T, GraphError> {
        match self.edge_map.get(edge_id) {
            None => return Err(GraphError::EdgeNotFoundError { id: *edge_id }),
            Some(edge) => Ok(&edge),
        }
    }

    // Attempt to remove edge from graph
    /// 
    /// Only use this if edge_id is confirmed to be a valid key
    fn unsafe_remove_edge(&mut self, edge_id: &EdgeId) {
        self.edge_map.remove(edge_id);
    }

    /// Attempt to remove edge from graph
    pub fn remove_edge(&mut self, edge_id: &EdgeId) -> Result<(), GraphError> {
        self.verify_edge(edge_id)?;
        self.unsafe_remove_edge(edge_id);
        Ok(())
    }

    /// Insert edge
    /// 
    /// If the key already exists
    pub fn insert_edge(&mut self, edge_id: EdgeId, edge: T) -> Option<T> {
        self.edge_map.insert(edge_id, edge)
    }

    /// Verify edge is well defined
    fn verify_edge(&self, edge_id: &EdgeId) -> Result<(), GraphError> {
        if self.edge_map.contains_key(edge_id) { Ok(()) }
        else { Err(GraphError::EdgeNotFoundError { id: *edge_id })}
    }

    /// Verify edge is well defined
    fn verify(&self) -> Result<(), GraphError> {
        for edge_id in self.edge_map.keys() {
            self.verify_edge(edge_id)?;
        }
        Ok(())
    }
}

impl<T: Edge> fmt::Debug for EdgeMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut edge_vec = self.edge_map.keys().into_iter().collect::<Vec<_>>();
        edge_vec.sort();
        write!(f, "Edges: {:?}", &edge_vec)?;
        Ok(())
    }
}


#[cfg(test)]
mod node_tests {
    use crate::node::{BasicEdge, Edge, EdgeId, EdgeMap};

    
}