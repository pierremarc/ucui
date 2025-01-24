use std::{cmp::Ordering, str::FromStr};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use chrono::Duration;
use engine::EngineMessage;
/// Play endpoint
///
/// from https://docs.rs/axum/latest/axum/extract/ws/index.html
use serde::{Deserialize, Serialize};
use shakmaty::{fen::Fen, Chess, Color, FromSetup, Move, Outcome, Position, Square};

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

    fn set_position(&mut self, fen_str: &str) {
        if let Ok(fen) = Fen::from_str(fen_str) {
            if let Ok(game) = Chess::from_setup(fen.into_setup(), shakmaty::CastlingMode::Standard)
            {
                self.game = game;
            }
        }
    }
}

// async fn handler(ws: WebSocketUpgrade, State(state): State<GameState>) -> Response {
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
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

async fn send_position(state: &mut GameState, socket: &mut WebSocket) {
    let mut moves = state.game.legal_moves();
    moves.sort_by(sort_move);
    let _ = socket
        .send(ServerMessage::position(
            moves.into_iter().map(|m| m.into()).collect(),
            Fen::from_position(state.game.clone(), shakmaty::EnPassantMode::Legal).to_string(),
        ))
        .await;
}

async fn play_position(
    game: Chess,
    state: &mut GameState,
    socket: &mut WebSocket,
    white_time: i64,
    black_time: i64,
) -> bool {
    match game.outcome() {
        Some(outcome) => {
            let _ = socket.send(ServerMessage::outcome(outcome)).await;
            return true;
        }
        None => {
            state.game = game.clone();
            state.engine.go(
                Fen::from_position(game, shakmaty::EnPassantMode::Legal).to_string(),
                Duration::milliseconds(white_time),
                Duration::milliseconds(black_time),
            );
        }
    }
    false
}

async fn handle_incoming_message(
    msg: Message,
    state: &mut GameState,
    socket: &mut WebSocket,
) -> bool {
    if let Message::Text(text) = msg {
        log::info!("handle_incoming_message: {}", &text);
        match serde_json::from_str(text.as_str()) {
            Ok(ClientMessage::Move {
                _move: ply,
                white_time,
                black_time,
            }) => {
                let m: Move = ply.into();
                let game = state.game.clone();
                if let Ok(new_pos) = game.play(&m) {
                    return play_position(new_pos, state, socket, white_time, black_time).await;
                }
            }
            Ok(ClientMessage::Position {
                fen,
                white_time,
                black_time,
            }) => {
                state.set_position(&fen);
                log::info!("Got a starting position: {} ", &fen);
                log::info!("Half moves: {} ", &state.game.halfmoves());
                if state.game.turn() == Color::Black {
                    log::info!("My turn");
                    return play_position(
                        state.game.clone(),
                        state,
                        socket,
                        white_time,
                        black_time,
                    )
                    .await;
                } else {
                    send_position(state, socket).await;
                }
            }
            _ => {
                log::warn!("incoming_message failed to parse '{text}'")
            }
        }
    }

    false
}

async fn handle_socket(mut socket: WebSocket) {
    let mut state = GameState::new();
    let _ = socket.send(ServerMessage::ready(state.engine.name())).await;
    loop {
        if state.game.turn() == Color::White {
            if let Some(pack) = socket.recv().await {
                match pack {
                    Err(_) => break,
                    Ok(msg) => {
                        if handle_incoming_message(msg, &mut state, &mut socket).await {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        } else if let Ok(EngineMessage::BestMove(m)) = state.engine.recv() {
            let m: Move = m.into();
            let from: Vec<ucui_utils::MoveSerde> = state
                .game
                .legal_moves()
                .into_iter()
                .map(ucui_utils::MoveSerde::from)
                .collect();
            state.game = state.game.clone().play(&m).unwrap();
            let status = if state.game.is_check() {
                "+"
            } else if state.game.is_checkmate() {
                "#"
            } else {
                ""
            };
            let _ = socket
                .send(ServerMessage::engine_move(m, from, status.into()))
                .await;
            if let Some(outcome) = state.game.outcome() {
                let _ = socket.send(ServerMessage::outcome(outcome)).await;
                break;
            } else {
                send_position(&mut state, &mut socket).await;
            }
        }
    }
    log::info!("End Of Socket");
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_tag")]
enum ServerMessage {
    Ready {
        name: String,
    },
    Position {
        #[serde(rename = "legalMoves")]
        legal_moves: Vec<ucui_utils::MoveSerde>,
        fen: String,
    },
    EngineMove {
        #[serde(rename = "move")]
        _move: ucui_utils::MoveSerde,
        from: Vec<ucui_utils::MoveSerde>,
        status: String,
    },
    Outcome {
        outcome: String,
    },
}

impl ServerMessage {
    fn ready(name: String) -> Message {
        Message::text(serde_json::to_string(&ServerMessage::Ready { name }).unwrap())
    }

    fn position(legal_moves: Vec<ucui_utils::MoveSerde>, fen: String) -> Message {
        Message::text(serde_json::to_string(&ServerMessage::Position { legal_moves, fen }).unwrap())
    }

    fn engine_move(m: Move, from: Vec<ucui_utils::MoveSerde>, status: String) -> Message {
        Message::text(
            serde_json::to_string(&ServerMessage::EngineMove {
                _move: m.into(),
                from,
                status,
            })
            .unwrap(),
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
        _move: ucui_utils::MoveSerde,
        white_time: i64,
        black_time: i64,
    },
    Position {
        fen: String,
        white_time: i64,
        black_time: i64,
    },
}
