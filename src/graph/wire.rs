use crate::{
    coords::Coords,
    graph::{elbow::Elbow, node::Node},
};

#[allow(dead_code)]
pub struct Wire {
    pub input: Node,
    pub output: Node,
    pub elbow: Elbow,
}

impl Wire {
    pub fn is_intersecting_coords(&self, _: &Coords) -> bool {
        todo!("Not yet implemented");
    }
}
