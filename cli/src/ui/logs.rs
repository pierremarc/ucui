use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::state::State;

pub fn render(state: &State, frame: &mut Frame, area: Rect) {
    let border = Block::bordered().title("Logs");
    let inner = border.inner(area);
    frame.render_widget(border, area);
    let lines: Vec<Line> = state
        .log
        .lines
        .iter()
        .rev()
        .take(inner.height as usize)
        .rev()
        .map(Line::raw)
        .collect();
    frame.render_widget(Paragraph::new(lines), inner);
}
