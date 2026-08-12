use raylib::prelude::*;

use super::{
    is_key_pressed,
    is_key_pressed_with,
    KeyMod,
    TextCursor,
    MouseHandler,
    MouseAction,
};

pub struct TextBoxWidget {
    pub rect: Rectangle,
    pub editable: bool,
    text: String,
    cursor_pos: usize,
    mouse: MouseHandler,
}

impl TextBoxWidget {
    pub const BORDER: f32 = 2.0;
    pub const CURSOR_WIDTH: f32 = 3.0;
    pub const PAD_LEFT: f32 = 6.0;
    pub const PAD_TOP: f32 = 8.0;
    pub const PAD_BOTTOM: f32 = 4.0;
    pub const PAD_VERTICAL: f32 = Self::PAD_TOP + Self::PAD_BOTTOM;

    pub fn new(rect: Rectangle, text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor_pos = text.len();
        TextBoxWidget {
            rect,
            text,
            cursor_pos,
            editable: true,
            mouse: MouseHandler::new(),
        }
    }

    #[allow(unused)]
    pub fn read_only(mut self) -> Self {
        self.editable = false;
        self
    }

    pub fn want_focus(&self) -> bool {
        true
    }

    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        Some(MouseCursor::MOUSE_CURSOR_IBEAM)
    }

    fn cursor(&self) -> TextCursor {
        TextCursor::new(&self.text, self.cursor_pos)
    }

    fn new_cursor(&self, pos: usize) -> TextCursor {
        TextCursor::new(&self.text, pos)
    }

    fn fix_cursor_pos(&mut self) {
        self.cursor_pos = self.cursor().pos();
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
        self.cursor_pos = self.cursor().move_glyph(&self.text, dir);
    }

    pub fn move_cursor_word(&mut self, dir: i32) {
        let mut cursor = self.cursor();
        cursor.move_glyph_while(&self.text, dir, |glyph| glyph.chars().next().map(|ch| ! ch.is_alphanumeric()).unwrap_or(false));
        cursor.move_glyph_while(&self.text, dir, |glyph| glyph.chars().next().map(|ch| ch.is_alphanumeric()).unwrap_or(false));
        self.cursor_pos = cursor.pos();
    }

    pub fn delete_char(&mut self, dir: i32) -> bool {
        let mut cursor = self.cursor();
        let (start, end) = if dir < 0 {
            let end = self.cursor_pos;
            if let Some(start) = cursor.try_move_glyph(&self.text, -1) {
                self.cursor_pos = start;
                (start, end)
            } else {
                return false;
            }
        } else if dir > 0 {
            let start = self.cursor_pos;
            if let Some(end) = cursor.try_move_glyph(&self.text, 1) {
                (start, end)
            } else {
                return false;
            }
        } else {
            return false;
        };
        if start == end {
            return false;
        }

        self.text.replace_range(start..end, "");
        true
    }

    pub fn insert_char(&mut self, ch: char) -> bool {
        self.text.insert(self.cursor_pos, ch);
        self.move_cursor(1);
        true
    }

    pub fn handle_mouse(&mut self, rl: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32) -> bool {
        if ! self.mouse.is_inside(rl, self.rect) {
            return false;
        }

        let mouse_pos = rl.get_mouse_position();

        match self.mouse.update(rl) {
            MouseAction::Down(MouseButton::MOUSE_BUTTON_LEFT) |
            MouseAction::Drag(MouseButton::MOUSE_BUTTON_LEFT, _) => {
                let mouse_x = mouse_pos.x - (self.rect.x + Self::PAD_LEFT);
                let mut last_dist = f32::INFINITY;
                let mut cursor = self.new_cursor(0);
                loop {
                    let size = font.measure_text(&self.text[0..cursor.pos()], font_size, super::Widget::TEXT_SPACING);
                    let dist = mouse_x - size.x;
                    if dist < 0.0 || dist.abs() > last_dist {
                        if dist.abs() > last_dist {
                            cursor.move_glyph(&self.text, -1);
                        }
                        self.set_cursor_pos(cursor.pos());
                        break;
                    }
                    if cursor.move_glyph(&self.text, 1) == self.text.len() {
                        self.set_cursor_pos(cursor.pos());
                        break;
                    }
                    last_dist = dist;
                }
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn handle_keyboard(&mut self, d: &mut RaylibDrawHandle<'_>, focused: bool) -> bool {
        if ! focused || ! self.editable { return false; }

        if is_key_pressed(d, KeyboardKey::KEY_LEFT)  { self.move_cursor(-1); }
        if is_key_pressed(d, KeyboardKey::KEY_RIGHT) { self.move_cursor(1); }
        if is_key_pressed(d, KeyboardKey::KEY_HOME)  { self.set_cursor_pos(0); }
        if is_key_pressed(d, KeyboardKey::KEY_END)   { self.set_cursor_pos(self.get_text().len()); }

        if is_key_pressed_with(d, KeyboardKey::KEY_LEFT, KeyMod::Ctrl)  { self.move_cursor_word(-1); }
        if is_key_pressed_with(d, KeyboardKey::KEY_RIGHT, KeyMod::Ctrl) { self.move_cursor_word(1); }
        if is_key_pressed_with(d, KeyboardKey::KEY_A, KeyMod::Ctrl) { self.set_cursor_pos(0); }
        if is_key_pressed_with(d, KeyboardKey::KEY_E, KeyMod::Ctrl) { self.set_cursor_pos(self.get_text().len()); }
        if is_key_pressed_with(d, KeyboardKey::KEY_D, KeyMod::Ctrl) { self.delete_char(1); }

        if is_key_pressed(d, KeyboardKey::KEY_BACKSPACE) { return self.delete_char(-1); }
        if is_key_pressed(d, KeyboardKey::KEY_DELETE)    { return self.delete_char(1); }
        if let Some(ch) = d.get_char_pressed()           { return self.insert_char(ch); }

        if is_key_pressed_with(d, KeyboardKey::KEY_C, KeyMod::Ctrl) {
            d.set_clipboard_text(&self.text).unwrap_or(());
        }
        if is_key_pressed_with(d, KeyboardKey::KEY_V, KeyMod::Ctrl) && let Ok(text) = d.get_clipboard_text() && ! text.is_empty() {
            if let Some(line_end) = text.char_indices().find(|(_, ch)| *ch == '\r' || *ch == '\n').map(|(index, _)| index) {
                if line_end > 0 {
                    self.text.replace_range(self.cursor_pos..self.cursor_pos, &text[..line_end]);
                    self.set_cursor_pos(self.cursor_pos + line_end);
                    return true;
                }
            } else {
                self.text.replace_range(self.cursor_pos..self.cursor_pos, &text);
                self.set_cursor_pos(self.cursor_pos + text.len());
                return true
            }
        }
        false
    }

    fn draw_widget(&self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32, focused: bool, highlight: Option<Color>) {
        // draw background
        if focused {
            d.draw_rectangle_rec(self.rect, Color::new(240, 255, 255, 255));
        } else {
            d.draw_rectangle_rec(self.rect, Color::WHITE);
        }

        // draw border
        self.draw_highlight(d, highlight);

        // draw text
        let pos = Vector2::new(self.rect.x + Self::PAD_LEFT, self.rect.y + Self::PAD_TOP);
        d.draw_text_codepoints(font, &self.text, pos, font_size, super::Widget::TEXT_SPACING, Color::BLACK);

        if focused && self.editable {
            // draw cursor
            let cursor_offset = font.measure_text(&self.text[0..self.cursor_pos], font_size, super::Widget::TEXT_SPACING);
            let cursor_top = Vector2::new(pos.x + cursor_offset.x, self.rect.y + 4.0);
            let cursor_bot = Vector2::new(cursor_top.x, self.rect.y + self.rect.height - 4.0);
            d.draw_line_ex(cursor_top, cursor_bot, Self::CURSOR_WIDTH, Color::RED);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, font: &Font, font_size: f32, focused: bool, highlight: Option<Color>) {
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

    pub fn draw_highlight(&self, d: &mut RaylibDrawHandle<'_>, highlight: Option<Color>) {
        if let Some(highlight_color) = highlight {
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, highlight_color);
        } else {
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
        }
    }
}
