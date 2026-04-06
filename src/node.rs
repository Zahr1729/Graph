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

/// Anything that behaves like a node, which is everything.
pub trait Node {
    fn new() -> Self;
}

#[derive(Default, Debug)]
pub struct BasicNode();

impl Node for BasicNode {
    fn new() -> Self {
        Self()
    }
}

/// Structure to deal with storing nodes in a graph.
pub struct NodeMap<Node>{
    node_counter: usize,
    pub node_map: HashMap<NodeId, Node>,
}

impl<N: Node> NodeMap<N> {
    pub fn new() -> Self {
        Self {
            node_counter: 0,
            node_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.node_map.len()
    }

    /// Add node
    pub fn add(&mut self, node: N) ->  NodeId {
        let return_value = self.node_counter;
        self.node_map.insert(NodeId(self.node_counter), node);
        self.node_counter += 1;
        NodeId(return_value)
    }

    /// Get node corresponding to node id
    pub fn get(&self, node_id: &NodeId) -> Result<&N, GraphError> {
        match self.node_map.get(node_id) {
            None => return Err(GraphError::NodeNotFoundError { id: node_id.clone() }),
            Some(node) => Ok(&node),
        }
    }

    /// Attempt to remove node from graph
    pub fn remove(&mut self, node_id: &NodeId) -> Option<N> {
        self.node_map.remove(node_id)
    }

    /// Insert node
    pub fn insert(&mut self, node_id: NodeId, node: N) -> Option<N> {
        self.node_map.insert(node_id, node)
    }

    /// Verify node is well defined
    pub fn verify_node(&self, node_id: &NodeId) -> Result<(), GraphError> {
        if self.node_map.contains_key(node_id) { Ok(()) }
        else { Err(GraphError::NodeNotFoundError { id: node_id.clone() })}
    }
}

impl<N: Node> fmt::Debug for NodeMap<N> {
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

    fn get_3_node_map<N: Node + Default>() -> NodeMap<N> {
        let mut node_map = NodeMap::<N>::new();
        for _ in 0..3 {
            node_map.add(N::default());
        }
        return node_map;
    }

    fn get_example_node_map<N: Node + Default>() -> NodeMap<N> {
        let mut node_map = NodeMap::<N>::new();
        for _ in 0..5 {
            node_map.add(N::default());
        }
        node_map.remove(&NodeId(2)).unwrap();
        node_map.remove(&NodeId(3)).unwrap();
        return node_map;
    }

    fn test_add_helper<N: Node + Default>() {
        let mut node_map = NodeMap::<N>::new();

        let id0 = node_map.add(N::default());
        assert_eq!(id0, NodeId(0));
        let node_1 = N::default();
        let id1 = node_map.add(node_1);
        assert_eq!(id1, NodeId(1));
        assert_eq!(node_map.len(), 2);
        assert!(node_map.verify_node(&NodeId(0)).is_ok());
        assert!(node_map.verify_node(&NodeId(1)).is_ok());
        assert!(node_map.verify_node(&NodeId(2)).is_err());
    }

    fn test_get_helper<N: Node + Default>() {
        let node_map = get_3_node_map::<N>();
        assert!(node_map.get(&NodeId(1)).is_ok());
        assert!(node_map.get(&NodeId(3)).is_err());
    }

    fn test_remove_helper<N: Node + Default>() {
        let mut node_map = get_3_node_map::<N>();
        assert!(node_map.remove(&NodeId(3)).is_none());
        assert!(node_map.remove(&NodeId(2)).is_some());
        assert_eq!(node_map.len(), 2);
        // Try to remove 2 again without checks.
        node_map.remove(&NodeId(2));
        assert_eq!(node_map.len(), 2);
        // remove 0 without checks
        node_map.remove(&NodeId(0));
        assert_eq!(node_map.len(), 1);
        assert!(node_map.verify_node(&NodeId(0)).is_err());
        assert!(node_map.verify_node(&NodeId(2)).is_err());
        assert!(node_map.verify_node(&NodeId(1)).is_ok());
    }

    fn test_insert_helper<N: Node + Default>() {
        let mut node_map = get_example_node_map::<N>();
        let ret2 = node_map.insert(NodeId(2), N::default());
        assert!(ret2.is_none());
        assert_eq!(node_map.len(), 4);
        let ret0 = node_map.insert(NodeId(0), N::default());
        assert!(ret0.is_some());
        assert_eq!(node_map.len(), 4);
    }

    fn test_debug_helper<N: Node + Default>() {
        let node_map = get_example_node_map::<N>();
        assert_eq!(format!("{node_map:?}"), "Nodes: [0, 1, 4]")
    }

    mod basic_node_tests {
        use super::*;

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
}