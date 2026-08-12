use unicode_segmentation::GraphemeCursor;

pub struct TextCursor {
    cursor: GraphemeCursor,
}

impl TextCursor {
    pub fn new(text: &str, pos: usize) -> Self {
        let mut cursor = GraphemeCursor::new(pos, text.len(), true);
        if ! cursor.is_boundary(text, 0).unwrap_or(false) {
            if cursor.next_boundary(text, 0).is_err() {
                cursor.set_cursor(text.len());
            }
        }
        TextCursor {
            cursor
        }
    }

    pub fn pos(&self) -> usize {
        self.cursor.cur_cursor()
    }

    pub fn move_glyph(&mut self, text: &str, dir: i32) -> usize {
        if dir < 0 {
            self.cursor.prev_boundary(text, 0).unwrap_or(Some(self.cursor.cur_cursor())).unwrap_or(0);
        } else if dir > 0 {
            self.cursor.next_boundary(text, 0).unwrap_or(Some(self.cursor.cur_cursor())).unwrap_or(text.len());
        }
        self.cursor.cur_cursor()
    }

    pub fn try_move_glyph(&mut self, text: &str, dir: i32) -> Option<usize> {
        if dir < 0 {
            self.cursor.prev_boundary(text, 0).unwrap_or(None);
        } else if dir > 0 {
            self.cursor.next_boundary(text, 0).unwrap_or(None);
        }
        Some(self.cursor.cur_cursor())
    }

    pub fn move_glyph_while(&mut self, text: &str, dir: i32, test: fn (&str) -> bool) -> usize {
        let mut prev_pos = self.cursor.cur_cursor();
        if dir < 0 {
            while let Ok(Some(pos)) = self.cursor.prev_boundary(text, 0) {
                if ! test(&text[pos..prev_pos]) {
                    self.cursor.set_cursor(prev_pos);
                    return prev_pos;
                }
                prev_pos = pos;
            }
        } else if dir > 0 {
            while let Ok(Some(pos)) = self.cursor.next_boundary(text, 0) {
                if ! test(&text[prev_pos..pos]) {
                    self.cursor.set_cursor(prev_pos);
                    return prev_pos;
                }
                prev_pos = pos;
            }
        }
        self.cursor.cur_cursor()
    }
}
