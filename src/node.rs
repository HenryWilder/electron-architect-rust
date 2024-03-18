use crate::coords::Coords;
use crate::wire::Wire;

pub enum Gate {
    Never,
    Always,
    Random,
    Buffer,
    And,
    Or,
    Nor,
    Xor,
}

pub struct Node {
    pub gate: Gate,
    pub coords: Coords,
    inputs: Vec<Wire>,
}

impl Node {
    pub fn new(gate: Gate, coords: Coords) -> Self {
        Self {
            gate,
            coords,
            inputs: Vec::new(),
        }
    }
}
