use std::fmt::{self, Debug};

use serde::{Serialize, Deserialize};

use crate::core::{edge::Edge, node::{Node, NodeId}};


pub trait Coords2d {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn get_coords(&self)  -> (f32, f32) { (self.get_x(), self.get_y()) }
    fn set_coords(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CoordNode {
    x: f32,
    y: f32,
}

impl Node for CoordNode {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Coords2d for CoordNode {
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
    fn set_x(&mut self, x: f32) { self.x = x }
    fn set_y(&mut self, y: f32) { self.y = y }
}

#[cfg(test)]
mod location_node_tests {
    use crate::core::graph::graph_tests::test_serde_helper;
    use crate::graphs::coords::CoordNode;
    use crate::core::node::node_tests::*;
    use crate::graphs::weights::WeightedEdge;

    #[test]
    fn test_add() {
        test_add_helper::<CoordNode>();
    }

    #[test]
    fn test_get() {
        test_get_helper::<CoordNode>();
    }
    #[test]
    fn test_remove() {
        test_remove_helper::<CoordNode>();
    }

    #[test]
    fn test_insert() {
        test_insert_helper::<CoordNode>();
    }

    #[test]
    fn test_debug() {
        let node_map = get_example_node_map::<CoordNode>();
        assert_eq!(format!("{node_map:?}"), "Nodes: [0, 1, 4]")
    }

    #[test]
    fn test_serde() {
        test_serde_helper::<CoordNode, WeightedEdge>("./tests/data/save_load/coords.json");
    }
}