use crate::{
    coords::Coords,
    graph::{elbow::Elbow, node::Node},
};

#[allow(dead_code)]
pub struct Wire<'graph> {
    pub input: &'graph Node,
    pub output: &'graph Node,
    pub elbow: Elbow,
}

impl<'graph> Wire<'graph> {
    pub fn new(input: &'graph Node, output: &'graph Node, elbow: &Elbow) -> Self {
        Self {
            input,
            output,
            elbow: *elbow,
        }
    }

    pub fn is_intersecting_coords(&self, _: &Coords) -> bool {
        todo!("Not yet implemented");
    }
}
