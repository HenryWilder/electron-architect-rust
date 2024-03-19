use crate::coords::Coords;
use crate::wire::{Elbow, Wire};

#[derive(Clone)]
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

impl Gate {
    pub fn next(&self) -> Self {
        use Gate::*;
        match *self {
            Never => Always,
            Always => Random,
            Random => Buffer,
            Buffer => And,
            And => Or,
            Or => Nor,
            Nor => Xor,
            Xor => Never,
        }
    }

    pub fn prev(&self) -> Self {
        use Gate::*;
        match *self {
            Never => Xor,
            Always => Never,
            Random => Always,
            Buffer => Random,
            And => Buffer,
            Or => And,
            Nor => Or,
            Xor => Nor,
        }
    }
}

pub struct Node {
    pub gate: Gate,
    pub coords: Coords,
    inputs: Vec<Wire>,
}

impl Node {
    pub fn new(gate: &Gate, coords: &Coords) -> Self {
        Self {
            gate: gate.clone(),
            coords: coords.clone(),
            inputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, input: *mut Node, elbow: Elbow) {
        let wire = Wire::new(input, self, elbow);
        self.inputs.push(wire);
    }

    pub fn num_input(&self) -> usize {
        self.inputs.len()
    }
}
