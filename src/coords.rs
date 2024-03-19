use crate::Vector2;

pub const GRID_SIZE: f32 = 8.0;

#[derive(Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn to_position(&self) -> Vector2 {
        Vector2 {
            x: self.x as f32 * GRID_SIZE,
            y: self.y as f32 * GRID_SIZE,
        }
    }

    pub fn from_position(p: Vector2) -> Self {
        Self {
            x: (p.x / GRID_SIZE).round() as i32,
            y: (p.y / GRID_SIZE).round() as i32,
        }
    }

    pub fn to_vector(&self) -> Vector2 {
        Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn from_vector(&mut self, v: Vector2) -> Self {
        Self {
            x: v.x.round() as i32,
            y: v.y.round() as i32,
        }
    }
}
