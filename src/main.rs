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

fn draw_grid(d: &mut RaylibDrawHandle, width: i32, height: i32) {
    for x in (0..width).step_by(Coords::GRID_SIZE as usize) {
        d.draw_line(x, 0, x, height, Color::DARKGRAY);
    }
    for y in (0..height).step_by(Coords::GRID_SIZE as usize) {
        d.draw_line(0, y, width, y, Color::DARKGRAY);
    }
}

fn main() {
    let window_width: i32 = 1280;
    let window_height: i32 = 720;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Electron Architect")
        .vsync()
        .build();

    rl.set_target_fps(244);
    rl.hide_cursor();

    let input = InputHandler::new();
    let mut console = Console::new();
    let mut graph = Graph::new();
    let mut current_gate = Gate::G1(Gate1::Buffer);
    let mut current_elbow = Elbow::HorzS;
    let mut cursor: Cursor = Cursor::new();

    let mut hovered_node: Option<&Node> = None;
    let mut hovered_wire: Option<&Wire> = None;
    let mut current_node: Option<&Node> = None;

    console.log("Hello world!");

    while !rl.window_should_close() {
        // Tick

        cursor.update(&rl);

        hovered_node = graph.find_node_at_coords(&cursor.coords);
        hovered_wire = match hovered_node {
            None => graph.find_wire_intersecting_coords(&cursor.coords),
            Some(_) => None,
        };

        if input.is_pressed(&rl, &Input::CreateNode) {
            let new_node = graph.add_node(&current_gate, &cursor.coords);

            // Chain nodes
            if current_node.is_some() {
                graph.add_wire(current_node.unwrap(), new_node, &current_elbow);
            }
            current_node = Some(new_node);
            console.log(format!("Created node at {}", cursor.coords));
        }

        if input.is_pressed(&rl, &Input::IncrementGate) {
            current_gate.incr();
        } else if input.is_pressed(&rl, &Input::DecrementGate) {
            current_gate.decr();
        }

        if input.is_pressed(&rl, &Input::IncrementElbow) {
            current_elbow.incr();
        } else if input.is_pressed(&rl, &Input::DecrementElbow) {
            current_elbow.decr();
        }

        if (console.bounding_box()).check_collision_point_rec(&cursor.pos) {
            console.log("Hovering console");
        }

        // Draw
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            draw_grid(&mut d, window_width, window_height);

            graph.draw(&mut d);

            current_gate.draw(&mut d, &cursor.coords, Color::BLUE);

            console.draw(&mut d);

            cursor.draw(&mut d);
        }
    }
}
