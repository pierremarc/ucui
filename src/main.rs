use std::io;

use chrono::Duration;
use clock::Clock;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use engine::{connect_engine, EngineConnection};
use ratatui::{DefaultTerminal, Frame};
use shakmaty::{Chess, Color, Move, Position};
use ui::render_main;
use util::alpha_to_i;
// use seek::seek;
// use seek::Week;

mod clock;
mod engine;
mod ui;
mod util;

fn main() -> io::Result<()> {
    let _ = start_app();
    Ok(())
}

fn start_app() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    t: chrono::DateTime<chrono::Utc>,
    game: Chess,
    hist: Vec<Move>,
    clock: Clock,
    input_move: Option<String>,
    connection: EngineConnection,
    engine_move: Option<Move>,
}
impl App {
    fn new() -> Self {
        App {
            exit: false,
            t: chrono::Utc::now(),
            game: Chess::new(),
            hist: Vec::new(),
            clock: Clock::new(),
            input_move: None,
            connection: connect_engine(),
            engine_move: None,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                // It's guaranteed that `read` won't block, because `poll` returned
                // `Ok(true)`.
                match event::read()? {
                    // it's important to check that the event is a key press event as
                    // crossterm also emits key release and repeat events on Windows.
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    _ => {}
                };
            } else {
                let now = chrono::Utc::now();
                let diff = now - self.t;
                if diff > Duration::milliseconds(200) {
                    self.t = now;
                    terminal.draw(|frame| self.draw(frame))?;
                }
            }
            if self.game.turn() == Color::Black && self.connection.waiting() {
                self.connection.check_move();
                self.engine_move();
            }
            if self.exit {
                break;
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        let game = &self.game;
        let clock = &self.clock;
        let hist = &self.hist;
        let engine_move = &self.engine_move;
        let avail_input = self
            .input_move
            .clone()
            .and_then(|input| alpha_to_i(&input).ok());
        render_main(game, hist, clock, engine_move, avail_input, frame);
    }

    fn handle_move_input(&mut self, c: char) {
        if self.game.turn() == shakmaty::Color::White {
            let base = match self.input_move.clone() {
                None => format!("{c}"),
                Some(i) => format!("{i}{c}"),
            };

            self.input_move = Some(base);
        }
    }

    fn run_engine(&mut self) {
        self.engine_move = None;
        self.connection.go(
            &self.game,
            self.clock.remaining_for(Color::White),
            self.clock.remaining_for(Color::Black),
        );
    }

    fn engine_move(&mut self) {
        if let Some(m) = self.connection.bestmove(&self.game) {
            self.game = self
                .game
                .clone()
                .play(&m)
                .expect("we got the move from engine");
            self.hist.push(m.clone());
            self.engine_move = Some(m);
            self.input_move = None;
            self.clock.hit();
            print!("{}", 0x07 as char);
        }
    }

    fn validate_move_input(&mut self) {
        if let Some(input) = self.input_move.clone() {
            if let Ok(index) = alpha_to_i(&input) {
                if let Some(m) = self.game.legal_moves().get(index) {
                    self.game = self
                        .game
                        .clone()
                        .play(m)
                        .expect("we got the move from legal moves");
                    self.hist.push(m.clone());
                    self.input_move = None;
                    self.clock.hit();
                    self.run_engine();
                }
            }
        }
    }

    fn clear_input(&mut self) {
        self.input_move = None;
    }

    fn start_game(&mut self) {
        self.clock.start();
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(' ') => self.start_game(),
            KeyCode::Char(c) => self.handle_move_input(c),
            KeyCode::Backspace => self.clear_input(),
            KeyCode::Enter => self.validate_move_input(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.connection.stop();
        self.exit = true;
    }
}

// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         match seek(&self.filepath, &self.name) {
//             Ok(week) => {
//                 self.render_week(&week, area, buf);
//             }
//             Err(err) => {
//                 self.render_err(&err, area, buf);
//             }
//         }
//     }
// }

// const NOON: u32 = 13;

// fn style_task(task: String, now: &DateTime<Local>, morning: bool) -> Line {
//     if (now.time().hour() < NOON && morning) || (now.time().hour() >= NOON && !morning) {
//         Line::from(vec![task.into()]).style(Style::new().add_modifier(Modifier::BOLD))
//     } else {
//         Line::from(vec![task.into()])
//     }
// }

// impl App {
//     fn render_week(&self, week: &Week, area: Rect, buf: &mut Buffer) {
//         let now = chrono::Local::now();
//         let title = Title::from(
//             format!(
//                 " La semaine de {} - {} ",
//                 &week.name,
//                 now.format("%d/%m/%Y %H:%M")
//             )
//             .bold(),
//         );

//         let block = Block::bordered()
//             .title(title.alignment(Alignment::Center))
//             .border_set(border::THICK)
//             .padding(Padding::uniform(6));

//         let mut footer = vec![Line::from(vec![format!("{}", week.block_name).into()])
//             .style(Style::new())
//             .fg(Color::Gray)];

//         let mut lines = week
//             .days
//             .iter()
//             .map(|wd| {
//                 if wd.today {
//                     vec![
//                         Line::from(vec![format!(" {} ", wd.day).into()])
//                             .style(Style::new().add_modifier(Modifier::BOLD))
//                             .bg(Color::LightCyan),
//                         style_task(format!("Matin:              {}", wd.matin), &now, true),
//                         style_task(format!("Aprés-midi:         {}", wd.apreme), &now, false),
//                         Line::from(vec!["".into()]),
//                     ]
//                 } else {
//                     vec![
//                         Line::from(vec![format!(" {} ", wd.day).into()])
//                             .style(Style::new().add_modifier(Modifier::BOLD))
//                             .bg(Color::Gray)
//                             .fg(Color::Black),
//                         Line::from(vec![format!("Matin:              {}", wd.matin).into()]),
//                         Line::from(vec![format!("Aprés-midi:         {}", wd.apreme).into()]),
//                         Line::from(vec!["".into()]),
//                     ]
//                 }
//             })
//             .flatten()
//             .collect::<Vec<_>>();

//         lines.append(&mut footer);
//         Paragraph::new(Text::from(lines))
//             .block(block)
//             .render(area, buf);
//     }

//     fn render_err(&self, err: &anyhow::Error, area: Rect, buf: &mut Buffer) {
//         let now = chrono::Local::now();
//         let title = Title::from(format!(" Error - {} ", now.format("%d/%m/%Y %H:%M")).bold());

//         let block = Block::bordered()
//             .title(title.alignment(Alignment::Center))
//             .border_set(border::THICK)
//             .padding(Padding::uniform(6));
//         Paragraph::new(Text::from(vec![Line::from(format!("-> {}", &err))]))
//             .block(block)
//             .render(area, buf);
//     }
// }
