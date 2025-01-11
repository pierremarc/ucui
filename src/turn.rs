use std::num::NonZeroU32;

use shakmaty::{Chess, Color, Move, Position};

use crate::util::san_format_move;

#[derive(Clone)]
pub struct Turn<'a> {
    game: Chess,
    move_list: &'a Vec<Move>,
    start_turn: Color,
    index: usize,
    sep_move: String,
    sep_outcome: String,
    bother_outcome: bool,
}

impl<'a> Turn<'a> {
    pub fn new(game: Chess, move_list: &'a Vec<Move>) -> Self {
        let start_turn = game.turn();
        Turn {
            move_list,
            game,
            start_turn,
            index: 0,
            sep_move: String::from("\t"),
            sep_outcome: String::from("\n\t"),
            bother_outcome: true,
        }
    }

    pub fn seps(self, sep0: String, sep1: String) -> Self {
        Turn {
            sep_move: sep0,
            sep_outcome: sep1,
            ..self
        }
    }
    pub fn without_outcome(self) -> Self {
        Turn {
            bother_outcome: false,
            ..self
        }
    }

    pub fn with_outcome(&self, line: &str) -> String {
        match (self.game.outcome(), self.bother_outcome) {
            (Some(outcome), true) => format!("{}{}{}", line, self.sep_outcome, outcome),
            _ => String::from(line),
        }
    }

    fn get_moves(&self) -> (NonZeroU32, Option<&Move>, Option<&Move>) {
        let fullmoves = self.game.fullmoves();

        match (self.start_turn, self.index) {
            (Color::Black, 0) => (fullmoves, None, self.move_list.get(self.index)),
            (_, _) => (
                fullmoves,
                self.move_list.get(self.index),
                self.move_list.get(self.index + 1),
            ),
        }
    }

    pub fn format_move(&self) -> String {
        let (n, wm, bm) = self.get_moves();
        match (wm, bm) {
            (Some(w), Some(b)) => {
                let game = self.game.clone();
                if !game.is_legal(w) {
                    return format!("{:>3}. !legal {}", n, w.to_string());
                }
                let np = game.play(w).expect("turn move should be OK");
                self.with_outcome(&format!(
                    "{:>3}. {}{}{}",
                    n,
                    san_format_move(&self.game, w, false),
                    self.sep_move,
                    san_format_move(&np, b, false)
                ))
            }
            (Some(w), None) => self.with_outcome(&format!(
                "{:>3}. {}",
                n,
                san_format_move(&self.game, w, false)
            )),
            (None, Some(b)) => self.with_outcome(&format!(
                "{:>3}... {}",
                n,
                san_format_move(&self.game, b, false)
            )),
            _ => self.with_outcome(""),
        }
    }

    pub fn step(&mut self) -> bool {
        if self.game.outcome().is_some() {
            false
        } else {
            let cloned = self.clone();
            let (_, wm, bm) = cloned.get_moves();
            match (wm, bm) {
                (_, None) => false,
                (Some(w), Some(b)) => {
                    self.game = self.game.clone().play(w).expect("white move should be ok");
                    self.game = self.game.clone().play(b).expect("black move should be ok");
                    self.index += 2;
                    true
                }
                (None, Some(b)) => {
                    self.game = self.game.clone().play(b).expect("black move should be ok");
                    self.index += 1;
                    true
                }
            }
        }
    }
}
