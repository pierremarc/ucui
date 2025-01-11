use std::io;

use crate::clock::{Clock, ClockState};
use crate::config::{get_engine_color, get_start_pos};
use crate::engine::{connect_engine, EngineConnection};
use crate::ui::{render, AppState, Screen};
use crate::util::alpha_to_i;
use chrono::Duration;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use shakmaty::{Chess, Color, Move, Position};

pub fn start_app() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    t: chrono::DateTime<chrono::Utc>,
    exit: bool,
    pub game: Chess,
    pub hist: Vec<Move>,
    pub clock: Clock,
    pub input_move: Option<String>,
    pub connection: EngineConnection,
    pub engine_move: Option<Move>,
    pub screen: Screen,
}
impl App {
    fn new() -> Self {
        App {
            exit: false,
            t: chrono::Utc::now(),
            game: match get_start_pos() {
                None => Chess::default(),
                Some(pos) => pos,
            },
            hist: Vec::new(),
            clock: Clock::new(),
            input_move: None,
            connection: connect_engine(),
            engine_move: None,
            screen: Screen::Home,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.clear()?;
        terminal.draw(|frame| self.draw(frame))?;
        loop {
            if event::poll(std::time::Duration::from_millis(16))? {
                // It's guaranteed that `read` won't block, because `poll` returned
                // `Ok(true)`.
                match event::read()? {
                    // it's important to check that the event is a key press event as
                    // crossterm also emits key release and repeat events on Windows.
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        crate::ui::handle_key_event(self, key_event)
                    }
                    _ => {}
                };
            } else {
                let now = chrono::Utc::now();
                let diff = now - self.t;
                if diff > Duration::milliseconds(60) {
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
        let engine_waiting = self.connection.waiting();
        let avail_input = self
            .input_move
            .clone()
            .and_then(|input| alpha_to_i(&input).ok());

        let state = AppState {
            screen: self.screen,
            game,
            hist,
            clock,
            engine_move,
            engine_waiting,
            avail_input,
        };
        render(&state, frame);
    }

    fn run_engine(&mut self) {
        self.engine_move = None;
        self.connection.go(
            &self.game,
            self.clock.remaining_for_uci(Color::White),
            self.clock.remaining_for_uci(Color::Black),
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

    pub fn validate_move_input(&mut self) {
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

    pub fn clear_input(&mut self) {
        self.input_move = None;
    }

    pub fn start_game(&mut self) {
        if let ClockState::Initial = self.clock.check_state() {
            self.clock.start(self.game.turn());
            log::info!("start_game {:?} {:?}", get_engine_color(), self.game.turn());
            if get_engine_color() == self.game.turn() {
                self.run_engine();
            }
        }
        self.screen = Screen::Play;
    }

    pub fn exit(&mut self) {
        self.connection.stop();
        self.exit = true;
    }
}
