use std::str::FromStr as _;

use axum::{extract::Query, Json};
use serde::Deserialize;
use shakmaty::{fen::Fen, Chess, FromSetup, Position};
use ucui_eco::{lookup_eco_from_name, Eco};
use ucui_utils::MoveSerde;

#[derive(Deserialize)]
pub struct Lookup {
    term: String,
}

pub async fn lookup_eco(Query(lookup): Query<Lookup>) -> Json<Vec<Eco>> {
    Json(lookup_eco_from_name(lookup.term.trim()))
}

#[derive(Deserialize)]
pub struct Pos {
    fen: String,
}

pub async fn legal_moves(Query(pos): Query<Pos>) -> Json<Vec<MoveSerde>> {
    // Fen::from_position(state.game.clone(), shakmaty::EnPassantMode::Legal).to_string()
    if let Ok(fen) = Fen::from_str(&pos.fen) {
        if let Ok(game) = Chess::from_setup(fen.into_setup(), shakmaty::CastlingMode::Standard) {
            return Json(
                game.legal_moves()
                    .into_iter()
                    .map(MoveSerde::from)
                    .collect(),
            );
        }
    }
    Json(Vec::new())
}
