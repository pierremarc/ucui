use ratatui::{
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use super::AppState;

pub fn render(_: &AppState, frame: &mut Frame) {
    let area = frame.area();

    frame.render_widget(
        Paragraph::new(vec![Line::raw("Welcome to ucui, press <Space> to start.")])
            .centered()
            .block(Block::bordered().title("home")),
        area,
    );
}
