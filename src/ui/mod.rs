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

pub const KEY_START_GAME: char = ' ';
pub const KEY_EXPORT_PGN: char = '1';
// pub const KEY_EXPORT_FEN: char = '2';

pub enum Screen {
    Home,
    Info,
    Play,
}

pub struct AppState<'a> {
    screen: Screen,
    game: &'a shakmaty::Chess,
    hist: &'a Vec<shakmaty::Move>,
    clock: &'a crate::clock::Clock,
    engine_move: &'a Option<shakmaty::Move>,
    engine_waiting: bool,
    avail_input: Option<usize>,
}

pub fn render(app: &AppState, frame: &mut ratatui::Frame) {
    match app.screen {
        Screen::Home => home::render(app, frame),
        Screen::Info => info::render(app, frame),
        Screen::Play => play::render(app, frame),
    }
}
