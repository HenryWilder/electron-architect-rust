pub mod elbow;
pub mod gate;
pub mod node;
pub mod wire;

use raylib::prelude::*;

use crate::coords::Coords;
use crate::graph::{gate::Gate, node::Node, wire::Wire};

pub struct Graph {
    nodes: Vec<Node>,
    wires: Vec<Wire>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            wires: Vec::new(),
        }
    }

    // Returns true on success
    pub fn add_node(&mut self, gate: &Gate, coords: &Coords) {
        self.nodes.push(Node::new(gate, coords));
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for node in &self.nodes {
            node.draw(d);
        }
    }

    pub fn find_node_at_coords(&self, search_coords: &Coords) -> Option<&Node> {
        for node in &self.nodes {
            if node.coords == *search_coords {
                return Some(&node);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn find_wire_intersecting_coords(&self, search_coords: &Coords) -> Option<&Wire> {
        for wire in &self.wires {
            if wire.is_intersecting_coords(search_coords) {
                return Some(&wire);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph = Graph::new();
        assert_eq!(graph.nodes.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();

        const TEST_GATE: Gate = Gate::G1(gate::Gate1::Buffer);
        const TEST_COORDS: Coords = Coords { x: 0, y: 0 };

        graph.add_node(&TEST_GATE, &TEST_COORDS);
        assert_eq!(graph.nodes.len(), 1);
        // Todo if ever they are made PartiallyEq
        // assert_eq!(graph.nodes.last().unwrap().gate, TEST_GATE);
        // assert_eq!(graph.nodes.last().unwrap().coords, TEST_COORDS);
    }
}
