use std::io;
use std::sync::mpsc::{channel, Receiver};

use crate::clock::{Clock, ClockState, SharedClock};
use crate::config::{get_engine_color, get_start_pos, get_time_black, get_time_white};
use crate::engine::{connect_engine, EngineConnection, EngineState};
use crate::logger::Logger;
use crate::state::{self, State, StateValue};
use crate::ui::{event_loop, render, Screen};
use crate::util::alpha_to_i;
use chrono::Duration;
use ratatui::{DefaultTerminal, Frame};
use shakmaty::fen::Fen;
use shakmaty::{Chess, Color, Position};

pub fn start_app() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct ActionState {
    engine_moved: bool,
    game_started: bool,
    input_validated: bool,
}

pub struct App {
    logger: Logger,
    store_change: Receiver<state::State>,
    store: state::Store,
    state: state::State,
    clock: SharedClock,
    engine: EngineConnection,
    action_state: ActionState,
}
impl App {
    fn new() -> Self {
        let (store_signal, store_change) = channel::<State>();
        let store = state::Store::new(store_signal);
        App {
            logger: Logger::try_new(256).expect("Failed to set logger"),
            store_change,
            store: store.clone(),
            state: state::State::default(),
            engine: connect_engine(store.clone()),
            clock: Clock::new_shared(),
            action_state: ActionState::default(),
        }
    }

    pub fn game(&self) -> Chess {
        self.state.game()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // println!("init logger");
        self.logger.init(self.store.clone());
        // println!("init event loop");
        event_loop(self.store.clone());
        // println!("update start position");
        self.store.update_game(get_start_pos().unwrap_or_default());
        // println!("initiating first draw");
        terminal.draw(|frame| self.draw(frame))?;

        loop {
            if self.state.exit {
                break;
            }

            match self.store_change.recv() {
                Ok(new_state) => {
                    if self.state != new_state {
                        // if let Some(diff) = self.state.diff(&new_state) {
                        //     if diff.get(0).is_some()
                        //         && diff.len() == 1
                        //         && (diff[0] == String::from("Clock") || diff[0] == String::from("Log"))
                        //     {
                        //         //
                        //     } else {
                        //         log::info!("[state diff] {}", diff.join(", "));
                        //     }
                        self.state = new_state;
                        self.actions();

                        terminal.draw(|frame| self.draw(frame))?;
                    }
                }
                Err(_) => break,
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.buffer_mut().reset();
        render(&self.state, frame);
    }

    fn actions(&mut self) {
        self.engine_move();
        self.input_move();
        self.start_game();
    }

    fn engine_move(&mut self) {
        if let EngineState::PendingMove(m) = self.state.engine.clone() {
            if !self.action_state.engine_moved {
                self.action_state.engine_moved = true;
                self.action_state.input_validated = false;
                self.store.update_engine(EngineState::Move(m.clone()));
                self.clock
                    .lock()
                    .map(|mut clock| {
                        if let Ok(game) = self.game().play(&m) {
                            let mut hist = self.state.hist.clone();
                            hist.push(m.clone());
                            self.store.update_batch([
                                StateValue::ValidateInput(false),
                                StateValue::AvailInput(None),
                                StateValue::Hist(hist),
                                StateValue::Fen(Fen::from_position(
                                    game,
                                    shakmaty::EnPassantMode::Always,
                                )),
                            ]);

                            clock.hit();
                            log::info!("engine played {m}");
                        } else {
                            let msg = format!(
                                "!! engine move failed {} <> {}",
                                &self.state.fen,
                                m,
                            );
                            log::warn!("{}", &msg);
                        }
                    })
                    .expect("Failed to lock clock");
            }
        }
    }

    fn input_move(&mut self) {
        if let (false, true, Some(input)) = (
            self.action_state.input_validated,
            self.state.validate_input,
            self.state.avail_input.clone(),
        ) {
            self.action_state.input_validated = true;
            if let Ok(index) = alpha_to_i(&input) {
                let game = self.game();
                if let Some(m) = game.legal_moves().get(index) {
                    match (game.play(m), self.clock.lock()) {
                        (Ok(game), Ok(mut clock)) => {
                            let mut hist = self.state.hist.clone();
                            hist.push(m.clone());
                            self.store.update_batch([
                                StateValue::ValidateInput(false),
                                StateValue::AvailInput(None),
                                StateValue::Hist(hist),
                                StateValue::Fen(Fen::from_position(
                                    game.clone(),
                                    shakmaty::EnPassantMode::Always,
                                )),
                                StateValue::Engine(EngineState::Computing),
                            ]);
                            clock.hit();
                            log::info!(
                                "[input board] {}",
                                Fen::from_position(game.clone(), shakmaty::EnPassantMode::Always)
                            );
                            self.engine.go(
                                Fen::from_position(game, shakmaty::EnPassantMode::Always),
                                clock.remaining(Color::White),
                                clock.remaining(Color::Black),
                            );
                            self.action_state.engine_moved = false;
                        }
                        _ => panic!("missing game or clock, very bad"),
                    }
                }
            }
        }
    }

    fn start_game(&mut self) {
        if self.state.game_started && !self.action_state.game_started {
            self.action_state.game_started = true;

            let turn = self.game().turn();

            if let ClockState::Initial = self.state.clock {
                self.engine.new_game();
                crate::clock::start_shared(self.clock.clone(), self.store.clone(), turn);
                if get_engine_color() == turn {
                    self.engine.go(
                        self.state.fen.clone(),
                        Duration::seconds(get_time_white()),
                        Duration::seconds(get_time_black()),
                    );
                }
                log::info!(
                    "start_game turn {:?}; engine:{:?}",
                    turn,
                    get_engine_color()
                );
            }

            self.store.update_screen(Screen::Play);
        }
    }
}
