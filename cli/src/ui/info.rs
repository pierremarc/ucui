use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};
use shakmaty::{Chess, Move, Position};

use crate::{config::get_start_pos, eco::find_eco, state::State, turn::Turn};

use super::board::render_board;

fn render_hist(hist: &Vec<Move>, frame: &mut Frame, area: Rect) {
    let mut turn = match get_start_pos() {
        Some(pos) => Turn::new(pos, hist),
        None => Turn::new(Chess::default(), hist),
    };
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

pub fn render(state: &State, frame: &mut Frame, area: Rect) {
    let [area_hist, area_board] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Fill(1)]).areas(area);

    frame.render_widget(Block::bordered().title("Board"), area_board);
    render_hist(&state.hist, frame, area_hist);
    render_board(state.game().board(), frame, area_board);
}
