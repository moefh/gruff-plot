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
    const PAD_TOP: f32 = 8.0;

    pub fn new(rect: Rectangle, text: impl Into<String>) -> Self {
        ButtonWidget {
            rect,
            text: text.into(),
            mouse: MouseHandler::new(),
        }
    }

    pub fn want_focus(&self) -> bool {
        false
    }

    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        None
    }

    pub fn set_text(&mut self, text: impl AsRef<str>) {
        self.text.replace_range(.., text.as_ref());
    }

    pub fn clicked(&mut self, d: &mut RaylibDrawHandle<'_>) -> bool {
        if ! self.mouse.is_inside(d, self.rect) {
            false
        } else {
            matches!(self.mouse.update(d), MouseAction::Up(MouseButton::MOUSE_BUTTON_LEFT))
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }

        d.draw_rectangle_rec(self.rect, Color::new(224, 224, 224, 255));
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);

        if ! self.text.is_empty() {
            let size = font.measure_text(&self.text, font_size, super::Widget::TEXT_SPACING);
            let pos = Vector2::new(
                self.rect.x + (0.5 * (self.rect.width - size.x)).floor(),
                self.rect.y + Self::PAD_TOP
            );
            d.draw_text_codepoints(font, &self.text, pos, font_size, super::Widget::TEXT_SPACING, Color::BLACK);
        }
    }
}
