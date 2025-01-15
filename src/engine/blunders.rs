use std::{str::FromStr, sync::mpsc::channel, thread};

use blunders_engine::Fen as _;
use shakmaty::{Chess, FromSetup as _, Move};
use shakmaty_uci::{ParseUciMoveError, UciMove};

use crate::state::Store;

use super::Engine;

pub struct BlunderEngine {
    store: Store,
}

fn to_move(r: blunders_engine::SearchResult, game: &Chess) -> Option<Move> {
    let bm = r.best_move;
    UciMove::from_str(&format!("{}", bm))
        .and_then(|um| um.to_move(game).map_err(|_| ParseUciMoveError))
        .ok()

    // match best_move_uci.to_move(game) {
    //     Err(e) => log::error!(
    //         "Failed to produce a bestmove from {best_move_uci}: {} ({})",
    //         e,
    //         Fen::from_position(game.clone(), shakmaty::EnPassantMode::Always)
    //     ),
    //     Ok(m) => self.store.update_engine(EngineState::PendingMove(m)),
    // }
}

impl Engine for BlunderEngine {
    fn new_game(&self) {}

    fn go(
        &self,
        fen: shakmaty::fen::Fen,
        white_time: chrono::Duration,
        black_time: chrono::Duration,
    ) {
        // thread::spawn(move || {
        //     Chess::from_setup(fen.as_setup().clone(), shakmaty::CastlingMode::Standard)
        //         .ok()
        //         .and_then(bestmove)
        //         .map(|mv| {
        //             store.update_engine(super::EngineState::PendingMove(mv));
        //         });
        // });
        if let Ok(pos) = blunders_engine::Position::parse_fen(&fen.to_string()) {
            let (tx, rx) = channel::<blunders_engine::SearchResult>();
            let store = self.store.clone();
            let game = Chess::from_setup(fen.as_setup().clone(), shakmaty::CastlingMode::Standard)
                .unwrap();
            let mut engine = blunders_engine::EngineBuilder::new()
                .position(pos)
                .transpositions_mb(10)
                .debug(false)
                .build();
            let _ = engine.search(
                blunders_engine::Mode::standard(
                    white_time.num_milliseconds() as i32,
                    black_time.num_milliseconds() as i32,
                    None,
                    None,
                    None,
                    None,
                ),
                tx,
            );

            thread::spawn(move || {
                rx.recv().map(|result| {
                    let m =
                        to_move(result, &game).expect("blunders move failed to parse correctly");
                    store.update_engine(super::EngineState::PendingMove(m));
                })
            });
        }
    }
}

pub fn connect_engine(store: Store) -> BlunderEngine {
    BlunderEngine { store }
}
