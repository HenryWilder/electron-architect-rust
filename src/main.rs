use raylib::prelude::*;
mod common_traits;
mod console;
mod coords;
mod cursor;
mod graph;
mod input;

#[allow(unused_imports)]
use {
    common_traits::Scrollable,
    console::Console,
    coords::Coords,
    cursor::Cursor,
    graph::{
        elbow::Elbow,
        gate::{Gate, Gate0, Gate1, GateN},
        node::Node,
        wire::Wire,
        Graph,
    },
    input::{Input, InputHandler},
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Electron Architect")
        .vsync()
        .build();

    rl.hide_cursor();

    let input = InputHandler::new();
    let mut console = Console::new();
    let mut graph = Graph::new();
    let mut current_gate = Gate::G1(Gate1::Buffer);
    let mut cursor: Cursor = Cursor::new();

    console.log("Hello world!");

    while !rl.window_should_close() {
        // Tick

        cursor.update(&rl);

        if input.is_pressed(&rl, &Input::CreateNode) {
            graph.add_node(&current_gate, &cursor.coords);
            console.log("Added node");
        }

        if input.is_pressed(&rl, &Input::IncrementGate) {
            current_gate.incr();
            console.scroll_up();
        }

        if input.is_pressed(&rl, &Input::DecrementGate) {
            current_gate.decr();
            console.scroll_down();
        }

        // Draw

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        console.draw(&mut d);

        cursor.draw(&mut d);
    }
}
