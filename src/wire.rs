use crate::node::Node;

#[derive(Clone)]
pub enum Elbow {
    Horizontal,
    Vertical,
    DiagonalFirst,
    DiagonalLast,
}

impl Elbow {
    pub fn next(&self) -> Self {
        use Elbow::*;
        match *self {
            Horizontal => DiagonalLast,
            DiagonalLast => Vertical,
            Vertical => DiagonalFirst,
            DiagonalFirst => Horizontal,
        }
    }

    pub fn prev(&self) -> Self {
        use Elbow::*;
        match *self {
            Horizontal => DiagonalFirst,
            DiagonalLast => Horizontal,
            Vertical => DiagonalLast,
            DiagonalFirst => Vertical,
        }
    }
}

pub struct Wire {
    pub input: *mut Node,
    pub output: *mut Node,
    pub elbow: Elbow,
}

impl Wire {
    pub fn new(input: *mut Node, output: *mut Node, elbow: Elbow) -> Self {
        Self {
            input,
            output,
            elbow,
        }
    }
}
