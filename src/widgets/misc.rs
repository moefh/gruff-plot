use raylib::prelude::*;

#[allow(unused)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum KeyMod {
    Shift,
    Ctrl,
    Alt,
}

impl KeyMod {
    pub fn is_down(self, rl: &RaylibHandle) -> bool {
        match self {
            KeyMod::Alt => { rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT) }
            KeyMod::Ctrl => { rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) }
            KeyMod::Shift => { rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) }
        }
    }
}

pub fn is_key_pressed(rl: &RaylibHandle, key: KeyboardKey) -> bool {
    rl.is_key_pressed(key) || rl.is_key_pressed_repeat(key)
}

pub fn is_key_pressed_with(rl: &RaylibHandle, key: KeyboardKey, keymod: KeyMod) -> bool {
    is_key_pressed(rl, key) && keymod.is_down(rl)
}

#[allow(unused)]
#[derive(Debug)]
pub enum MouseAction {
    None,
    Up(MouseButton),
    Down(MouseButton),
    Drag(MouseButton, Vector2),
    Wheel(f32),
}

pub struct MouseHandler {
    pub button_down: Option<MouseButton>,
    pub drag_from: Vector2,
    pub drag_to: Vector2,
}

impl MouseHandler {
    pub fn new() -> Self {
        MouseHandler {
            button_down: None,
            drag_from: Vector2::new(0.0, 0.0),
            drag_to: Vector2::new(0.0, 0.0),
        }
    }

    pub fn is_inside(&self, rl: &RaylibHandle, rect: Rectangle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        rect.check_collision_point_rec(mouse_pos)
    }

    pub fn update(&mut self, rl: &RaylibHandle) -> MouseAction {
        if let Some(button) = self.button_down {
            // mouse up
            if ! rl.is_mouse_button_down(button) {
                self.button_down = None;
                return MouseAction::Up(button);
            }

            // mouse drag
            let pos = rl.get_mouse_position();
            if pos != self.drag_to {
                let delta = self.drag_to - pos;
                self.drag_to = pos;
                return MouseAction::Drag(button, delta);
            }
            return MouseAction::None;
        }

        // check button down
        for button in [MouseButton::MOUSE_BUTTON_LEFT, MouseButton::MOUSE_BUTTON_MIDDLE, MouseButton::MOUSE_BUTTON_RIGHT] {
            if rl.is_mouse_button_down(button) {
                self.button_down = Some(button);
                self.drag_from = rl.get_mouse_position();
                self.drag_to = self.drag_from;
                return MouseAction::Down(button);
            }
        }

        // check wheel
        let delta = rl.get_mouse_wheel_move();
        if delta != 0.0 {
            return MouseAction::Wheel(delta);
        }

        MouseAction::None
    }
}
