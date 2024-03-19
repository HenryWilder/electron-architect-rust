use raylib::{
    consts::{KeyboardKey, MouseButton},
    RaylibHandle,
};

#[derive(Hash, PartialEq, Eq)]
pub enum Input {
    CreateNode,
    IncrementGate,
    DecrementGate,
}

impl Input {
    pub const fn len() -> usize {
        // todo: surely there's a standardized way to do this...
        return 3;
    }

    pub const fn variants() -> [Input; Input::len()] {
        use Input::*;
        // todo: surely there's a standardized way to do this...
        [CreateNode, IncrementGate, DecrementGate]
    }

    const fn bind_index(&self) -> usize {
        use Input::*;
        // todo: make this more programatic
        match *self {
            CreateNode => 0,
            IncrementGate => 1,
            DecrementGate => 2,
        }
    }
}

enum ScrollDirection {
    Positive,
    Negative,
}

enum InputBind {
    Scroll(ScrollDirection),
    Key(KeyboardKey),
    Btn(MouseButton),
}

impl Input {
    fn default_binding(&self) -> InputBind {
        use {Input::*, InputBind::*};
        match *self {
            CreateNode => Btn(MouseButton::MOUSE_LEFT_BUTTON),
            IncrementGate => Scroll(ScrollDirection::Positive),
            DecrementGate => Scroll(ScrollDirection::Negative),
        }
    }
}

pub struct InputHandler {
    bindings: [InputBind; Input::len()],
}

const _INPUT_CONFIG_FILENAME: &str = "keybinds.config";

impl InputHandler {
    pub fn new() -> Self {
        use Input::*;
        // todo: idiomatically associate each enum with a value
        Self {
            bindings: [
                CreateNode.default_binding(),
                IncrementGate.default_binding(),
                DecrementGate.default_binding(),
            ],
        }
    }

    fn set_binding(&mut self, input: Input, bind: InputBind) {
        self.bindings[input.bind_index()] = bind;
    }

    fn get_binding<'idk>(&'idk self, input: Input) -> &'idk InputBind {
        &self.bindings[input.bind_index()]
    }

    fn is_scrolled(rl: &RaylibHandle, dir: &ScrollDirection) -> bool {
        match dir {
            ScrollDirection::Positive => rl.get_mouse_wheel_move() > 0.0,
            ScrollDirection::Negative => rl.get_mouse_wheel_move() < 0.0,
        }
    }

    pub fn is_pressed(&self, rl: &RaylibHandle, id: Input) -> bool {
        match self.get_binding(id) {
            InputBind::Scroll(dir) => InputHandler::is_scrolled(rl, dir),
            InputBind::Key(key) => rl.is_key_pressed(*key),
            InputBind::Btn(btn) => rl.is_mouse_button_pressed(*btn),
        }
    }

    pub fn is_down(&self, rl: &RaylibHandle, id: Input) -> bool {
        match self.get_binding(id) {
            InputBind::Scroll(dir) => InputHandler::is_scrolled(rl, dir),
            InputBind::Key(key) => rl.is_key_down(*key),
            InputBind::Btn(btn) => rl.is_mouse_button_down(*btn),
        }
    }

    pub fn is_up(&self, rl: &RaylibHandle, id: Input) -> bool {
        match self.get_binding(id) {
            InputBind::Scroll(dir) => InputHandler::is_scrolled(rl, dir),
            InputBind::Key(key) => rl.is_key_up(*key),
            InputBind::Btn(btn) => rl.is_mouse_button_up(*btn),
        }
    }

    pub fn is_released(&self, rl: &RaylibHandle, id: Input) -> bool {
        match self.get_binding(id) {
            InputBind::Scroll(dir) => InputHandler::is_scrolled(rl, dir),
            InputBind::Key(key) => rl.is_key_released(*key),
            InputBind::Btn(btn) => rl.is_mouse_button_released(*btn),
        }
    }
}
