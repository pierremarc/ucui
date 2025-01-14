use std::sync::mpsc::Sender;

use chrono::Duration;
use shakmaty::{fen::Fen, Move};

mod simple;
mod uci;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum EngineState {
    #[default]
    Idle,
    Computing,
    PendingMove(Move),
    Move(Move),
}

pub enum EngineMessage {
    Go {
        fen: Fen,
        white_time: Duration,
        black_time: Duration,
    },
    NewGame,
}

pub trait Engine {
    fn new_game(&self);

    fn go(&self, fen: Fen, white_time: Duration, black_time: Duration);
}
