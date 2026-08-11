use raylib::prelude::*;

use super::{
    MouseHandler,
    MouseAction,
};

pub struct ButtonWidget {
    pub rect: Rectangle,
    pub text: String,
    mouse: MouseHandler,
}

impl ButtonWidget {
    const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle) -> Self {
        ButtonWidget {
            rect,
            text: String::new(),
            mouse: MouseHandler::new(),
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = String::from(text);
        self
    }

    pub fn want_focus(&self) -> bool {
        false
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.replace_range(.., text);
    }

    pub fn clicked(&mut self, d: &mut RaylibDrawHandle<'_>) -> bool {
        if ! self.mouse.is_inside(d, self.rect) {
            false
        } else {
            matches!(self.mouse.update(d), MouseAction::Up(MouseButton::MOUSE_BUTTON_LEFT))
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }

        d.draw_rectangle_rec(self.rect, Color::WHITE);
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);

        if ! self.text.is_empty() {
            let size = font.measure_text(&self.text, font_size, super::Widget::TEXT_SPACING);
            let pos = Vector2::new(
                self.rect.x + (0.5 * (self.rect.width - size.x)).floor(),
                self.rect.y + (0.5 * (self.rect.height - size.y)).floor()
            );
            d.draw_text_codepoints(font, &self.text, pos, font_size, super::Widget::TEXT_SPACING, Color::BLACK);
        }
    }
}
