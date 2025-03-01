use ropey::Rope;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
}

pub struct Editor {
    pub buffer: Rope,
    pub cursor_pos: usize,
    pub mode: EditorMode,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Rope::new(),
            cursor_pos: 0,
            mode: EditorMode::Normal,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.mode == EditorMode::Insert {
            self.buffer.insert_char(self.cursor_pos, ch);
            self.cursor_pos += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.buffer.len_chars() {
            self.cursor_pos += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.buffer.remove(self.cursor_pos - 1..self.cursor_pos);
            self.cursor_pos -= 1;
        }
    }

    pub fn insert_newline(&mut self) {
        if self.mode == EditorMode::Insert {
            self.buffer.insert_char(self.cursor_pos, '\n');
            self.cursor_pos += 1;
        }
    }

    pub fn get_text(&self) -> String {
        self.buffer.to_string()
    }
}
