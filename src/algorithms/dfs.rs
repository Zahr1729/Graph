use std::collections::{HashMap, HashSet};

use crate::core::{edge::{Edge, EdgeId}, error::GraphError, graph::Graph, node::{Node, NodeId}};

impl<N: Node, E: Edge> Graph<N,E> {
    /// Returns true if dfs was successful and false is not. writes appropriate path and visits between nodes.
    fn hidden_dfs<'a>(&'a self, start: &NodeId, end: &'a NodeId,
        visited_nodes: &mut HashSet<&'a NodeId>, path_stack: &mut Vec<(&'a NodeId, &'a EdgeId)>) -> bool {
            if start == end { return true; }

            let mut neighbors = self.get_directed_neighbors(start);
            neighbors.sort();

            for (edge, neighbor) in neighbors {
                if visited_nodes.contains(neighbor) { continue; }
                visited_nodes.insert(neighbor);

                // Add to the path stack
                path_stack.push((neighbor, edge));

                // Recurse
                let res = self.hidden_dfs(neighbor, end, visited_nodes, path_stack);
                if res { return res; }

                // Remove from path stack
                path_stack.pop();
            }

            return false;
        }

    pub fn dfs<'a>(&'a self, start: &'a NodeId, end: &'a NodeId) -> Option<(Vec<(&'a NodeId, &'a EdgeId)>, HashSet<&'a NodeId>)> {
        // Check if they actually exist
        let Ok(_) = self.verify_node(start) else { return None; };
        let Ok(_) = self.verify_node(end) else { return None; };

        // Setup dfs
        let mut visited_nodes = HashSet::with_capacity(self.node_map.len());
        let mut path_stack = Vec::with_capacity(self.node_map.len());
        visited_nodes.insert(start);

        // dfs
        match self.hidden_dfs(&start, end, &mut visited_nodes, &mut path_stack) {
            true => Some((path_stack, visited_nodes)),
            false => None,
        }
        
        
    }
}

#[cfg(test)]
mod test_dfs {
    use crate::{core::{edge::{Edge, EdgeId}, graph::{Graph, graph_tests::get_example_graph}, node::{Node, NodeId}}, graphs::basic::{BasicEdge, BasicNode}};

    #[test]
    fn test_dfs_success() {
        let graph = get_example_graph::<BasicNode, BasicEdge>();
        let Some((path, nodes)) = graph.dfs(&NodeId(0), &NodeId(2)) else { panic!(); };
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], (&NodeId(2), &EdgeId(2)));
        assert_eq!(nodes.len(), 5);
        assert!(nodes.contains(&NodeId(0)));
        // Wrong path.
        assert!(nodes.contains(&NodeId(1)));
        assert!(nodes.contains(&NodeId(4)));
        assert!(nodes.contains(&NodeId(6)));
        // Correct path.
        assert!(nodes.contains(&NodeId(2)));
    }

    #[test]
    fn test_simple_dfs_with_failures() {
        let mut graph = Graph::<BasicNode, BasicEdge>::new();

        // Try dfs on emtpy graph
        assert!(graph.dfs(&NodeId(0), &NodeId(1)).is_none());

        // Try dfs on only nodes
        graph.insert_node(NodeId(0), BasicNode::new());
        graph.insert_node(NodeId(1), BasicNode::new());
        assert!(graph.dfs(&NodeId(0), &NodeId(1)).is_none());

        // Try dfs on connected nodes, but wrongly oriented
        graph.insert_edge(EdgeId(0), BasicEdge::new(NodeId(1), NodeId(0)));
        assert!(graph.dfs(&NodeId(0), &NodeId(1)).is_none());

        // Try dfs on properly corrected edge.
        graph.insert_edge(EdgeId(1), BasicEdge::new(NodeId(0), NodeId(1)));
        let Some((path, nodes)) = graph.dfs(&NodeId(0), &NodeId(1)) else { panic!() };

        assert_eq!(path.len(), 1);
        assert_eq!(path[0], (&NodeId(1), &EdgeId(1)));
    }
}