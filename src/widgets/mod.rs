mod text_box;
mod plot;
mod button;
mod label;
mod misc;
mod cursor;

use raylib::prelude::*;

pub use text_box::*;
pub use plot::*;
pub use button::*;
pub use label::*;
pub use misc::*;
pub use cursor::*;

pub enum Widget {
    TextBox(TextBoxWidget),
    Plot(PlotWidget),
    Button(ButtonWidget),
    Label(LabelWidget),
}

impl Widget {
    pub const TEXT_SPACING: f32 = 1.0;

    pub fn want_focus(&self) -> bool {
        match self {
            Widget::TextBox(w) => { w.want_focus() }
            Widget::Plot(w) => { w.want_focus() }
            Widget::Button(w) => { w.want_focus() }
            Widget::Label(w) => { w.want_focus() }
        }
    }

    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        match self {
            Widget::TextBox(w) => { w.mouse_cursor() }
            Widget::Plot(w) => { w.mouse_cursor() }
            Widget::Button(w) => { w.mouse_cursor() }
            Widget::Label(w) => { w.mouse_cursor() }
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        match self {
            Widget::TextBox(w) => { w.rect }
            Widget::Plot(w) => { w.rect }
            Widget::Button(w) => { w.rect }
            Widget::Label(w) => { w.rect }
        }
    }
}

pub struct WidgetBag {
    pub widgets: Vec<Widget>,
    pub focus: usize,
}

impl WidgetBag {
    pub fn new() -> Self {
        WidgetBag {
            widgets: Vec::new(),
            focus: 0,
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
        let widget = self.widgets.len();
        self.widgets.push(Widget::TextBox(w));
        widget
    }

    pub fn add_plot(&mut self, w: PlotWidget) -> usize {
        let widget = self.widgets.len();
        self.widgets.push(Widget::Plot(w));
        widget
    }

    pub fn add_button(&mut self, w: ButtonWidget) -> usize {
        let widget = self.widgets.len();
        self.widgets.push(Widget::Button(w));
        widget
    }

    pub fn add_label(&mut self, w: LabelWidget) -> usize {
        let widget = self.widgets.len();
        self.widgets.push(Widget::Label(w));
        widget
    }

    pub fn get_text_box(&self, index: usize) -> Option<&TextBoxWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot(&self, index: usize) -> Option<&PlotWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    #[allow(unused)]
    pub fn get_button(&self, index: usize) -> Option<&ButtonWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Button(w) = w { Some(w) } else { None } })
    }

    pub fn get_label(&self, index: usize) -> Option<&LabelWidget> {
        self.widgets.get(index).and_then(|w| { if let Widget::Label(w) = w { Some(w) } else { None } })
    }

    pub fn get_text_box_mut(&mut self, index: usize) -> Option<&mut TextBoxWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::TextBox(w) = w { Some(w) } else { None } })
    }

    pub fn get_plot_mut(&mut self, index: usize) -> Option<&mut PlotWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::Plot(w) = w { Some(w) } else { None } })
    }

    pub fn get_button_mut(&mut self, index: usize) -> Option<&mut ButtonWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::Button(w) = w { Some(w) } else { None } })
    }

    pub fn get_label_mut(&mut self, index: usize) -> Option<&mut LabelWidget> {
        self.widgets.get_mut(index).and_then(|w| { if let Widget::Label(w) = w { Some(w) } else { None } })
    }

    pub fn handle_keyboard(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            let direction = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) { -1 } else { 1 };
            self.advance_focus(direction);
        }
    }

    pub fn handle_mouse(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();

        // set cursor
        let mouse_widget = self.widgets.iter().find(|w| w.get_rect().check_collision_point_rec(mouse_pos));
        if let Some(widget) = mouse_widget {
            let cursor = widget.mouse_cursor().unwrap_or(MouseCursor::MOUSE_CURSOR_DEFAULT);
            rl.set_mouse_cursor(cursor);
        } else {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        // change focus
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            for (index, widget) in self.widgets.iter_mut().enumerate() {
                if widget.want_focus() && widget.get_rect().check_collision_point_rec(mouse_pos) {
                    self.focus = index;
                    break;
                }
            }
        }
    }
}
