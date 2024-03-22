use raylib::prelude::*;

use crate::{common_traits::Scrollable, coords::Coords};
use std::fmt::{self, Formatter};

pub trait GateIn {
    const MAX_INPUTS: usize;
}

#[derive(Clone)]
pub enum Gate0 {
    Never,
    Always,
    Random,
}

impl Scrollable for Gate0 {
    fn next(&self) -> Self {
        use Gate0::*;
        match *self {
            Never => Always,
            Always => Random,
            Random => Never,
        }
    }

    fn prev(&self) -> Self {
        use Gate0::*;
        match *self {
            Never => Random,
            Always => Never,
            Random => Always,
        }
    }
}

impl GateIn for Gate0 {
    const MAX_INPUTS: usize = 0;
}

impl fmt::Display for Gate0 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Gate0::*;
        write!(
            f,
            "0-Gate {}",
            match *self {
                Never => "false",
                Always => "true",
                Random => "rand",
            }
        )
    }
}

#[derive(Clone)]
pub enum Gate1 {
    Buffer,
}

impl Scrollable for Gate1 {
    fn next(&self) -> Self {
        use Gate1::*;
        match *self {
            Buffer => Buffer,
        }
    }

    fn prev(&self) -> Self {
        use Gate1::*;
        match *self {
            Buffer => Buffer,
        }
    }
}

impl GateIn for Gate1 {
    const MAX_INPUTS: usize = 1;
}

impl fmt::Display for Gate1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Gate1::*;
        write!(
            f,
            "1-Gate {}",
            match *self {
                Buffer => "buffer",
            }
        )
    }
}

#[derive(Clone)]
pub enum GateN {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

impl Scrollable for GateN {
    fn next(&self) -> Self {
        use GateN::*;
        match *self {
            And => Nand,
            Nand => Or,
            Or => Nor,
            Nor => Xor,
            Xor => Xnor,
            Xnor => And,
        }
    }

    fn prev(&self) -> Self {
        use GateN::*;
        match *self {
            And => Xnor,
            Nand => And,
            Or => Nand,
            Nor => Or,
            Xor => Nor,
            Xnor => Xor,
        }
    }
}

impl GateIn for GateN {
    const MAX_INPUTS: usize = usize::MAX;
}

impl fmt::Display for GateN {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use GateN::*;
        write!(
            f,
            "N-Gate {}",
            match *self {
                And => "and",
                Nand => "nand",
                Or => "or",
                Nor => "nor",
                Xor => "xor",
                Xnor => "xnor",
            }
        )
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum Gate {
    G0(Gate0),
    G1(Gate1),
    GN(GateN),
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Gate::*;

        match self {
            G0(gate_0) => gate_0.fmt(f),
            G1(gate_1) => gate_1.fmt(f),
            GN(gate_n) => gate_n.fmt(f),
        }
    }
}

impl Scrollable for Gate {
    fn next(&self) -> Self {
        use Gate::*;
        match self {
            G0(gate_0) => Gate::G0(gate_0.next()),
            G1(gate_1) => Gate::G1(gate_1.next()),
            GN(gate_n) => Gate::GN(gate_n.next()),
        }
    }

    fn prev(&self) -> Self {
        use Gate::*;
        match self {
            G0(gate_0) => Gate::G0(gate_0.prev()),
            G1(gate_1) => Gate::G1(gate_1.prev()),
            GN(gate_n) => Gate::GN(gate_n.prev()),
        }
    }
}

impl Gate {
    pub fn draw_v(&self, d: &mut RaylibDrawHandle, center: &Vector2, color: Color) {
        d.draw_circle_v(center, Coords::GRID_SIZE / 2.0, color)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, coords: &Coords, color: Color) {
        self.draw_v(d, &coords.into_position(), color);
    }
}
