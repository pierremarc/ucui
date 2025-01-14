use std::thread;

use shakmaty::{Chess, FromSetup, Move, Position};

use crate::state::Store;

use super::Engine;

fn tot_mov<P>(depth: usize, mut game: P, m: &Move, base: usize) -> usize
where
    P: Position + Clone,
{
    // let mut game = game.clone();
    if depth == 0 {
        game.play_unchecked(m);
        game.legal_moves().len()
    } else {
        let score: usize = {
            game.play_unchecked(m);
            game.legal_moves()
                .iter()
                .map(|m| tot_mov(depth - 1, game.clone(), m, base))
                .sum()
        };
        base + score
    }
}

fn bestmove<P>(game: P) -> Option<Move>
where
    P: Position + Clone,
{
    let moves = game.legal_moves();
    if moves.len() == 0 {
        None
    } else {
        let scores: Vec<(Move, usize)> = moves
            .iter()
            .map(|m| (m.clone(), tot_mov(6, game.clone(), m, 0)))
            .collect();

        scores
            .into_iter()
            .reduce(|(am, ai), (m, i)| if i > ai { (m, i) } else { (am, ai) })
            .map(|(m, _)| m)
    }
}

pub struct SimpleEngine {
    store: Store,
}

impl Engine for SimpleEngine {
    fn new_game(&self) {}

    fn go(
        &self,
        fen: shakmaty::fen::Fen,
        _white_time: chrono::Duration,
        _black_time: chrono::Duration,
    ) {
        // let (tx, rx) = channel::<Move>();
        let store = self.store.clone();
        thread::spawn(move || {
            Chess::from_setup(fen.as_setup().clone(), shakmaty::CastlingMode::Standard)
                .ok()
                .and_then(bestmove)
                .map(|mv| {
                    store.update_engine(super::EngineState::Move(mv));
                });
        });
    }
}

pub fn connect_engine(store: Store) -> SimpleEngine {
    SimpleEngine { store }
}
