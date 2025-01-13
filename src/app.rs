use std::io;

use crate::clock::{Clock, ClockState, SharedClock};
use crate::config::{get_engine_color, get_start_pos};
use crate::engine::{connect_engine, EngineConnection};
use crate::logger::Logger;
use crate::state::{self, Gateway, State};
use crate::ui::{render, Screen};
use crate::util::alpha_to_i;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use shakmaty::fen::Fen;
use shakmaty::{Chess, Color, Position};

pub fn start_app() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

pub struct App {
    logger: Logger,
    exit: bool,
    store: state::Gateway,
    state: state::State,
    clock: SharedClock,
    pub connection: EngineConnection,
}
impl App {
    fn new() -> Self {
        App {
            exit: false,
            logger: Logger::init(256),
            store: state::Gateway::new(),
            state: state::State::default(),
            connection: connect_engine(),
            clock: Clock::new(),
        }
    }

    pub fn game(&self) -> Chess {
        self.state.game()
    }

    pub fn store(&self) -> &Gateway {
        &self.store
    }
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.store.update_game(get_start_pos().unwrap_or_default());
        terminal.draw(|frame| self.draw(frame))?;
        let mut count = 0;
        loop {
            if self.exit {
                break;
            }

            self.connection.check_move();

            if self.game().turn() == get_engine_color() && self.connection.waiting() {
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
                if let Ok(new_state) = self.store.current_state() {
                    if self.state != new_state {
                        terminal.draw(|frame| self.draw(frame))?;
                    } else {
                        println!("states are the same");
                    }
                }

                self.logger.check_logs(&self.store);
            }

            count += 1;
            {
                if count > 300 {
                    count = 0;
                    self.store.update_screen(Screen::Log);
                    terminal.draw(|frame| self.draw(frame))?;
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        render(&self.state, frame);
    }

    fn run_engine(&mut self) {
        self.state.engine_move = None;
        let clock = self.clock.lock().expect("oops clock");
        self.connection.go(
            &self.game(),
            clock.remaining(Color::White),
            clock.remaining(Color::Black),
        );
        self.store.update_engine_waiting(true);
    }

    fn engine_move_try(&mut self) {
        match (self.connection.bestmove(&self.game()), self.clock.lock()) {
            (Some(m), Ok(mut clock)) => {
                log::info!("engine play {m}");
                self.connection.stop_waiting();
                let game = self.game().play(&m).expect("we got the move from engine");
                let mut hist = self.state.hist.clone();
                hist.push(m.clone());
                self.store
                    .update_fen(Fen::from_position(game, shakmaty::EnPassantMode::Always));
                self.store.update_hist(hist);
                self.store.update_engine_move(Some(m));
                self.store.update_avail_input(None);
                self.store.update_engine_waiting(false);
                clock.hit();
                println!("{}", 0x07 as char);
            }
            (Some(_), _) => panic!("could not get a clock for engine"),
            _ => {}
        }
    }

    pub fn validate_move_input(&mut self) {
        if let Some(input) = self.state.avail_input.clone() {
            if let Ok(index) = alpha_to_i(&input) {
                let game = self.game();
                if let Some(m) = game.legal_moves().get(index) {
                    if let Ok(game) = game.play(m) {
                        let mut hist = self.state.hist.clone();
                        hist.push(m.clone());
                        self.store.update_hist(hist);
                        self.store.update_game(game);
                        self.clear_input();
                        let _ = self.clock.lock().map(|mut c| c.hit());
                        self.run_engine();
                    }
                }
            }
        }
    }

    pub fn clear_input(&mut self) {
        self.store.update_avail_input(None);
    }

    pub fn start_game(&mut self) {
        let clock = self.clock.clone();
        let turn = self.game().turn();
        let store = self.store.clone();
        if let Ok(clock) = clock.lock() {
            if let ClockState::Initial = clock.state() {
                self.clock = clock
                    .clone()
                    .start(turn, store)
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
        self.store.update_screen(Screen::Play);
    }

    pub fn exit(&mut self) {
        self.connection.stop();
        self.exit = true;
    }
}
