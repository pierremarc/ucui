use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::util::shrink_rect;

use super::AppState;

pub fn render(_: &AppState, frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::bordered().title("UCUI"), area);
    frame.render_widget(
        Paragraph::new(vec![Line::raw("Welcome to ucui, press <Space> to start.")]).centered(),
        shrink_rect(area, crate::util::PaddingMod::Top(area.height / 2)),
    );
}
