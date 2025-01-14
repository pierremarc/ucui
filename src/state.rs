use std::{
    error::Error,
    fmt::Display,
    sync::{
        mpsc::{sync_channel, Sender, SyncSender},
        Arc, Mutex,
    },
    thread,
};

use shakmaty::{fen::Fen, Chess, FromSetup, Move};

use crate::{
    clock::ClockState,
    engine::EngineState,
    ui::Screen,
    util::{alpha_to_i, MoveIndex},
};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct LogState {
    pub lines: Vec<String>,
}

impl LogState {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct State {
    pub screen: Screen,
    pub fen: Fen,
    pub hist: Vec<Move>,
    pub clock: ClockState,
    pub engine: EngineState,
    pub avail_input: Option<String>,
    pub validate_input: bool,
    pub log: LogState,
    pub game_started: bool,
    pub exit: bool,
    pub input: MoveIndex,
}

#[derive(Debug)]
pub enum StateValue {
    Screen(Screen),
    Fen(Fen),
    Hist(Vec<Move>),
    Clock(ClockState),
    Engine(EngineState),
    AvailInput(Option<String>),
    ValidateInput(bool),
    Log(LogState),
    GameStarted(bool),
    Exit(bool),
    Input(MoveIndex),
}

impl State {
    pub fn game(&self) -> Chess {
        Chess::from_setup(
            self.fen.as_setup().clone(),
            shakmaty::CastlingMode::Standard,
        )
        .expect("State is not supposed to hold an invalid FEN position.")
    }
    #[allow(unused)]
    pub fn input_move(&self) -> Option<usize> {
        self.avail_input
            .clone()
            .and_then(|input| alpha_to_i(&input).ok())
    }

    fn update(&mut self, batch: Vec<StateValue>) {
        for update in batch {
            match update {
                StateValue::Clock(_) | StateValue::Log(_) => {}
                _ => {
                    log::info!("{:?}", &update);
                }
            }
            match update {
                StateValue::Screen(value) => self.screen = value,
                StateValue::Fen(value) => self.fen = value,
                StateValue::Hist(value) => self.hist = value,
                StateValue::Clock(value) => self.clock = value,
                StateValue::Engine(value) => self.engine = value,
                StateValue::AvailInput(value) => self.avail_input = value,
                StateValue::ValidateInput(value) => self.validate_input = value,
                StateValue::Log(value) => self.log = value,
                StateValue::GameStarted(value) => self.game_started = value,
                StateValue::Exit(value) => self.exit = value,
                StateValue::Input(value) => self.input = value,
            }
        }
    }

    #[allow(unused)]
    pub fn diff(&self, other: &State) -> Option<Vec<String>> {
        if *self == *other {
            None
        } else {
            let mut diff: Vec<String> = Vec::new();
            let keys = [
                "Screen",
                "Fen",
                "Hist",
                "Clock",
                "Engine",
                "AvailInput",
                "ValidateInput",
                "Log",
                "GameStarted",
                "Exit",
            ];
            for key in keys {
                match key {
                    "Screen" => {
                        if self.screen != other.screen {
                            diff.push("Screen".into());
                        }
                    }
                    "Fen" => {
                        if self.fen != other.fen {
                            diff.push("Fen".into());
                        }
                    }
                    "Hist" => {
                        if self.hist != other.hist {
                            diff.push("Hist".into());
                        }
                    }
                    "Clock" => {
                        if self.clock != other.clock {
                            diff.push("Clock".into());
                        }
                    }
                    "Engine" => {
                        if self.engine != other.engine {
                            diff.push("Engine".into());
                        }
                    }
                    "AvailInput" => {
                        if self.avail_input != other.avail_input {
                            diff.push("AvailInput".into());
                        }
                    }
                    "ValidateInput" => {
                        if self.validate_input != other.validate_input {
                            diff.push("ValidateInput".into());
                        }
                    }
                    "Log" => {
                        if self.log != other.log {
                            diff.push("Log".into());
                        }
                    }
                    "GameStarted" => {
                        if self.game_started != other.game_started {
                            diff.push("GameStarted".into());
                        }
                    }
                    "Exit" => {
                        if self.exit != other.exit {
                            diff.push("Exit".into());
                        }
                    }
                    _ => {}
                };
            }
            Some(diff)
        }
    }
}

#[derive(Clone)]
pub struct Store {
    state: Arc<Mutex<State>>,
    updater: SyncSender<Vec<StateValue>>,
    signal: Sender<State>,
}

#[derive(Debug)]
pub enum StoreError {
    LockState(String),
}

impl Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::LockState(m) => f.write_str(m),
        }
    }
}
impl Error for StoreError {}

impl Store {
    pub fn new(signal: Sender<State>) -> Self {
        let (updater, receiver) = sync_channel::<Vec<StateValue>>(0);
        let state = Arc::new(Mutex::new(State::default()));
        let cloned = state.clone();
        thread::spawn(move || loop {
            // ATM we allow for only one global writer, lets see
            // if let Ok(mut state) = cloned.write() {
            match receiver.recv() {
                Err(_) => break,
                // Ok(update) => state.update(update),
                Ok(update) => cloned
                    .lock()
                    .map(|mut state| state.update(update))
                    .expect("cannot lock state, very bad."),
            }
            // }
        });
        Self {
            updater,
            signal,
            state,
        }
    }

    pub fn current_state(&self) -> Result<State, StoreError> {
        self.state
            .lock()
            .map(|state| state.clone())
            .map_err(|e| StoreError::LockState(format!("{e}")))
    }

    #[allow(unused)]
    pub fn update_batch<V>(&self, values: V)
    where
        V: Into<Vec<StateValue>>,
    {
        let _ = self.updater.send(values.into());
        let _ = self.current_state().map(|state| self.signal.send(state));
    }

    fn update(&self, value: StateValue) {
        self.update_batch([value]);
        // let _ = self.updater.send(vec![value]);
        // let _ = self.current_state().map(|state| self.signal.send(state));
    }

    pub fn update_screen(&self, value: Screen) {
        self.update(StateValue::Screen(value));
    }
    pub fn update_fen(&self, value: Fen) {
        self.update(StateValue::Fen(value));
    }
    pub fn update_game(&self, value: Chess) {
        self.update_fen(Fen::from_position(value, shakmaty::EnPassantMode::Always));
    }
    #[allow(unused)]
    pub fn update_hist(&self, value: Vec<Move>) {
        self.update(StateValue::Hist(value));
    }
    pub fn update_clock(&self, value: ClockState) {
        self.update(StateValue::Clock(value));
    }
    pub fn update_engine(&self, value: EngineState) {
        self.update(StateValue::Engine(value));
    }
    pub fn update_avail_input(&self, value: Option<String>) {
        self.update(StateValue::AvailInput(value));
    }
    pub fn update_validate_input(&self, value: bool) {
        self.update(StateValue::ValidateInput(value));
    }
    pub fn update_log(&self, value: LogState) {
        self.update(StateValue::Log(value));
    }
    pub fn update_game_started(&self, value: bool) {
        self.update(StateValue::GameStarted(value));
    }
    pub fn update_exit(&self, value: bool) {
        self.update(StateValue::Exit(value));
    }
    pub fn update_input(&self, value: MoveIndex) {
        self.update(StateValue::Input(value));
    }
}
