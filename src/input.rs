use crate::editor::{Editor, EditorMode};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::io;

pub fn handle_input(editor: &mut Editor) -> Result<bool, io::Error> {
    if let event::Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        match code {
            KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
            KeyCode::Char('i') if editor.mode == EditorMode::Normal => {
                editor.mode = EditorMode::Insert;
            }
            KeyCode::Esc if editor.mode == EditorMode::Insert => {
                editor.mode = EditorMode::Normal;
            }
            KeyCode::Char(c) => editor.insert_char(c),
            KeyCode::Enter => editor.insert_newline(),
            KeyCode::Backspace => editor.delete_char(),
            KeyCode::Left => editor.move_cursor_left(),
            KeyCode::Right => editor.move_cursor_right(),
            _ => {}
        }
    }
    Ok(false)
}
