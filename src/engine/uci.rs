pub trait UcuiEngine {
    fn new(tx: Sender<EngineMessage>) -> Self;

    fn new_game(&self);

    fn go(&self, fen: Fen, white_time: Duration, black_time: Duration);
}

use std::{
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use chrono::Duration;
use shakmaty::{fen::Fen, Chess};
use shakmaty_uci::{UciMessage, UciMove};

use crate::{
    config::{get_engine, get_engine_args, get_engine_options},
    state::Store,
};

use super::{Engine, EngineMessage, EngineState};

struct UciEngine {
    rx: Receiver<EngineMessage>,
    store: Store,
    engine: uci::Engine,
}

impl UciEngine {
    fn new(rx: Receiver<EngineMessage>, store: Store) -> Self {
        let engine = match get_engine_args() {
            None => uci::Engine::new(get_engine()).expect("engine should be OK"),
            Some(args) => uci::Engine::with_args(get_engine(), args).expect("engine should be OK"),
        };
        UciEngine { rx, engine, store }
    }

    fn set_options(&self) {
        for opt in get_engine_options() {
            let _ = self.engine.set_option(opt.id(), opt.value());
        }
    }

    fn start(&self) {
        self.set_options();

        loop {
            match self.rx.recv() {
                Err(err) => {
                    log::error!("Engine channel error: {}", err);
                    break;
                }
                Ok(msg) => match msg {
                    EngineMessage::NewGame => self.new_game(),
                    EngineMessage::Go {
                        fen,
                        white_time,
                        black_time,
                    } => self.go(fen, white_time, black_time),
                },
            }
        }
    }

    fn update_store(&self, state: EngineState) {
        self.store.update_engine(state);
    }

    pub fn update_move(&self, best_move_uci: UciMove, game: &Chess) {
        match best_move_uci.to_move(game) {
            Err(e) => log::error!(
                "Failed to produce a bestmove from {best_move_uci}: {} ({})",
                e,
                Fen::from_position(game.clone(), shakmaty::EnPassantMode::Always)
            ),
            Ok(m) => self.store.update_engine(EngineState::PendingMove(m)),
        }
    }

    fn new_game(&self) {
        let _ = self
            .engine
            .command_with_duration("ucinewgame", std::time::Duration::from_millis(100));
        self.update_store(EngineState::Idle);
    }

    fn computing(&self) {
        self.update_store(EngineState::Computing);
    }

    fn go(&self, fen: Fen, white_time: Duration, black_time: Duration) {
        self.computing();
        let setpos = shakmaty_uci::UciMessage::Position {
            startpos: false,
            fen: Some(fen),
            moves: Vec::new(),
        };
        let goc = shakmaty_uci::UciMessage::Go {
            time_control: Some(shakmaty_uci::UciTimeControl::TimeLeft {
                white_time: Some(white_time.to_std().expect("positive duration")),
                black_time: Some(black_time.to_std().expect("positive duration")),
                white_increment: None,
                black_increment: None,
                moves_to_go: None,
            }),
            search_control: None,
        };
        if self.engine.command(&setpos.to_string()).is_ok() {
            let game = self
                .store
                .current_state()
                .expect("failed to get state")
                .game();
            let _ = self
                .engine
                .command_and_wait_for(&goc.to_string(), "bestmove")
                .map(|lines| {
                    for line in lines.split("\n") {
                        if let Ok(UciMessage::BestMove { best_move, .. }) =
                            UciMessage::from_str(line)
                        {
                            self.update_move(best_move, &game);
                        } else {
                            log::debug!("<engine> {line}");
                        }
                    }
                    "OK".to_string()
                });
        };
    }
}

pub struct EngineConnection {
    tx: Sender<EngineMessage>,
}

impl EngineConnection {
    fn new(tx: Sender<EngineMessage>) -> Self {
        Self { tx }
    }
}

impl Engine for EngineConnection {
    fn new_game(&self) {
        let _ = self.tx.send(EngineMessage::NewGame);
    }

    fn go(&self, fen: Fen, white_time: Duration, black_time: Duration) {
        let _ = self.tx.send(EngineMessage::Go {
            fen,
            white_time,
            black_time,
        });
    }
}

pub fn connect_engine(store: Store) -> EngineConnection {
    let (sender_to, receiver_to) = channel::<EngineMessage>();
    thread::spawn(move || {
        let engine = UciEngine::new(receiver_to, store);
        engine.start();
    });
    EngineConnection::new(sender_to)
}
