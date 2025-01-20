use std::cmp::Ordering;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use chrono::Duration;
use engine::{EngineMessage, MoveSerde};
/// Play endpoint
///
/// from https://docs.rs/axum/latest/axum/extract/ws/index.html
use serde::{Deserialize, Serialize};
use shakmaty::{fen::Fen, Chess, Color, Move, Outcome, Position, Square};

use crate::config::{get_engine, get_engine_args, get_engine_options};

struct GameState {
    game: Chess,
    engine: Box<dyn engine::Engine + Send>,
}

impl GameState {
    fn new() -> Self {
        Self {
            game: Chess::default(),
            engine: engine::connect_engine(&get_engine(), get_engine_args(), get_engine_options()),
        }
    }
}

// async fn handler(ws: WebSocketUpgrade, State(state): State<GameState>) -> Response {
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket))
}

fn sort_square(a: Square, b: Square) -> Ordering {
    if a.file() > b.file() {
        Ordering::Greater
    } else if a.file() < b.file() {
        Ordering::Less
    } else if a.rank() > b.rank() {
        Ordering::Greater
    } else if a.rank() < b.rank() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn sort_move(a: &Move, b: &Move) -> Ordering {
    if a == b {
        Ordering::Equal
    } else {
        match (a, b) {
            (Move::Put { .. }, Move::Put { .. }) => Ordering::Equal,
            (Move::Put { .. }, _) => Ordering::Less,
            (_, Move::Put { .. }) => Ordering::Greater,
            (a, b) => match sort_square(a.from().unwrap(), b.from().unwrap()) {
                Ordering::Equal => sort_square(a.to(), b.to()),
                ord => ord,
            },
        }
    }
}

async fn handle_socket(mut socket: WebSocket) {
    let mut state = GameState::new();
    let _ = socket.send(ServerMessage::ready()).await;
    loop {
        if state.game.turn() == Color::White {
            if let Some(pack) = socket.recv().await {
                match pack {
                    Err(_) => break,
                    Ok(msg) => match msg {
                        Message::Text(t) => {
                            if let Ok(ClientMessage::Move {
                                _move: ply,
                                white_time,
                                black_time,
                            }) = serde_json::from_str(t.as_str())
                            {
                                log::info!("Got a move");
                                let m: Move = ply.into();
                                let game = state.game.clone();
                                if let Ok(new_pos) = game.play(&m) {
                                    match new_pos.outcome() {
                                        Some(outcome) => {
                                            let _ =
                                                socket.send(ServerMessage::outcome(outcome)).await;
                                            break;
                                        }
                                        None => {
                                            state.game = new_pos.clone();
                                            state.engine.go(
                                                Fen::from_position(
                                                    new_pos,
                                                    shakmaty::EnPassantMode::Legal,
                                                )
                                                .to_string(),
                                                Duration::milliseconds(white_time),
                                                Duration::milliseconds(black_time),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                }
            } else {
                break;
            }
        } else {
            if let Ok(EngineMessage::BestMove(m)) = state.engine.recv() {
                let m: Move = m.into();
                state.game = state.game.clone().play(&m).unwrap();
                let _ = socket.send(ServerMessage::engine_move(m)).await;
                if let Some(outcome) = state.game.outcome() {
                    let _ = socket.send(ServerMessage::outcome(outcome)).await;
                    break;
                } else {
                    let mut moves = state.game.legal_moves();
                    moves.sort_by(sort_move);
                    let _ = socket
                        .send(ServerMessage::position(
                            moves.into_iter().map(|m| m.into()).collect(),
                        ))
                        .await;
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
enum ServerMessage {
    Ready,
    Position {
        #[serde(rename = "legalMoves")]
        legal_moves: Vec<engine::MoveSerde>,
    },
    EngineMove {
        #[serde(rename = "move")]
        _move: engine::MoveSerde,
    },
    Outcome {
        outcome: String,
    },
}

impl ServerMessage {
    fn ready() -> Message {
        Message::text(serde_json::to_string(&ServerMessage::Ready).unwrap())
    }

    fn position(legal_moves: Vec<MoveSerde>) -> Message {
        Message::text(serde_json::to_string(&ServerMessage::Position { legal_moves }).unwrap())
    }

    fn engine_move(m: Move) -> Message {
        Message::text(
            serde_json::to_string(&ServerMessage::EngineMove { _move: m.into() }).unwrap(),
        )
    }

    fn outcome(outcome: Outcome) -> Message {
        let o = match outcome {
            Outcome::Draw => "½-½",
            Outcome::Decisive { winner } => {
                if winner == Color::White {
                    "1-0"
                } else {
                    "0-1"
                }
            }
        };
        Message::text(serde_json::to_string(&ServerMessage::Outcome { outcome: o.into() }).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
enum ClientMessage {
    Move {
        #[serde(rename = "move")]
        _move: engine::MoveSerde,
        white_time: i64,
        black_time: i64,
    },
}
