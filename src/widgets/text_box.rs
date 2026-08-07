use raylib::prelude::*;
use unicode_segmentation::GraphemeCursor;

fn is_key_pressed(d: &RaylibDrawHandle<'_>, key: KeyboardKey) -> bool {
    d.is_key_pressed(key) || d.is_key_pressed_repeat(key)
}

pub struct TextBoxWidget {
    pub rect: Rectangle,
    pub editable: bool,
    pub changed: bool,
    text: String,
    cursor_pos: usize,
}

impl TextBoxWidget {
    pub const SPACING: f32 = 1.0;
    pub const BORDER: f32 = 2.0;
    pub const CURSOR_WIDTH: f32 = 3.0;
    pub const PAD_LEFT: f32 = 6.0;
    pub const PAD_TOP: f32 = 8.0;
    pub const PAD_BOTTOM: f32 = 4.0;
    pub const PAD_VERTICAL: f32 = Self::PAD_TOP + Self::PAD_BOTTOM;

    pub fn new(rect: Rectangle) -> Self {
        TextBoxWidget {
            rect,
            text: String::new(),
            editable: true,
            cursor_pos: 0,
            changed: false,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.set_text(text);
        self
    }

    #[allow(unused)]
    pub fn read_only(mut self) -> Self {
        self.editable = false;
        self
    }

    pub fn want_focus(&self) -> bool {
        true
    }

    fn try_fix_cursor_pos(&mut self) -> Option<usize> {
        let text_len = self.text.len();
        if self.cursor_pos > text_len {
            return None;
        }

        let mut cursor = GraphemeCursor::new(self.cursor_pos, text_len, true);
        if ! cursor.is_boundary(&self.text, 0).ok()? {
            cursor.next_boundary(&self.text, 0).ok()?
        } else {
            Some(self.cursor_pos)
        }
    }

    fn fix_cursor_pos(&mut self) {
        if let Some(pos) = self.try_fix_cursor_pos() {
            self.cursor_pos = pos;
        } else {
            self.cursor_pos = self.text.len();
        }
    }

    #[allow(unused)]
    pub fn set_cursor_pos(&mut self, cursor_pos: usize) {
        self.cursor_pos = cursor_pos;
        self.fix_cursor_pos();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl AsRef<str>) {
        self.text.replace_range(.., text.as_ref());
        self.cursor_pos = self.text.len();
        self.fix_cursor_pos();
    }

    pub fn move_cursor(&mut self, dir: i32) {
        let mut cursor = GraphemeCursor::new(self.cursor_pos, self.text.len(), true);
        if dir < 0 && let Ok(Some(pos)) = cursor.prev_boundary(&self.text, 0) {
            self.cursor_pos = pos;
        } else if dir > 0 && let Ok(Some(pos)) = cursor.next_boundary(&self.text, 0) {
            self.cursor_pos = pos;
        }
    }

    pub fn delete_char(&mut self, dir: i32) {
        let mut cursor = GraphemeCursor::new(self.cursor_pos, self.text.len(), true);
        let (start, end) = if dir < 0 {
            let end = self.cursor_pos;
            if let Ok(Some(start)) = cursor.prev_boundary(&self.text, 0) {
                self.cursor_pos = start;
                (start, end)
            } else {
                return;
            }
        } else if dir > 0 {
            let start = self.cursor_pos;
            if let Ok(Some(end)) = cursor.next_boundary(&self.text, 0) {
                (start, end)
            } else {
                return;
            }
        } else {
            return;
        };
        if start != end {
            self.text.replace_range(start..end, "");
            self.changed = true;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.text.insert(self.cursor_pos, ch);
        self.move_cursor(1);
        self.changed = true;
    }

    fn draw_widget(&mut self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32, focused: bool, highlight: Option<Color>) {
        // draw background
        if focused {
            d.draw_rectangle_rec(self.rect, Color::new(224, 224, 224, 255));
        } else {
            d.draw_rectangle_rec(self.rect, Color::WHITE);
        }

        // draw border
        if let Some(highlight_color) = highlight {
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, highlight_color);
        } else {
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
        }

        // draw text
        let pos = Vector2::new(self.rect.x + Self::PAD_LEFT, self.rect.y + Self::PAD_TOP);
        d.draw_text_codepoints(font, &self.text, pos, font_size, Self::SPACING, Color::BLACK);

        if focused && self.editable {
            // draw cursor
            let cursor_offset = font.measure_text(&self.text[0..self.cursor_pos], font_size, Self::SPACING);
            let cursor_top = Vector2::new(pos.x + cursor_offset.x, self.rect.y + 4.0);
            let cursor_bot = Vector2::new(cursor_top.x, self.rect.y + self.rect.height - 4.0);
            d.draw_line_ex(cursor_top, cursor_bot, Self::CURSOR_WIDTH, Color::RED);

            // handle keyboard
            if is_key_pressed(d, KeyboardKey::KEY_LEFT)      { self.move_cursor(-1); }
            if is_key_pressed(d, KeyboardKey::KEY_RIGHT)     { self.move_cursor(1); }
            if is_key_pressed(d, KeyboardKey::KEY_HOME)      { self.set_cursor_pos(0); }
            if is_key_pressed(d, KeyboardKey::KEY_END)       { self.set_cursor_pos(self.get_text().len()); }
            if is_key_pressed(d, KeyboardKey::KEY_BACKSPACE) { self.delete_char(-1); }
            if is_key_pressed(d, KeyboardKey::KEY_DELETE)    { self.delete_char(1); }
            if let Some(ch) = d.get_char_pressed()           { self.insert_char(ch); }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32, focused: bool, highlight: Option<Color>) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }

        d.draw_scissor_mode(
            self.rect.x.floor() as i32,
            self.rect.y.floor() as i32,
            self.rect.width.floor() as i32,
            self.rect.height.floor() as i32,
            |mut d| {
                self.draw_widget(&mut d, font, font_size, focused, highlight);
            }
        );
    }
}
