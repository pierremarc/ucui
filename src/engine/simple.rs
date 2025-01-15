use std::thread;

use chrono::Utc;
use shakmaty::{
    fen::Fen, CastlingMode, Chess, Color, FromSetup, Move, MoveList, Outcome, Position, Role,
};

use crate::state::Store;

use super::Engine;

const DEPTH: usize = 3;
const MIN_KEEP: usize = 3;

enum ScoreResult {
    Score(i32, MoveList, Chess),
    Outcome(Outcome),
}

fn piece_score(color: Color, game: &Chess) -> i32 {
    let board = game.board();
    let bb = board.by_color(color);
    (bb.intersect(board.by_role(Role::Pawn)).count()
        + bb.intersect(board.by_role(Role::Bishop)).count() * 3
        + bb.intersect(board.by_role(Role::Knight)).count() * 3
        + bb.intersect(board.by_role(Role::Rook)).count() * 5
        + bb.intersect(board.by_role(Role::Queen)).count() * 9) as i32
}

fn score_move(color: Color, mut game: Chess, m: &Move, len: i32) -> ScoreResult {
    // log::debug!("score_move {}", m);
    // if !game.is_legal(&m) {
    //     log::error!(
    //         "Illegal move?? {} {}",
    //         Fen::from_position(game, shakmaty::EnPassantMode::Always),
    //         m
    //     );
    //     return ScoreResult::Outcome(Outcome::Decisive {
    //         winner: color.other(),
    //     });
    // }
    game.play_unchecked(m);
    if let Some(outcome) = game.outcome() {
        return ScoreResult::Outcome(outcome);
    }
    let legal_moves = game.legal_moves();
    // let capture_moves = game.capture_moves();
    let (us, them) = (piece_score(color, &game), piece_score(color.other(), &game));
    let count = legal_moves.len();
    ScoreResult::Score((len + us) - ((count as i32) + them), legal_moves, game)
}

fn eval_outcome(color: Color, outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Draw => 0,
        Outcome::Decisive { winner } => {
            if winner == color {
                i32::MAX
            } else {
                i32::MIN
            }
        }
    }
}

fn eval_move(color: Color, depth: usize, game: Chess, m: &Move, from: i32) -> i32 {
    // log::debug!("eval_move {}", m);

    match score_move(color, game.clone(), m, from) {
        ScoreResult::Outcome(outcome) => eval_outcome(color, outcome),
        ScoreResult::Score(score, oponent_moves, game) => {
            if depth == 0 {
                return score;
            }
            let mut variant_scores: Vec<i32> = Vec::new();
            for op_move in oponent_moves {
                let mut op_game = game.clone();

                op_game.play_unchecked(&op_move);
                let legal_moves = op_game.legal_moves();
                let len = legal_moves.len();
                let n_keep = MIN_KEEP; //std::cmp::max(MIN_KEEP, len / DEPTH);
                let mut scores: Vec<(Move, i32)> = Vec::with_capacity(len);
                for m in legal_moves {
                    match score_move(color, op_game.clone(), &m, len as i32) {
                        ScoreResult::Outcome(outcome) => {
                            scores.push((m, eval_outcome(color, outcome)))
                        }
                        ScoreResult::Score(score, _, _) => scores.push((m, score)),
                    }
                }
                scores.sort_by_key(|(_, s)| *s);
                variant_scores.push(
                    scores
                        .into_iter()
                        .rev()
                        .take(n_keep)
                        .map(|(m, _)| eval_move(color, depth - 1, op_game.clone(), &m, len as i32))
                        .max()
                        .unwrap_or(i32::MIN),
                );
            }
            variant_scores.into_iter().max().unwrap_or(i32::MIN)
        }
    }
}

// fn eval(color: Color, mut game: Chess, m: &Move) -> i32 {

// if depth == 0 {
//     game.play_unchecked(m);
//     game.legal_moves().len()
// } else {
//     let score: usize = {
//     game.play_unchecked(m);
//     game.legal_moves()
//     .iter()
//             .map(|m| eval(depth - 1, game.clone(), m, base))
//             .sum()
//     };
//     if depth == DEPTH {
//         log::debug!("{} {} ", m, base + score);
//     }
//     base + score
// }
// }

fn bestmove(game: Chess) -> Option<Move> {
    let color = game.turn();
    let moves = game.legal_moves();
    let len = moves.len();
    let fen = Fen::from_position(game.clone(), shakmaty::EnPassantMode::Always);
    log::debug!("bestmove {}", fen);
    if moves.len() == 0 {
        None
    } else {
        let scores: Vec<(&Move, i32)> = moves
            .iter()
            .map(|m| {
                let start = Utc::now();
                log::debug!("start-eval {}", m.to_uci(CastlingMode::Standard));
                let score = eval_move(color, DEPTH, game.clone(), m, len as i32);
                log::debug!("..............score = {} ", score);
                log::debug!(
                    "..............time  = {}",
                    (Utc::now() - start).num_milliseconds()
                );
                (m, score)
            })
            .collect();

        scores
            .into_iter()
            .reduce(|(am, ai), (m, i)| if i > ai { (m, i) } else { (am, ai) })
            .map(|(m, _)| m.clone())
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
                    store.update_engine(super::EngineState::PendingMove(mv));
                });
        });
    }
}

pub fn connect_engine(store: Store) -> SimpleEngine {
    SimpleEngine { store }
}
