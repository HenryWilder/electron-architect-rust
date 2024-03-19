use raylib::prelude::*;

use crate::coords::Coords;

pub struct Cursor {
    pub pos: Vector2,
    pub coords: Coords,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: Vector2::default(),
            coords: Coords::default(),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.pos = rl.get_mouse_position();
        self.coords = Coords::from_position(self.pos);
    }

    const WIDTH: f32 = 3.0;

    const SIZE: Vector2 = Vector2 {
        x: Cursor::WIDTH,
        y: Cursor::WIDTH,
    };

    const EXTENT: Vector2 = Vector2 {
        x: Cursor::WIDTH / 2.0,
        y: Cursor::WIDTH / 2.0,
    };

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS)
            .draw_rectangle_v(self.pos - Cursor::EXTENT, Cursor::SIZE, Color::WHITE);
    }
}
