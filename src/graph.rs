use crate::coords::Coords;
use crate::node::{Gate, Node};

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    // Returns true on success
    pub fn add_node(&mut self, gate: &Gate, coords: &Coords) {
        self.nodes.push(Node::new(gate, coords));
    }
}
