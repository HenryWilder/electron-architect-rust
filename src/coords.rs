use std::convert::From;
use std::fmt::Display;

use crate::Vector2;

#[derive(Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn default() -> Self {
        Self {
            x: i32::default(),
            y: i32::default(),
        }
    }

    /// Size in world units of the space between gridlines (including the infinitely thin gridline itself)
    pub const GRID_SIZE: f32 = 8.0;

    /// Converts grid coordinates to world vector
    pub fn into_position(self) -> Vector2 {
        Vector2 {
            x: self.x as f32 * Coords::GRID_SIZE,
            y: self.y as f32 * Coords::GRID_SIZE,
        }
    }

    /// Converts world vector to grid coordinates
    pub fn from_position(p: Vector2) -> Self {
        Self {
            x: (p.x / Coords::GRID_SIZE).round() as i32,
            y: (p.y / Coords::GRID_SIZE).round() as i32,
        }
    }
}

impl From<Vector2> for Coords {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}

impl From<Coords> for Vector2 {
    fn from(value: Coords) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
