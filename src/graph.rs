pub mod elbow;
pub mod gate;
pub mod node;
pub mod wire;

use raylib::prelude::*;

use crate::coords::Coords;
use crate::graph::{elbow::Elbow, gate::Gate, node::Node, wire::Wire};

pub struct Graph<'g> {
    nodes: Vec<Node>,
    wires: Vec<Wire<'g>>,
}

impl<'g> Graph<'g> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            wires: Vec::new(),
        }
    }

    // Returns true on success
    pub fn add_node(&mut self, gate: &Gate, coords: &Coords) -> &'g Node {
        self.nodes.push(Node::new(gate, coords));
        self.nodes.last().unwrap() // If we didn't make a new node, something is wrong and we should panic.
    }

    pub fn add_wire(&mut self, src: &'g Node, dest: &'g Node, elbow: &Elbow) -> &'g Wire {
        self.wires.push(Wire::new(src, dest, elbow));
        self.wires.last().unwrap()
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for node in &self.nodes {
            node.draw(d);
        }
    }

    pub fn find_node_at_coords(&self, search_coords: &Coords) -> Option<&'g Node> {
        self.nodes
            .iter()
            .find(|&node| node.coords == *search_coords)
    }

    #[allow(dead_code)]
    pub fn find_wire_intersecting_coords(&self, search_coords: &Coords) -> Option<&'g Wire> {
        self.wires
            .iter()
            .find(|&wire| wire.is_intersecting_coords(search_coords))
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
