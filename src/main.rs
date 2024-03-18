use raylib::prelude::*;

const CURSOR_WIDTH: f32 = 2.0;
const CURSOR_SIZE: Vector2 = Vector2 {
    x: CURSOR_WIDTH,
    y: CURSOR_WIDTH,
};
const CURSOR_EXTENT: Vector2 = Vector2 {
    x: CURSOR_WIDTH / 2.0,
    y: CURSOR_WIDTH / 2.0,
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Electron Architect")
        .vsync()
        .build();

    rl.hide_cursor();

    while !rl.window_should_close() {
        // Tick

        let mouse_pos = rl.get_mouse_position();

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut b = d.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS);
            b.draw_rectangle_v(mouse_pos - CURSOR_EXTENT, CURSOR_SIZE, Color::WHITE);
        }
    }
}
