use crate::common_traits::Scrollable;

#[derive(Clone, Debug)]
pub enum Elbow {
    Horizontal,
    Vertical,
    DiagonalFirst,
    DiagonalLast,
}

impl Scrollable for Elbow {
    fn next(&self) -> Self {
        use Elbow::*;
        match *self {
            Horizontal => DiagonalLast,
            DiagonalLast => Vertical,
            Vertical => DiagonalFirst,
            DiagonalFirst => Horizontal,
        }
    }

    fn prev(&self) -> Self {
        use Elbow::*;
        match *self {
            Horizontal => DiagonalFirst,
            DiagonalLast => Horizontal,
            Vertical => DiagonalLast,
            DiagonalFirst => Vertical,
        }
    }
}
