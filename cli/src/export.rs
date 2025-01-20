// use std::{fs::File, io::Write};

use shakmaty::{fen::Fen, Chess, Move, Position};

use crate::{config::get_start_pos, turn::Turn};

pub fn export_pgn(game: &Chess, move_list: &Vec<Move>) -> String {
    let now = chrono::Utc::now();
    let date_format = now.format("%Y.%m.%d");
    let mut turn = Turn::new(
        get_start_pos().unwrap_or_default(),
        move_list,
    )
    .seps(String::from(" "), String::from(" "))
    .without_outcome();

    let mut parts: Vec<String> = vec![];

    let headers = format!("[Event \"Me vs Engine\"]\n[Date \"{date_format}\"]\n");
    parts.push(headers);
    if let Some(outcome) = game.outcome() {
        let result = format!("[Result \"{outcome}\"]\n");
        parts.push(result);
    } else {
        parts.push("[Result \"*\"]\n".to_string());
    }

    if let Some(start_pos) = get_start_pos() {
        let fen = export_fen(&start_pos);
        parts.push(format!("[FEN \"{fen}\"]\n"));
        parts.push("[SetUp 1]\n".to_string());
    }

    let start = format!("\n{}", turn.format_move());
    parts.push(start);
    while turn.step() {
        parts.push(turn.format_move());
    }

    if let Some(outcome) = game.outcome() {
        parts.push(outcome.to_string());
    }

    parts.push(String::from("\n"));
    parts.join("")
}

pub fn export_fen(game: &Chess) -> String {
    Fen::from_position(game.clone(), shakmaty::EnPassantMode::Always).to_string()
}
