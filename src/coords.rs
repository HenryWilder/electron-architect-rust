use crate::Vector2;

pub const GRID_SIZE: f32 = 8.0;

pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn to_position(&self) -> Vector2 {
        return Vector2 {
            x: self.x as f32 * GRID_SIZE,
            y: self.y as f32 * GRID_SIZE,
        };
    }

    pub fn from_position(&mut self, p: Vector2) {
        self.x = (p.x / GRID_SIZE).round() as i32;
        self.y = (p.y / GRID_SIZE).round() as i32;
    }

    pub fn to_vector(&self) -> Vector2 {
        return Vector2 {
            x: self.x as f32,
            y: self.y as f32,
        };
    }

    pub fn from_vector(&mut self, v: Vector2) {
        self.x = v.x.round() as i32;
        self.y = v.y.round() as i32;
    }
}
