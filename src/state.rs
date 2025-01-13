use std::{
    error::Error,
    fmt::Display,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crossterm::event::Event;
use shakmaty::{fen::Fen, Chess, FromSetup, Move};

use crate::{clock::ClockState, ui::Screen, util::alpha_to_i};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct LogState {
    pub lines: Vec<String>,
}

impl LogState {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

#[derive(Debug, PartialOrd, Clone, Default)]
enum EventContainer {
    #[default]
    None,
    Event(Event),
}

impl EventContainer {
    fn new(event: Event) -> Self {
        Self::Event(event)
    }
    fn event(&self) -> Option<Event> {
        match self {
            EventContainer::None => None,
            EventContainer::Event(event) => Some(event.clone()),
        }
    }
}

impl PartialEq for EventContainer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventContainer::None, EventContainer::None) => true,
            (EventContainer::Event(a), EventContainer::Event(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for EventContainer {}

#[derive(Clone, Eq, PartialEq, Debug)]
#[derive(Default)]
pub struct State {
    pub screen: Screen,
    pub fen: Fen,
    pub hist: Vec<Move>,
    pub clock: ClockState,
    pub engine_move: Option<Move>,
    pub engine_waiting: bool,
    pub avail_input: Option<String>,
    pub log: LogState,
    pub event_container: EventContainer,
}


pub enum StateValue {
    Screen(Screen),
    Fen(Fen),
    Hist(Vec<Move>),
    Clock(ClockState),
    EngineMove(Option<Move>),
    EngineWaiting(bool),
    AvailInput(Option<String>),
    Log(LogState),
    Event(EventContainer),
}

impl State {
    pub fn game(&self) -> Chess {
        Chess::from_setup(
            self.fen.as_setup().clone(),
            shakmaty::CastlingMode::Standard,
        )
        .expect("State is not supposed to hold an invalid FEN position.")
    }
    pub fn input_move(&self) -> Option<usize> {
        self.avail_input
            .clone()
            .and_then(|input| alpha_to_i(&input).ok())
    }
}

// enum StoreMessage {
//     RequestState,
//     ResponseState(State),
// }

struct Store {
    receiver: Receiver<StateValue>,
    state: State,
}

impl Store {
    fn new(receiver: Receiver<StateValue>) -> Self {
        Self {
            receiver,
            state: State::default(),
        }
    }

    pub fn start(&mut self) {
        loop {
            match self.receiver.recv() {
                Err(_) => break,
                Ok(msg) => self.update_state(msg),
            }
        }
    }

    fn update_state(&mut self, update: StateValue) {
        let state = &mut self.state;
        match update {
            StateValue::Screen(value) => state.screen = value,
            StateValue::Fen(value) => state.fen = value,
            StateValue::Hist(value) => state.hist = value,
            StateValue::Clock(value) => state.clock = value,
            StateValue::EngineMove(value) => state.engine_move = value,
            StateValue::EngineWaiting(value) => state.engine_waiting = value,
            StateValue::AvailInput(value) => state.avail_input = value,
            StateValue::Log(value) => state.log = value,
            StateValue::Event(value) => state.event_container = value,
        }
    }
}

#[derive(Clone)]
pub struct Gateway {
    store: Arc<Mutex<Store>>,
    transmiter: Sender<StateValue>,
}

#[derive(Debug)]
pub enum GatewayError {
    LockState(String),
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Error for GatewayError {}

impl Gateway {
    pub fn new() -> Self {
        let (transmiter, receiver) = channel::<StateValue>();
        let store = Arc::new(Mutex::new(Store::new(receiver)));
        let cloned = store.clone();
        thread::spawn(move || match cloned.lock() {
            Err(_) => panic!("Failed to lock store"),
            Ok(mut store) => store.start(),
        });
        Self { transmiter, store }
    }

    pub fn current_state(&self) -> Result<State, GatewayError> {
        self.store
            .lock()
            .map(|store| store.state.clone())
            .map_err(|e| GatewayError::LockState(format!("{e}")))
    }

    fn update(&self, value: StateValue) {
        let _ = self.transmiter.send(value);
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
    pub fn update_hist(&self, value: Vec<Move>) {
        self.update(StateValue::Hist(value));
    }
    pub fn update_clock(&self, value: ClockState) {
        self.update(StateValue::Clock(value));
    }
    pub fn update_engine_move(&self, value: Option<Move>) {
        self.update(StateValue::EngineMove(value));
    }
    pub fn update_engine_waiting(&self, value: bool) {
        self.update(StateValue::EngineWaiting(value));
    }
    pub fn update_avail_input(&self, value: Option<String>) {
        self.update(StateValue::AvailInput(value));
    }
    pub fn update_log(&self, value: LogState) {
        self.update(StateValue::Log(value));
    }
    pub fn update_event(&self, value: Event) {
        self.update(StateValue::Event(EventContainer::new(value)));
    }
}
