use std::{
    // io::{stderr, Write},
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use chrono::Duration;
use shakmaty::{fen::Fen, Chess, Position};
use shakmaty_uci::{UciMessage, UciMove};

use crate::config::{get_engine, get_engine_args, get_engine_options};

pub enum MessageTo {
    Go {
        fen: Fen,
        white_time: Duration,
        black_time: Duration,
    },
    Stop,
}

pub enum MessageFrom {
    Move(UciMove),
    Output(String),
}

struct Engine {
    rx: Receiver<MessageTo>,
    tx: Sender<MessageFrom>,
    engine: uci::Engine,
}

impl Engine {
    fn new(rx: Receiver<MessageTo>, tx: Sender<MessageFrom>) -> Self {
        let engine = match get_engine_args() {
            None => uci::Engine::new(get_engine()).expect("engine should be OK"),
            Some(args) => uci::Engine::with_args(get_engine(), args).expect("engine should be OK"),
        };
        Engine { rx, tx, engine }
    }

    fn set_options(&self) {
        for opt in get_engine_options() {
            let _ = self.engine.set_option(opt.id(), opt.value());
        }
    }

    fn start(&self) {
        self.set_options();
        let _ = self
            .engine
            .command_with_duration("ucinewgame", std::time::Duration::from_millis(100));

        loop {
            match self.rx.recv() {
                Err(err) => {
                    log::error!("Engine channel error: {}", err);
                    break;
                }
                Ok(msg) => match msg {
                    MessageTo::Stop => {
                        break;
                    }
                    MessageTo::Go {
                        fen,
                        white_time,
                        black_time,
                    } => self.go(fen, white_time, black_time),
                },
            }
        }
    }

    fn go(&self, fen: Fen, white_time: Duration, black_time: Duration) {
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
            let _ = self
                .engine
                .command_and_wait_for(&goc.to_string(), "bestmove")
                .map(|lines| {
                    for line in lines.split("\n") {
                        if let Ok(UciMessage::BestMove { best_move, .. }) =
                            UciMessage::from_str(line)
                        {
                            // log::info!("[go] send a bestmove! {}", best_move.to_string());
                            self.tx
                                .send(MessageFrom::Move(best_move))
                                .expect("tx.send to never fail");
                        } else {
                            let _ = self.tx.send(MessageFrom::Output(line.into()));
                        }
                    }
                    "OK".to_string()
                });
        };
    }
}

#[derive(Debug)]
pub struct EngineConnection {
    rx: Receiver<MessageFrom>,
    tx: Sender<MessageTo>,
    waiting: bool,
    best_move_uci: Option<UciMove>,
}

impl EngineConnection {
    fn new(rx: Receiver<MessageFrom>, tx: Sender<MessageTo>) -> Self {
        EngineConnection {
            rx,
            tx,
            waiting: false,
            best_move_uci: None,
        }
    }

    pub fn waiting(&self) -> bool {
        self.waiting
    }

    pub fn stop_waiting(&mut self) {
        self.waiting = false;
    }

    pub fn check_move(&mut self) {
        for msg in self.rx.try_iter() {
            match msg {
                MessageFrom::Move(m) => {
                    // log::info!("[check_move] receive a bestmove! {}", m.to_string());
                    self.best_move_uci = Some(m);
                }
                MessageFrom::Output(s) => {
                    if !s.is_empty() {
                        // log::info!("<engine>> {s}");
                    }
                }
            }
        }
    }

    pub fn bestmove(&self, pos: &Chess) -> Option<shakmaty::Move> {
        self.best_move_uci
            .clone()
            .and_then(|um| match um.to_move(pos) {
                Err(e) => {
                    log::error!("Failed to produce a bestmove from {um}: {e}");
                    None
                }
                Ok(bestmove) => Some(bestmove),
            })
    }

    pub fn go(&mut self, pos: &Chess, white_time: Duration, black_time: Duration) {
        let setup = pos.clone().into_setup(shakmaty::EnPassantMode::Always);
        let fen = Fen::from_setup(setup);
        self.best_move_uci = None;
        self.waiting = true;
        self.tx
            .send(MessageTo::Go {
                fen,
                white_time,
                black_time,
            })
            .expect("to send go");
    }

    pub fn stop(&self) {
        self.tx.send(MessageTo::Stop).expect("to send stop");
    }
}

pub fn connect_engine() -> EngineConnection {
    let (sender_to, receiver_to) = channel::<MessageTo>();
    let (sender_from, receiver_from) = channel::<MessageFrom>();

    thread::spawn(move || {
        let engine = Engine::new(receiver_to, sender_from);
        engine.start();
    });
    EngineConnection::new(receiver_from, sender_to)
}
