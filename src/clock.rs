use chrono::{DateTime, Duration, Utc};
use shakmaty::Color;

use crate::config::{get_time_black, get_time_white};

#[derive(Debug)]
pub struct Clock {
    white: i64,
    black: i64,
    // start_time: Option<DateTime<Utc>>,
    max_time_white: Duration,
    max_time_black: Duration,
    // running: Color,
    state: ClockState,
}

#[derive(Debug, Clone, Copy)]
pub enum ClockState {
    Initial,
    Running(Color, DateTime<Utc>),
    Flag(Color),
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            white: 0,
            black: 0,
            // start_time: None,
            max_time_white: Duration::seconds(get_time_white()),
            max_time_black: Duration::seconds(get_time_black()),
            // running: Color::White,
            state: ClockState::Initial,
        }
    }

    pub fn check_state(&mut self) -> ClockState {
        if let ClockState::Running(turn, start_time) = self.state {
            let now = chrono::Utc::now();

            let wt = self.spent_for_at_time(start_time, Color::White, turn, now);
            let bt = self.spent_for_at_time(start_time, Color::Black, turn, now);
            if bt > self.max_time_black {
                self.state = ClockState::Flag(Color::Black)
            };
            if wt > self.max_time_white {
                self.state = ClockState::Flag(Color::White)
            };
        };
        self.state
    }

    pub fn white(&self) -> i64 {
        std::cmp::min(self.max_time_white.num_seconds(), self.white)
    }

    pub fn black(&self) -> i64 {
        std::cmp::min(self.max_time_black.num_seconds(), self.black)
    }

    pub fn start(&mut self, color: Color) {
        if let ClockState::Initial = self.check_state() {
            self.state = ClockState::Running(color, chrono::Utc::now());
        }
    }

    pub fn hit(&mut self) {
        if let ClockState::Running(turn, start_time) = self.check_state() {
            let now = chrono::Utc::now();
            let total_spent = Duration::seconds(self.white + self.black);
            let total = now - start_time;
            let inc = total - total_spent;

            match turn {
                Color::Black => self.black += inc.num_seconds(),
                Color::White => self.white += inc.num_seconds(),
            }
            self.state = ClockState::Running(turn.other(), start_time);
        }
    }

    pub fn remaining_for_uci(&self, color: Color) -> i64 {
        match color {
            Color::White => self.max_time_white.num_seconds() - self.white(),
            Color::Black => self.max_time_black.num_seconds() - self.black(),
        }
    }

    fn spent_for_at_time(
        &self,
        start_time: DateTime<Utc>,
        color: Color,
        turn: Color,
        now: DateTime<Utc>,
    ) -> Duration {
        let dw = Duration::seconds(self.white());
        let db = Duration::seconds(self.black());
        let spent = dw + db;
        let inc = (now - start_time) - spent;

        match (color, turn) {
            (Color::Black, Color::Black) => db + inc,
            (Color::Black, Color::White) => db,
            (Color::White, Color::Black) => dw,
            (Color::White, Color::White) => dw + inc,
        }
    }

    pub fn format(&self) -> (String, String) {
        match self.state {
            ClockState::Initial => (String::from("--:--"), String::from("--:--")),
            ClockState::Flag(color) => match color {
                Color::White => (String::from("FLAG!"), String::from("++:++")),
                Color::Black => (String::from("++:++"), String::from("FLAG!")),
            },
            ClockState::Running(running, start_time) => {
                let now = chrono::Utc::now();
                let wt = self.max_time_white
                    - self.spent_for_at_time(start_time, Color::White, running, now);
                let bt = self.max_time_black
                    - self.spent_for_at_time(start_time, Color::Black, running, now);
                (format_time(wt), format_time(bt))
            }
        }
    }
}

fn format_time(t: Duration) -> String {
    let h = t.num_hours();
    let m = t.num_minutes() % 60;
    let s = t.num_seconds() % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}
