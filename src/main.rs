mod core;
mod utils;
mod graphs;
mod algorithms;

use std::path::Path;
use crate::{core::{edge::{Edge, EdgeMap}, graph::Graph, node::{Node, NodeId, NodeMap}}, graphs::{basic::{BasicEdge, BasicNode}, coords::{CoordNode, Coords2d}, weights::WeightedEdge}};
use crate::utils::saveload::{load_from_json, save_to_json};

// Duplicate code to testing.
fn get_example_graph<N: Node + Coords2d, E: Edge>() -> Graph<N, E> {
    let mut graph = Graph { node_map: NodeMap::<N>::new(), edge_map: EdgeMap::<E>::new() };
    let mut rng = rng();
    for _ in 0..7 {
        let (x, y) = (rng.random_range(50.0..700.0), rng.random_range(50.0..500.0));
        let mut node = N::new();
        node.set_coords(x, y);
        graph.add_node(node);
    }
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



use macroquad::prelude::*;
use ::rand::{RngExt, rng};

#[macroquad::main("MyGame")]
async fn main() {
    // let g = get_example_graph::<CoordNode, BasicEdge>();
    // let _ = save_to_json(Path::new("./data/graph.json"), &g);
    let g: Graph<CoordNode, BasicEdge> = load_from_json(Path::new("./data/graph.json")).unwrap();
    // println!("{:?}", g);
    loop {
        // clear_background(BLACK);
        draw_grid(20, 30.0, WHITE, WHITE);

        for edge_id in g.edges() {
            let e = g.get_edge(edge_id).unwrap();
            let (first, second) = e.get_nodes();
            let (f, s) = (g.get_node(first).unwrap(), g.get_node(second).unwrap());
            let ((x1, y1), (x2, y2)) = (f.get_coords(), s.get_coords());
            draw_line(x1, y1, x2, y2, 3.0, GRAY);
        }

        for node_id in g.nodes() {
            let n = g.get_node(node_id).unwrap();
            draw_circle(n.get_x(), n.get_y(), 23.0, GRAY);
            draw_circle(n.get_x(), n.get_y(), 20.0, RED);
            
        }
        next_frame().await
    }
}
