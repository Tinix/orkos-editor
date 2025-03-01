mod editor;
mod input;
mod ui;

use crate::editor::Editor;
use crate::input::handle_input;
use crate::ui::draw_ui;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use ropey::Rope;
use std::io;

const APP_NAME: &str = "Orkos";

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut editor = Editor::new();

    let os_name = std::env::consts::OS;
    let system_info = format!("{} | Rust", os_name);

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Min(1),
                ])
                .split(size);

            let header = Paragraph::new("Press Ctrl+Q to exit")
                .block(Block::default().borders(Borders::ALL).title(APP_NAME))
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(header, chunks[0]);

            let subheader =
                Paragraph::new(system_info.clone()).style(Style::default().fg(Color::Yellow));
            frame.render_widget(subheader, chunks[1]);

            let text = editor.buffer.to_string();
            let editor_area =
                Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Orkos."));
            frame.render_widget(editor_area, chunks[2]);
        })?;

        if let event::Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Char(c) => editor.insert_char(c),
                KeyCode::Left => editor.move_cursor_left(),
                KeyCode::Right => editor.move_cursor_right(),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
