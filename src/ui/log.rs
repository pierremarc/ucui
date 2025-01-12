use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use super::AppState;

pub fn render(app: &AppState, frame: &mut Frame, area: Rect) {
    let border = Block::bordered().title("Logs");
    let inner = border.inner(area);
    frame.render_widget(border, area);
    let lines: Vec<Line> = app
        .log
        .lines
        .iter()
        .rev()
        .take(area.height as usize)
        .map(Line::raw)
        .collect();
    frame.render_widget(Paragraph::new(lines), inner);
}
