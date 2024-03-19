use std::collections::HashMap;

use raylib::{
    consts::{KeyboardKey, MouseButton},
    RaylibHandle,
};

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Input {
    CreateNode,
    IncrementGate,
    DecrementGate,
}

enum ScrollDirection {
    Positive,
    Negative,
}

#[allow(dead_code)]
enum KeyBind {
    /// Scrollwheel
    Whl(ScrollDirection),

    /// Keyboard key
    Key(KeyboardKey),

    /// Mouse button
    Btn(MouseButton),
}

impl Input {
    const fn default_binding(&self) -> KeyBind {
        use {Input::*, KeyBind::*};
        match *self {
            CreateNode => Btn(MouseButton::MOUSE_LEFT_BUTTON),
            IncrementGate => Whl(ScrollDirection::Positive),
            DecrementGate => Whl(ScrollDirection::Negative),
        }
    }
}

pub struct InputHandler {
    bindings: HashMap<Input, KeyBind>,
}

const _INPUT_CONFIG_FILENAME: &str = "keybinds.config";

impl InputHandler {
    pub fn new() -> Self {
        use Input::*;
        Self {
            bindings: HashMap::from([
                (CreateNode, CreateNode.default_binding()),
                (IncrementGate, IncrementGate.default_binding()),
                (DecrementGate, DecrementGate.default_binding()),
            ]),
        }
    }

    fn is_scrolled(rl: &RaylibHandle, dir: &ScrollDirection) -> bool {
        match dir {
            ScrollDirection::Positive => rl.get_mouse_wheel_move() > 0.0,
            ScrollDirection::Negative => rl.get_mouse_wheel_move() < 0.0,
        }
    }

    /// Tells whether the Input has been pressed since last checked.
    ///
    /// # Example
    /// ```no_run
    /// let (mut rl, thread) = raylib::init().size(640, 480).title("Test").build();
    /// let handler = InputHandler::new();
    ///
    /// let pressed: bool = handler.is_pressed(&rl, &Input::CreateNode);
    /// ```
    pub fn is_pressed(&self, rl: &RaylibHandle, id: &Input) -> bool {
        use KeyBind::*;
        match self.bindings.get(id) {
            Some(Whl(dir)) => InputHandler::is_scrolled(rl, dir),
            Some(Key(key)) => rl.is_key_pressed(*key),
            Some(Btn(btn)) => rl.is_mouse_button_pressed(*btn),
            None => panic!("Missing input binding"),
        }
    }

    pub fn _is_down(&self, rl: &RaylibHandle, id: &Input) -> bool {
        use KeyBind::*;
        match self.bindings.get(id) {
            Some(Whl(dir)) => InputHandler::is_scrolled(rl, dir),
            Some(Key(key)) => rl.is_key_down(*key),
            Some(Btn(btn)) => rl.is_mouse_button_down(*btn),
            None => panic!("Missing input binding"),
        }
    }

    pub fn _is_up(&self, rl: &RaylibHandle, id: &Input) -> bool {
        use KeyBind::*;
        match self.bindings.get(id) {
            Some(Whl(dir)) => InputHandler::is_scrolled(rl, dir),
            Some(Key(key)) => rl.is_key_up(*key),
            Some(Btn(btn)) => rl.is_mouse_button_up(*btn),
            None => panic!("Missing input binding"),
        }
    }

    pub fn _is_released(&self, rl: &RaylibHandle, id: &Input) -> bool {
        use KeyBind::*;
        match self.bindings.get(id) {
            Some(Whl(dir)) => InputHandler::is_scrolled(rl, dir),
            Some(Key(key)) => rl.is_key_released(*key),
            Some(Btn(btn)) => rl.is_mouse_button_released(*btn),
            None => panic!("Missing input binding"),
        }
    }
}
