use std::sync::mpsc::RecvError;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use shakmaty::Move;
mod uci;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum EngineState {
    #[default]
    Idle,
    Computing,
    PendingMove(Move),
    Move(Move),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum EngineCommand {
    Go {
        fen: String,
        white_time: Duration,
        black_time: Duration,
    },
    NewGame,
    Stop,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
pub enum EngineMessage {
    Id(String),
    BestMove(ucui_utils::MoveSerde),
}

pub trait Engine {
    fn name(&self) -> String;
    fn new_game(&self) {}
    fn stop(&self) {}
    fn go(&self, fen: String, white_time: Duration, black_time: Duration);
    fn recv(&self) -> Result<EngineMessage, RecvError>;
}

pub fn connect_engine(
    engine_path: &str,
    args: Option<Vec<String>>,
    options: Vec<(String, Option<String>)>,
) -> Box<dyn Engine + Send> {
    Box::new(uci::connect_engine(engine_path, args, options))
}
