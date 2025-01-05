use std::{
    // io::{stderr, Write},
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use chrono::Duration;
use shakmaty::{fen::Fen, Chess, Position};
use shakmaty_uci::{UciMessage, UciMove};

const STOCKFISH: &str = "/home/pierre/System/src/Stockfish/src/stockfish";

pub enum MessageTo {
    Go {
        fen: Fen,
        white_time: i64,
        black_time: i64,
    },
    Stop,
}

pub enum MessageFrom {
    Move(UciMove),
}

struct Engine {
    rx: Receiver<MessageTo>,
    tx: Sender<MessageFrom>,
    engine: uci::Engine,
}

impl Engine {
    fn new(rx: Receiver<MessageTo>, tx: Sender<MessageFrom>) -> Self {
        let engine = uci::Engine::new(STOCKFISH).expect("engine should be OK");
        Engine { rx, tx, engine }
    }

    fn set_options(&self) {
        let options = [
            ("Debug Log File", "/home/pierre/tmp/stockfish.log"),
            ("Threads", "2"),
            ("UCI_Elo", "2000"),
        ];
        for (name, value) in options {
            let _ = self.engine.set_option(name, value);
        }
    }

    fn start(&self) {
        self.set_options();
        loop {
            match self.rx.recv() {
                Err(err) => {
                    print!("Engine channel error: {}", err);
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

    fn go(&self, fen: Fen, white_time: i64, black_time: i64) {
        let setpos = shakmaty_uci::UciMessage::Position {
            startpos: false,
            fen: Some(fen),
            moves: Vec::new(),
        };
        let goc = shakmaty_uci::UciMessage::Go {
            time_control: Some(shakmaty_uci::UciTimeControl::TimeLeft {
                white_time: Some(
                    Duration::seconds(white_time)
                        .to_std()
                        .expect("good duration"),
                ),
                black_time: Some(
                    Duration::seconds(black_time)
                        .to_std()
                        .expect("good duration"),
                ),
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
                            UciMessage::from_str(&line)
                        {
                            self.tx
                                .send(MessageFrom::Move(best_move))
                                .expect("tx.send to never fail");
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
    // waiting: bool,
    best_move_uci: Option<UciMove>,
}

impl EngineConnection {
    fn new(rx: Receiver<MessageFrom>, tx: Sender<MessageTo>) -> Self {
        EngineConnection {
            rx,
            tx,
            // waiting: false,
            best_move_uci: None,
        }
    }

    pub fn waiting(&self) -> bool {
        self.best_move_uci.is_none()
    }

    pub fn check_move(&mut self) {
        if let Ok(MessageFrom::Move(m)) = self.rx.try_recv() {
            self.best_move_uci = Some(m);
        }
    }

    pub fn bestmove(&self, pos: &Chess) -> Option<shakmaty::Move> {
        self.best_move_uci
            .clone()
            .and_then(|um| um.to_move(pos).ok())
    }

    pub fn go(&mut self, pos: &Chess, white_time: i64, black_time: i64) {
        let setup = pos.clone().into_setup(shakmaty::EnPassantMode::Always);
        let fen = Fen::from_setup(setup);
        self.best_move_uci = None;
        println!("Clear Best Move");
        self.tx
            .send(MessageTo::Go {
                fen,
                white_time,
                black_time,
            })
            .expect("send move to work");
    }

    pub fn stop(&self) {
        self.tx.send(MessageTo::Stop).expect("send to work OK");
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
