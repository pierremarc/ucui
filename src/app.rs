use std::io;

use crate::clock::{Clock, ClockState, SharedClock};
use crate::config::{get_engine_color, get_start_pos};
use crate::engine::{connect_engine, EngineConnection};
use crate::logger::Logger;
use crate::ui::{render, AppState, LogState, Screen};
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

pub struct App {
    t: chrono::DateTime<chrono::Utc>,
    logger: Logger,
    exit: bool,
    pub game: Chess,
    pub hist: Vec<Move>,
    pub clock: SharedClock,
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
            logger: Logger::init(256),
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
        // terminal.clear()?;
        // terminal.draw(|frame| self.draw(frame))?;
        loop {
            if self.exit {
                break;
            }

            self.connection.check_move();
            if self.game.turn() == get_engine_color() && self.connection.waiting() {
                self.engine_move_try();
            }

            if event::poll(std::time::Duration::from_millis(120))? {
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

                self.logger.check_logs();
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        let game = &self.game;
        let clock = &self.clock.lock().expect("oops clock");
        let hist = &self.hist;
        let engine_move = &self.engine_move;
        let engine_waiting = self.connection.waiting();
        let avail_input = self
            .input_move
            .clone()
            .and_then(|input| alpha_to_i(&input).ok());
        let log = &LogState {
            lines: self.logger.logs(),
        };

        let state = AppState {
            screen: self.screen,
            game,
            hist,
            clock,
            engine_move,
            engine_waiting,
            avail_input,
            log,
        };
        render(&state, frame);
    }

    fn run_engine(&mut self) {
        self.engine_move = None;
        let clock = self.clock.lock().expect("oops clock");
        self.connection.go(
            &self.game,
            clock.remaining_seconds(Color::White),
            clock.remaining_seconds(Color::Black),
        );
    }

    fn engine_move_try(&mut self) {
        match (self.connection.bestmove(&self.game), self.clock.lock()) {
            (Some(m), Ok(mut clock)) => {
                log::info!("engine play {m}");
                self.connection.stop_waiting();
                self.game = self
                    .game
                    .clone()
                    .play(&m)
                    .expect("we got the move from engine");
                self.hist.push(m.clone());
                self.engine_move = Some(m);
                self.input_move = None;
                clock.hit();
                println!("{}", 0x07 as char);
            }
            (None, _) => log::info!("missing bestmove"),
            (Some(_), _) => panic!("could not get a clock for engine"),
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
                    let _ = self.clock.lock().map(|mut c| c.hit());
                    self.run_engine();
                }
            }
        }
    }

    pub fn clear_input(&mut self) {
        self.input_move = None;
    }

    pub fn start_game(&mut self) {
        let clock = self.clock.clone();
        let turn = self.game.turn();
        if let Ok(clock) = clock.lock() {
            if let ClockState::Initial = clock.state() {
                self.clock = clock
                    .clone()
                    .start(turn)
                    .expect("could not build a started clock");
                if get_engine_color() == turn {
                    self.run_engine();
                }
                log::info!(
                    "start_game turn {:?}; engine:{:?}",
                    turn,
                    get_engine_color()
                );
            }
        }
        self.screen = Screen::Play;
    }

    pub fn exit(&mut self) {
        self.connection.stop();
        self.exit = true;
    }
}
