mod text_box;
mod plot;

use raylib::prelude::*;

pub use text_box::*;
pub use plot::*;

pub enum Widget {
    TextBox(TextBoxWidget),
    Plot(PlotWidget),
}

impl Widget {
    pub fn want_focus(&self) -> bool {
        match self {
            Widget::TextBox(w) => { w.want_focus() }
            Widget::Plot(w) => { w.want_focus() }
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        match self {
            Widget::TextBox(w) => { w.rect }
            Widget::Plot(w) => { w.rect }
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

    pub fn add_text_box(&mut self, w: TextBoxWidget) -> usize {
        let index = self.widgets.len();
        self.widgets.push(Widget::TextBox(w));
        index
    }

    pub fn add_plot(&mut self, w: PlotWidget) -> usize {
        let index = self.widgets.len();
        self.widgets.push(Widget::Plot(w));
        index
    }

    pub fn get_text_box(&mut self, index: usize) -> Option<&TextBoxWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot(&mut self, index: usize) -> Option<&PlotWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    pub fn get_text_box_mut(&mut self, index: usize) -> Option<&mut TextBoxWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot_mut(&mut self, index: usize) -> Option<&mut PlotWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
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
            let mouse_pos = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);
            for (index, widget) in self.widgets.iter_mut().enumerate() {
                if widget.want_focus() && widget.get_rect().check_collision_point_rec(mouse_pos) {
                    self.focus = index;
                    break;
                }
            }
        }
    }
}
