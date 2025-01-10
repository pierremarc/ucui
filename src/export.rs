use std::{fs::File, io::Write};

use shakmaty::{Chess, Move, Position};

use crate::turn::Turn;

pub fn export_pgn(game: &Chess, move_list: &Vec<Move>) {
    let now = chrono::Utc::now();
    let format1 = now.format("%Y-%m-%d-%H%M");
    let format2 = now.format("%Y.%m.%d");
    let p = format!("game-{}.pgn", format1);
    let path = std::path::Path::new(&p);
    let mut turn = Turn::new(move_list).seps(String::from(" "), String::from(" "));

    let mut file = File::create(path).expect("create file?");

    let headers = format!("[Event \"Me vs Engine\"]\n[Date \"{format2}\"]\n");
    file.write_all(headers.as_bytes()).expect("write headers");
    if let Some(outcome) = game.outcome() {
        let result = format!("[Result \"{outcome}\"]\n");
        file.write_all(result.as_bytes()).expect("ouch");
    }
    let start = format!("\n{}", turn.format_move());
    file.write_all(start.as_bytes()).expect("ouch");
    while turn.step() {
        let t = turn.format_move();
        file.write_all(t.as_bytes()).expect("ouch");
    }

    file.write_all("\n".as_bytes()).expect("ouch");
}
