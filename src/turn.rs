use shakmaty::{Chess, Move, Position};

use crate::util::san_format_move;

pub struct Turn<'a> {
    c: Chess,
    ml: &'a Vec<Move>,
    i: usize,
    sep0: String,
    sep1: String,
}

impl<'a> Turn<'a> {
    pub fn new(hist: &'a Vec<Move>) -> Self {
        Turn {
            ml: hist,
            c: Chess::new(),
            i: 0,
            sep0: String::from("\t"),
            sep1: String::from("\n\t"),
        }
    }

    pub fn seps(self, sep0: String, sep1: String) -> Self {
        Turn {
            ml: self.ml,
            c: self.c,
            i: self.i,
            sep0,
            sep1,
        }
    }

    pub fn with_outcome(&self, line: &str) -> String {
        if let Some(outcome) = self.c.outcome() {
            format!("{}{}{}", line, self.sep1, outcome)
        } else {
            String::from(line)
        }
    }
    pub fn format_move(&self) -> String {
        let wm = self.ml.get(self.i);
        let bm = self.ml.get(self.i + 1);
        let n = (self.i / 2) + 1;
        match (wm, bm) {
            (Some(w), Some(b)) => {
                let np = self.c.clone().play(w).expect("turn move should be OK");
                self.with_outcome(&format!(
                    "{:>3}. {}{}{}",
                    n,
                    san_format_move(&self.c, w, false),
                    self.sep0,
                    san_format_move(&np, b, false)
                ))
            }
            (Some(w), None) => {
                self.with_outcome(&format!("{:>3}. {}", n, san_format_move(&self.c, w, false)))
            }
            _ => self.with_outcome(""),
        }
    }

    pub fn step(&mut self) -> bool {
        if self.c.outcome().is_some() {
            false
        } else {
            let wm = self.ml.get(self.i);
            let bm = self.ml.get(self.i + 1);
            match (wm, bm) {
                (_, None) => false,
                (Some(w), Some(b)) => {
                    self.c = self.c.clone().play(w).expect("white move should be ok");
                    self.c = self.c.clone().play(b).expect("black move should be ok");
                    self.i += 2;
                    true
                }
                _ => panic!("that cannot be"),
            }
        }
    }
}
