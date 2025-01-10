mod board;
mod footer;
mod home;
mod info;
mod play;

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
pub const KEY_EXPORT_PGN: char = '9';
pub const KEY_START_GAME: char = ' ';
// pub const KEY_EXPORT_FEN: char = '2';

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Home,
    Info,
    Play,
}

impl Screen {
    fn name(&self) -> &'static str {
        match self {
            Screen::Home => "Home",
            Screen::Info => "Info",
            Screen::Play => "Play",
        }
    }
}

pub struct AppState<'a> {
    pub screen: Screen,
    pub game: &'a shakmaty::Chess,
    pub hist: &'a Vec<shakmaty::Move>,
    pub clock: &'a crate::clock::Clock,
    pub engine_move: &'a Option<shakmaty::Move>,
    pub engine_waiting: bool,
    pub avail_input: Option<usize>,
}

pub fn render(app: &AppState, frame: &mut ratatui::Frame) {
    let area = footer::render(&app.screen, frame);
    match app.screen {
        Screen::Home => home::render(app, frame, area),
        Screen::Info => info::render(app, frame, area),
        Screen::Play => play::render(app, frame, area),
    }
}
