use crate::{coords::Coords, graph::gate::Gate};

use raylib::prelude::*;

pub struct Node {
    pub gate: Gate,
    pub coords: Coords,
}

impl Node {
    pub fn new(gate: &Gate, coords: &Coords) -> Self {
        Self {
            gate: gate.clone(),
            coords: coords.clone(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.gate.draw(d, &self.coords, Color::GRAY);
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node(gate: {}, coords: {})", self.gate, self.coords,)
    }
}
