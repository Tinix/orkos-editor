use crate::editor::Editor;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame<'_>, editor: &Editor, system_info: &str) {
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
        .block(Block::default().borders(Borders::ALL).title("Orkos Editor"))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(header, chunks[0]);

    let subheader = Paragraph::new(system_info).style(Style::default().fg(Color::Yellow));
    frame.render_widget(subheader, chunks[1]);

    let text = editor.get_text();
    let editor_area =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Orkos"));
    frame.render_widget(editor_area, chunks[2]);
}
