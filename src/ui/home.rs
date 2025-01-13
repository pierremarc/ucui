use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{state::State, util::shrink_rect};

pub fn render(_: &State, frame: &mut Frame, area: Rect) {
    frame.render_widget(Block::bordered().title("UCUI"), area);
    frame.render_widget(
        Paragraph::new(vec![Line::raw("Welcome to ucui, press <Space> to start.")]).centered(),
        shrink_rect(area, crate::util::PaddingMod::Top(area.height / 2)),
    );
}
