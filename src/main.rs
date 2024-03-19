use raylib::prelude::*;
mod console;
mod coords;
mod graph;
mod input;
mod node;
mod wire;

use {console::*, coords::*, graph::*, input::*, node::*, wire::*};

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

    let input = InputHandler::new();
    let mut offset: usize = 0;
    let mut console = Console::new();
    let mut graph = Graph::new();
    let mut current_gate: Gate = Gate::Buffer;

    console.writeln("Hello world!");

    while !rl.window_should_close() {
        // Tick

        let mouse_pos = rl.get_mouse_position();
        let mouse_coords = Coords::from_position(mouse_pos);

        if input.is_pressed(&rl, Input::CreateNode) {
            graph.add_node(&current_gate, &mouse_coords);
            console.writeln("Added node");
        }

        if input.is_pressed(&rl, Input::IncrementGate) {
            current_gate = current_gate.next();
            offset += 1;
        }

        if input.is_pressed(&rl, Input::DecrementGate) {
            current_gate = current_gate.prev();
            if offset > 0 {
                offset -= 1;
            }
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut b = d.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS);
            b.draw_rectangle_v(mouse_pos - CURSOR_EXTENT, CURSOR_SIZE, Color::WHITE);
        }

        d.draw_text(console.get_lines(offset, 10), 12, 12, 8, Color::BLUE);
        d.draw_text(format!("offset: {offset}").as_str(), 80, 12, 8, Color::BLUE);
    }
}
