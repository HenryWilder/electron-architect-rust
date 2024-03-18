pub enum Elbow {
    Horizontal,
    Vertical,
    DiagonalFirst,
    DiagonalLast,
}

struct Node;

pub struct Wire {
    input: *mut Node,
    output: *mut Node,
    elbow: Elbow,
}

impl Wire {}
