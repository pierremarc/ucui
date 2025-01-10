use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};
use shakmaty::{Move, Position};

use crate::{eco::find_eco, turn::Turn};

use super::{board::render_board, AppState};

fn render_hist(hist: &Vec<Move>, frame: &mut Frame, area: Rect) {
    let mut turn = Turn::new(hist);
    let mut lines = if let Some(eco) = find_eco(hist) {
        vec![
            Line::raw(eco.name).centered(),
            Line::raw(eco.code).centered(),
            Line::default(),
            Line::raw(turn.format_move()),
        ]
    } else {
        vec![Line::raw(turn.format_move())]
    };
    while turn.step() {
        lines.push(Line::raw(turn.format_move()));
    }
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("game")),
        area,
    );
}

pub fn render(app: &AppState, frame: &mut Frame, area: Rect) {
    let [area_hist, area_board] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Fill(1)]).areas(area);

    frame.render_widget(Block::bordered().title("chessboard"), area_board);
    render_hist(app.hist, frame, area_hist);
    render_board(app.game.board(), frame, area_board);
}
