use crate::common_traits::Scrollable;

#[derive(Clone, Debug)]
pub enum Elbow {
    HorzS, // Horizontal start
    VertS, // Vertical start
    DiagS, // Diagonal start
    DiagE, // Diagonal end
}

impl Scrollable for Elbow {
    fn next(&self) -> Self {
        use Elbow::*;
        match *self {
            HorzS => DiagE,
            DiagE => VertS,
            VertS => DiagS,
            DiagS => HorzS,
        }
    }

    fn prev(&self) -> Self {
        use Elbow::*;
        match *self {
            HorzS => DiagS,
            DiagE => HorzS,
            VertS => DiagE,
            DiagS => VertS,
        }
    }
}
