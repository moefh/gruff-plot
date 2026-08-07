mod text_box;
mod plot;
mod zoom_axis;

use raylib::prelude::*;

pub use text_box::*;
pub use plot::*;
pub use zoom_axis::*;

pub enum Widget {
    TextBox(TextBoxWidget),
    Plot(PlotWidget),
    ZoomAxis(ZoomAxisWidget),
}

impl Widget {
    pub fn want_focus(&self) -> bool {
        match self {
            Widget::TextBox(w) => { w.want_focus() }
            Widget::Plot(w) => { w.want_focus() }
            Widget::ZoomAxis(w) => { w.want_focus() }
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        match self {
            Widget::TextBox(w) => { w.rect }
            Widget::Plot(w) => { w.rect }
            Widget::ZoomAxis(w) => { w.rect }
        }
    }
}

pub struct WidgetBag {
    pub width: i32,
    pub height: i32,
    pub widgets: Vec<Widget>,
    pub focus: usize,
}

impl WidgetBag {
    pub fn new() -> Self {
        WidgetBag {
            widgets: Vec::new(),
            focus: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn advance_focus(&mut self, direction: i32) {
        if self.widgets.is_empty() {
            return;
        }

        let start = self.focus;
        loop {
            if direction >= 0 {
                self.focus = (self.focus + 1) % self.widgets.len();
            } else {
                self.focus = (self.focus + self.widgets.len() - 1) % self.widgets.len();
            }
            if self.focus == start || self.widgets[self.focus].want_focus() {
                break;
            }
        }
    }

    pub fn add_text_box(&mut self, w: TextBoxWidget) {
        self.widgets.push(Widget::TextBox(w));
    }

    pub fn add_plot(&mut self, w: PlotWidget) {
        self.widgets.push(Widget::Plot(w));
    }

    pub fn add_zoom_axis(&mut self, w: ZoomAxisWidget) {
        self.widgets.push(Widget::ZoomAxis(w));
    }

    pub fn get_text_box(&mut self, index: usize) -> Option<&TextBoxWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot(&mut self, index: usize) -> Option<&PlotWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    pub fn get_zoom_axis(&mut self, index: usize) -> Option<&PlotWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    pub fn get_text_box_mut(&mut self, index: usize) -> Option<&mut TextBoxWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot_mut(&mut self, index: usize) -> Option<&mut PlotWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    pub fn get_zoom_axis_mut(&mut self, index: usize) -> Option<&mut ZoomAxisWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::ZoomAxis(w) = w { Some(w) } else { None } })
    }

    pub fn clear_text_box_changed(&mut self) {
        for widget in self.widgets.iter_mut() {
            if let Widget::TextBox(text) = widget {
                text.changed = false;
            }
        }
    }

    pub fn handle_keyboard(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            let direction = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) { -1 } else { 1 };
            self.advance_focus(direction);
        }
    }

    pub fn handle_mouse(&mut self, rl: &RaylibHandle) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            for (index, widget) in self.widgets.iter_mut().enumerate() {
                if widget.want_focus() && widget.get_rect().check_collision_point_rec(mouse_pos) {
                    self.focus = index;
                    break;
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ZoomAxis {
    Both,
    X,
    Y,
}

impl ZoomAxis {
    pub fn has_x(self) -> bool { self == ZoomAxis::Both || self == ZoomAxis::X }
    pub fn has_y(self) -> bool { self == ZoomAxis::Both || self == ZoomAxis::Y }
}

#[allow(unused)]
#[derive(Debug)]
enum MouseAction {
    None,
    Up(MouseButton),
    Down(MouseButton),
    Drag(MouseButton, Vector2),
    Wheel(f32),
}

struct MouseHandler {
    pub button_down: Option<MouseButton>,
    pub drag_from: Vector2,
    pub drag_to: Vector2,
}

impl MouseHandler {
    fn new() -> Self {
        MouseHandler {
            button_down: None,
            drag_from: Vector2::new(0.0, 0.0),
            drag_to: Vector2::new(0.0, 0.0),
        }
    }

    fn is_inside(&self, rl: &RaylibHandle, rect: Rectangle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        rect.check_collision_point_rec(mouse_pos)
    }

    fn update(&mut self, rl: &RaylibHandle) -> MouseAction {
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

        return MouseAction::None;
    }
}
