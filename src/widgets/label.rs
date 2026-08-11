use raylib::prelude::*;

use super::{
    TextAlign,
};

pub struct LabelWidget {
    pub rect: Rectangle,
    pub text: String,
    pub align: TextAlign,
}

impl LabelWidget {
    pub fn new(rect: Rectangle, text: impl Into<String>) -> Self {
        LabelWidget {
            rect,
            text: text.into(),
            align: TextAlign::Left,
        }
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
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

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }

        d.draw_rectangle_rec(self.rect, Color::WHITE);

        if ! self.text.is_empty() {
            let pos = match self.align {
                TextAlign::Left => {
                    Vector2::new(self.rect.x, self.rect.y)
                }
                TextAlign::Center => {
                    let size = font.measure_text(&self.text, font_size, super::Widget::TEXT_SPACING);
                    Vector2::new(
                        self.rect.x + (0.5 * (self.rect.width - size.x)).floor(),
                        self.rect.y
                    )
                }
                TextAlign::Right => {
                    let size = font.measure_text(&self.text, font_size, super::Widget::TEXT_SPACING);
                    Vector2::new(
                        self.rect.x + (self.rect.width - size.x).floor(),
                        self.rect.y
                    )
                }
            };
            d.draw_scissor_mode(
                self.rect.x.floor() as i32,
                self.rect.y.floor() as i32,
                self.rect.width.floor() as i32,
                self.rect.height.floor() as i32,
                |mut d| {
                    d.draw_text_codepoints(font, &self.text, pos, font_size, super::Widget::TEXT_SPACING, Color::BLACK);
                }
            );
        }
    }
}
