mod board;
mod event;
mod footer;
mod home;
mod info;
mod input;
mod logs;
mod play;
pub use event::handle_key_event;

use crate::state::State;

pub const WHITE_PAWN: &str = "♙";
pub const WHITE_ROOK: &str = "♖";
pub const WHITE_KNIGHT: &str = "♘";
pub const WHITE_BISHOP: &str = "♗";
pub const WHITE_QUEEN: &str = "♕";
pub const WHITE_KING: &str = "♔";

pub const BLACK_PAWN: &str = "♟";
pub const BLACK_ROOK: &str = "♜";
pub const BLACK_KNIGHT: &str = "♞";
pub const BLACK_BISHOP: &str = "♝";
pub const BLACK_QUEEN: &str = "♛";
pub const BLACK_KING: &str = "♚";

pub const KEY_GO_HOME: char = '1';
pub const KEY_GO_INFO: char = '2';
pub const KEY_GO_PLAY: char = '3';
pub const KEY_GO_LOGS: char = '4';
pub const KEY_EXPORT_PGN: char = 'p';
pub const KEY_EXPORT_FEN: char = 'f';
pub const KEY_START_GAME: char = ' ';

#[derive(Clone, Eq, PartialEq, Debug, Copy, Default)]
pub enum Screen {
    #[default]
    Home,
    Info,
    Play,
    Log,
}

impl Screen {
    fn name(&self) -> &'static str {
        match self {
            Screen::Home => "Home",
            Screen::Info => "Info",
            Screen::Play => "Play",
            Screen::Log => "Log",
        }
    }
}

pub fn render(app: &State, frame: &mut ratatui::Frame) {
    let area = footer::render(&app.screen, frame);
    match app.screen {
        Screen::Home => home::render(app, frame, area),
        Screen::Info => info::render(app, frame, area),
        Screen::Play => play::render(app, frame, area),
        Screen::Log => logs::render(app, frame, area),
    }
}
