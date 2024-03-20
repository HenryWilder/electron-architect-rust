use crate::graph::{elbow::Elbow, node::Node};

#[allow(dead_code)]
pub struct Wire {
    pub input: Node,
    pub output: Node,
    pub elbow: Elbow,
}
