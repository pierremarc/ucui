use chrono::{DateTime, Duration, Utc};
use shakmaty::Color;

#[derive(Debug)]
pub struct Clock {
    pub white: i64,
    pub black: i64,
    start_time: Option<DateTime<Utc>>,
    max_time: Duration,
    running: Color,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            white: 0,
            black: 0,
            start_time: None,
            max_time: Duration::minutes(20),
            running: Color::White,
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(chrono::Utc::now());
        }
    }

    pub fn hit(&mut self) {
        if let Some(start_time) = self.start_time {
            let now = chrono::Utc::now();
            let total_spent = Duration::seconds(self.white + self.black);
            let total = now - start_time;
            let inc = total - total_spent;

            match self.running {
                Color::Black => self.black += inc.num_seconds(),
                Color::White => self.white += inc.num_seconds(),
            }
            self.running = self.running.other();
            print!("{}", 0x07 as char);
            // println!("<HIT> {}", self.running.other());
            // println!("now\t= {now}");
            // println!("total_spent\t= {total_spent}");
            // println!("total\t= {total}");
            // println!("inc\t= {inc}");
        }
    }

    pub fn remaining_for(&self, color: Color) -> i64 {
        match color {
            Color::White => self.max_time.num_seconds() - self.white,
            Color::Black => self.max_time.num_seconds() - self.black,
        }
    }

    fn time_for(&self, color: Color, turn: Color) -> Duration {
        match self.start_time {
            None => Duration::zero(),
            Some(start_time) => {
                let now = chrono::Utc::now();
                let dw = Duration::seconds(self.white);
                let db = Duration::seconds(self.black);
                let spent = dw + db;
                let inc = (now - start_time) - spent;

                match (color, turn) {
                    (Color::Black, Color::Black) => db + inc,
                    (Color::Black, Color::White) => db,
                    (Color::White, Color::Black) => dw,
                    (Color::White, Color::White) => dw + inc,
                }
            }
        }
    }

    pub fn format(&self, color: Color, turn: Color) -> String {
        let t = self.max_time - self.time_for(color, turn);
        let h = t.num_hours();
        let m = t.num_minutes() % 60;
        let s = t.num_seconds() % 60;
        if h > 0 {
            format!("{:02}:{:02}:{:02}", h, m, s)
        } else {
            format!("{:02}:{:02}", m, s)
        }
    }
}
