mod core;
mod utils;
mod graphs;

use std::path::Path;

use crate::{core::{edge::{Edge, EdgeMap}, graph::Graph, node::{Node, NodeId, NodeMap}}, graphs::basic::{BasicEdge, BasicNode}};
use crate::utils::saveload::{load_from_json, save_to_json};

// Duplicate code to testing.
fn get_example_graph<N: Node, E: Edge>() -> Graph<N, E> {
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

pub fn main() {
    // let g = get_example_graph::<BasicNode, BasicEdge>();
    // let _ = save_to_json(Path::new("./data/graph.json"), &g);
    let g: Graph<BasicNode, BasicEdge> = load_from_json(Path::new("./data/graph.json")).unwrap();
    println!("{:?}", g);
}