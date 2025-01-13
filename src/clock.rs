use crate::{
    config::{get_time_black, get_time_white},
    state::Gateway,
};
use chrono::{DateTime, Duration, Utc};
use shakmaty::Color;
use std::sync::{Arc, Mutex};

pub struct Clock {
    white: Duration,
    black: Duration,
    max_time_white: Duration,
    max_time_black: Duration,
    state: ClockState,
    _timer: Option<(timer::Timer, timer::Guard)>,
}

pub type SharedClock = Arc<Mutex<Clock>>;

#[derive(Clone, Eq, PartialEq, Debug, Copy, Default)]
pub enum ClockState {
    #[default]
    Initial,
    Running {
        turn: Color,
        start_time: DateTime<Utc>,
        remaining_white: Duration,
        remaining_black: Duration,
    },
    Flag(
        Color,    // fallen color
        Duration, // other's time
    ),
}

impl ClockState {
    pub fn format(&self) -> (String, String) {
        match self {
            ClockState::Initial => (String::from("--:--"), String::from("--:--")),
            ClockState::Flag(color, other_time) => match color {
                Color::White => (String::from("FLAG"), format_time(other_time)),
                Color::Black => (format_time(other_time), String::from("FLAG")),
            },
            ClockState::Running {
                remaining_white,
                remaining_black,
                ..
            } => (format_time(remaining_white), format_time(remaining_black)),
        }
    }
}

impl Clock {
    pub fn new() -> SharedClock {
        Arc::new(Mutex::new(Clock {
            white: Duration::zero(),
            black: Duration::zero(),
            max_time_white: Duration::seconds(get_time_white()),
            max_time_black: Duration::seconds(get_time_black()),
            state: ClockState::Initial,
            _timer: None,
        }))
    }

    pub fn start(self, color: Color, store: Gateway) -> Option<SharedClock> {
        if let ClockState::Initial = self.state {
            let now = chrono::Utc::now();
            let clock = Arc::new(Mutex::new(Clock {
                state: ClockState::Running {
                    turn: color,
                    start_time: now,
                    remaining_white: self.max_time_white,
                    remaining_black: self.max_time_black,
                },
                ..self
            }));
            let cloned = clock.clone();
            let timer = timer::Timer::new();
            let guard = {
                timer.schedule_repeating(chrono::Duration::milliseconds(500), move || {
                    match cloned.lock() {
                        Err(_) => {}
                        Ok(mut clock) => {
                            store.update_clock(clock.update_state());
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
            white: self.white,
            black: self.black,
            max_time_white: self.max_time_white,
            max_time_black: self.max_time_black,
            state: self.state,
            _timer: None,
        }
    }

    pub fn state(&self) -> ClockState {
        self.state
    }

    pub(self) fn update_state(&mut self) -> ClockState {
        if let ClockState::Running {
            turn, start_time, ..
        } = self.state
        {
            let now = chrono::Utc::now();
            let total_spent = self.white + self.black;
            let total = now - start_time;
            let inc = total - total_spent;

            match turn {
                Color::White => self.white += inc,
                Color::Black => self.black += inc,
            }

            if self.black >= self.max_time_black {
                self.state = ClockState::Flag(Color::Black, self.remaining(Color::White));
            } else if self.white >= self.max_time_white {
                self.state = ClockState::Flag(Color::White, self.remaining(Color::Black));
            } else {
                self.state = ClockState::Running {
                    turn,
                    start_time,
                    remaining_white: self.remaining(Color::White),
                    remaining_black: self.remaining(Color::Black),
                }
            }
        };
        self.state
    }

    fn white(&self) -> Duration {
        std::cmp::min(self.max_time_white, self.white)
    }

    fn black(&self) -> Duration {
        std::cmp::min(self.max_time_black, self.black)
    }

    pub fn hit(&mut self) {
        if let ClockState::Running {
            turn, start_time, ..
        } = self.state
        {
            log::info!("Clock::hit {:?} -> {:?}", turn, turn.other());
            self.state = ClockState::Running {
                turn: turn.other(),
                start_time,
                remaining_white: self.remaining(Color::White),
                remaining_black: self.remaining(Color::Black),
            };
        }
    }

    pub fn remaining(&self, color: Color) -> Duration {
        match color {
            Color::White => self.max_time_white - self.white(),
            Color::Black => self.max_time_black - self.black(),
        }
    }
}

fn format_time(t: &Duration) -> String {
    let h = t.num_hours();
    let m = t.num_minutes() % 60;
    let s = t.num_seconds() % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}
