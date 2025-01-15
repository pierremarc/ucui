use chrono::Duration;
use shakmaty::{fen::Fen, Move};

use crate::{config::get_engine, state::Store};

mod blunders;
// mod simple;
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

pub fn connect_engine(store: Store) -> Box<dyn Engine> {
    if let Some(engine_path) = get_engine() {
        Box::new(uci::connect_engine(&engine_path, store))
    } else {
        Box::new(blunders::connect_engine(store))
    }
}
