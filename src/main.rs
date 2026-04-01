mod graph;
mod error;

use graph::Graph;

use crate::graph::NodeId;

pub fn main() {
    let mut g = Graph::new();
    for _ in 0..3 {
        g.add_new_node();
    }
    let _ = g.add_edge_from_nodes(NodeId(0), NodeId(1));
    let _ = g.add_edge_from_nodes(NodeId(0), NodeId(0));
    let _ = g.add_edge_from_nodes(NodeId(2), NodeId(0));
    println!("{:?}", g);
}