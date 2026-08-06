mod text_box;
mod plot;

pub use text_box::*;
pub use plot::*;

pub trait WidgetBehavior {
    fn want_focus(&self) -> bool { false }
}

pub enum Widget {
    TextBox(TextBoxWidget),
    Plot(PlotWidget),
}

impl Widget {
    pub fn behavior(&self) -> &dyn WidgetBehavior {
        match self {
            Widget::TextBox(w) => { return w; }
            Widget::Plot(w) => { return w; }
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
        let start = self.focus;
        loop {
            if direction >= 0 {
                self.focus = (self.focus + 1) % self.widgets.len();
            } else {
                self.focus = (self.focus + self.widgets.len() - 1) % self.widgets.len();
            }
            if self.focus == start || self.widgets[self.focus].behavior().want_focus() {
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
}
