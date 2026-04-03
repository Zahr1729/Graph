use std::{collections::HashMap, fmt};

use crate::error::GraphError;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, PartialOrd, Ord)]
pub struct NodeId(pub usize);

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl NodeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Anything that behaves like a node, which is everything.
pub trait Node: Default + Clone {}

#[derive(Default, Clone)]
pub struct BasicNode();

impl Node for BasicNode {}

/// Structure to deal with storing nodes in a graph.
pub struct NodeMap<Node>{
    node_counter: usize,
    node_map: HashMap<NodeId, Node>,
}

impl<T: Node> NodeMap<T> {
    pub fn new() -> Self {
        Self {
            node_counter: 0,
            node_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.node_map.len()
    }

    /// Add T
    pub fn add_node(&mut self, node: T) ->  NodeId {
        let return_value = self.node_counter;
        self.node_map.insert(NodeId(self.node_counter), node);
        self.node_counter += 1;
        NodeId(return_value)
    }

    /// Add new node
    pub fn add_default_node(&mut self) -> NodeId {
        let new_node = T::default();
        self.add_node(new_node)
    }

    /// Get node corresponding to node id
    pub fn get_node(&self, node_id: &NodeId) -> Result<&T, GraphError> {
        match self.node_map.get(node_id) {
            None => return Err(GraphError::NodeNotFoundError { id: *node_id }),
            Some(node) => Ok(&node),
        }
    }

    // Attempt to remove node from graph
    /// 
    /// Only use this if node_id is confirmed to be a valid key
    fn unsafe_remove_node(&mut self, node_id: &NodeId) {
        self.node_map.remove(node_id);
    }

    /// Attempt to remove node from graph
    pub fn remove_node(&mut self, node_id: &NodeId) -> Result<(), GraphError> {
        self.verify_node(node_id)?;
        self.unsafe_remove_node(node_id);
        Ok(())
    }

    /// Insert node
    /// 
    /// If the key already exists
    pub fn insert_node(&mut self, node_id: NodeId, node: T) -> Option<T> {
        self.node_map.insert(node_id, node)
    }

    /// Verify node is well defined
    fn verify_node(&self, node_id: &NodeId) -> Result<(), GraphError> {
        if self.node_map.contains_key(node_id) { Ok(()) }
        else { Err(GraphError::NodeNotFoundError { id: *node_id })}
    }

    /// Verify node is well defined
    fn verify(&self) -> Result<(), GraphError> {
        for node_id in self.node_map.keys() {
            self.verify_node(node_id)?;
        }
        Ok(())
    }
}

impl<T: Node> fmt::Debug for NodeMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut node_vec = self.node_map.keys().into_iter().collect::<Vec<_>>();
        node_vec.sort();
        write!(f, "Nodes: {:?}", &node_vec)?;
        Ok(())
    }
}

#[cfg(test)]
mod node_tests {
    use crate::node::{BasicNode, Node, NodeId, NodeMap};

    fn get_3_node_map<T: Node>() -> NodeMap<T> {
        let mut node_map = NodeMap::<T>::new();
        for _ in 0..3 {
            node_map.add_default_node();
        }
        return node_map;
    }

    fn get_example_node_map<T: Node>() -> NodeMap<T> {
        let mut node_map = NodeMap::<T>::new();
        for _ in 0..5 {
            node_map.add_default_node();
        }
        node_map.remove_node(&NodeId(2)).unwrap();
        node_map.remove_node(&NodeId(3)).unwrap();
        return node_map;
    }

    fn test_add_node_helper<T: Node>() {
        let mut node_map = NodeMap::<T>::new();
        let id0 = node_map.add_default_node();
        assert_eq!(id0, NodeId(0));
        let node_1 = T::default();
        let id1 = node_map.add_node(node_1);
        assert_eq!(id1, NodeId(1));
        assert_eq!(node_map.len(), 2);
        assert!(node_map.verify().is_ok());
    }

    fn test_get_node_helper<T: Node>() {
        let node_map = get_3_node_map::<T>();
        assert!(node_map.get_node(&NodeId(1)).is_ok());
        assert!(node_map.get_node(&NodeId(3)).is_err());
        assert!(node_map.verify().is_ok());
    }

    fn test_remove_node_helper<T: Node>() {
        let mut node_map = get_3_node_map::<T>();
        assert!(node_map.remove_node(&NodeId(3)).is_err());
        assert!(node_map.remove_node(&NodeId(2)).is_ok());
        assert_eq!(node_map.len(), 2);
        // Try to remove 2 again without checks.
        node_map.unsafe_remove_node(&NodeId(2));
        assert_eq!(node_map.len(), 2);
        // remove 0 without checks
        node_map.unsafe_remove_node(&NodeId(0));
        assert_eq!(node_map.len(), 1);
        assert!(node_map.verify().is_ok());
    }

    fn test_insert_node_helper<T: Node>() {
        let mut node_map = get_example_node_map::<T>();
        let ret2 = node_map.insert_node(NodeId(2), T::default());
        assert!(ret2.is_none());
        assert_eq!(node_map.len(), 4);
        let ret0 = node_map.insert_node(NodeId(0), T::default());
        assert!(ret0.is_some());
        assert_eq!(node_map.len(), 4);
    }

    fn test_debug_helper<T: Node>() {
        let node_map = get_example_node_map::<T>();
        assert_eq!(format!("{node_map:?}"), "Nodes: [0, 1, 4]")
    }

    mod basic_node_tests {
        use super::*;

        #[test]
        fn test_add_node() {
            test_add_node_helper::<BasicNode>();
        }

        #[test]
        fn test_get_node() {
            test_get_node_helper::<BasicNode>();
        }
        #[test]
        fn test_remove_node() {
            test_remove_node_helper::<BasicNode>();
        }

        #[test]
        fn test_insert_node() {
            test_insert_node_helper::<BasicNode>();
        }

        #[test]
        fn test_debug() {
            test_debug_helper::<BasicNode>();
        }
    }
}