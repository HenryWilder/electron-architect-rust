use crate::graph::{elbow::Elbow, node::Node};

pub struct Wire {
    pub input: Node,
    pub output: Node,
    pub elbow: Elbow,
}
