use crate::config::{get_time_black, get_time_white};
use chrono::{DateTime, Duration, Utc};
use shakmaty::Color;
use std::sync::{Arc, Mutex};

// #[derive(Clone)]
pub struct Clock {
    white: i64,
    black: i64,
    max_time_white: Duration,
    max_time_black: Duration,
    state: ClockState,
    _timer: Option<(timer::Timer, timer::Guard)>,
}

pub type SharedClock = Arc<Mutex<Clock>>;

#[derive(Debug, Clone, Copy)]
pub enum ClockState {
    Initial,
    Running(Color, DateTime<Utc>),
    Flag(Color, Duration),
}

impl Clock {
    pub fn new() -> SharedClock {
        Arc::new(Mutex::new(Clock {
            white: 0,
            black: 0,
            max_time_white: Duration::seconds(get_time_white()),
            max_time_black: Duration::seconds(get_time_black()),
            state: ClockState::Initial,
            _timer: None,
        }))
    }

    pub fn start(self, color: Color) -> Option<SharedClock> {
        if let ClockState::Initial = self.state {
            let clock = Arc::new(Mutex::new(Clock {
                state: ClockState::Running(color, chrono::Utc::now()),
                ..self
            }));
            let cloned = clock.clone();
            let timer = timer::Timer::new();
            let guard = {
                timer.schedule_repeating(chrono::Duration::milliseconds(16), move || {
                    match cloned.lock() {
                        Err(_) => {}
                        Ok(mut clock) => {
                            clock.update_state();
                        }
                    };
                })
            };
            clock
                .lock()
                .map(|mut c| c._timer = Some((timer, guard)))
                .expect("failed to get clock");
            log::info!("Clock Started");
            Some(clock)
        } else {
            None
        }
    }

    pub fn clone(&self) -> Self {
        Clock {
            white: self.white.clone(),
            black: self.black.clone(),
            max_time_white: self.max_time_white.clone(),
            max_time_black: self.max_time_black.clone(),
            state: self.state.clone(),
            _timer: None,
        }
    }

    pub fn state(&self) -> ClockState {
        self.state
    }

    pub(self) fn update_state(&mut self) {
        if let ClockState::Running(turn, start_time) = self.state {
            let now = chrono::Utc::now();
            let total_spent = Duration::seconds(self.white + self.black);
            let total = now - start_time;
            let inc = total - total_spent;

            match turn {
                Color::White => self.white += inc.num_seconds(),
                Color::Black => self.black += inc.num_seconds(),
            }

            if Duration::seconds(self.black) >= self.max_time_black {
                self.state = ClockState::Flag(Color::Black, self.remaining(Color::White));
            };
            if Duration::seconds(self.white) >= self.max_time_white {
                self.state = ClockState::Flag(Color::White, self.remaining(Color::Black));
            };
        };
    }

    fn white(&self) -> i64 {
        std::cmp::min(self.max_time_white.num_seconds(), self.white)
    }

    fn black(&self) -> i64 {
        std::cmp::min(self.max_time_black.num_seconds(), self.black)
    }

    pub fn hit(&mut self) {
        if let ClockState::Running(turn, start_time) = self.state {
            log::info!("Clock::hit {:?} -> {:?}", turn, turn.other());
            self.state = ClockState::Running(turn.other(), start_time);
        }
    }

    pub fn remaining_seconds(&self, color: Color) -> i64 {
        match color {
            Color::White => self.max_time_white.num_seconds() - self.white(),
            Color::Black => self.max_time_black.num_seconds() - self.black(),
        }
    }

    fn remaining(&self, color: Color) -> Duration {
        match color {
            Color::White => self.max_time_white - Duration::seconds(self.white()),
            Color::Black => self.max_time_black - Duration::seconds(self.black()),
        }
    }

    pub fn format(&self) -> (String, String) {
        match self.state {
            ClockState::Initial => (String::from("--:--"), String::from("--:--")),
            ClockState::Flag(color, other_time) => match color {
                Color::White => (String::from("FLAG"), format_time(other_time)),
                Color::Black => (format_time(other_time), String::from("FLAG")),
            },
            ClockState::Running(_, _) => (
                format_time(self.remaining(Color::White)),
                format_time(self.remaining(Color::Black)),
            ),
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
