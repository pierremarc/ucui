use std::{
    str::FromStr,
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
};

use chrono::Duration;
use shakmaty::{fen::Fen, Chess, FromSetup};
use shakmaty_uci::{UciMessage, UciMove};

use super::{Engine, EngineCommand, EngineMessage};

struct UciEngine {
    rx: Receiver<EngineCommand>,
    tx: Sender<EngineMessage>,

    engine: uci::Engine,
    options: Vec<(String, Option<String>)>,
}

impl UciEngine {
    fn new(
        path: &str,
        rx: Receiver<EngineCommand>,
        tx: Sender<EngineMessage>,
        args: Option<Vec<String>>,
        options: Vec<(String, Option<String>)>,
    ) -> Self {
        let engine = match args {
            None => uci::Engine::new(path).expect("engine should be OK"),
            Some(args) => uci::Engine::with_args(path, args).expect("engine should be OK"),
        };
        UciEngine {
            rx,
            tx,
            engine,
            options,
        }
    }

    fn set_options(&self) {
        for (id, value) in self.options.iter() {
            let _ = self
                .engine
                .set_option(id, &value.clone().unwrap_or(String::new()));
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
                    EngineCommand::NewGame => self.new_game(),
                    EngineCommand::Go {
                        fen,
                        white_time,
                        black_time,
                    } => self.go(fen, white_time, black_time),
                    EngineCommand::Stop => break,
                },
            }
        }
    }

    pub fn update_move(&self, best_move_uci: UciMove, game: Chess) {
        match best_move_uci.to_move(&game) {
            Err(e) => log::error!(
                "<uci-engine> Failed to produce a bestmove from {best_move_uci}: {} ",
                e,
            ),
            Ok(m) => {
                let _ = self.tx.send(EngineMessage::BestMove(m.into()));
            }
        }
    }

    fn new_game(&self) {
        let _ = self
            .engine
            .command_with_duration("ucinewgame", std::time::Duration::from_millis(100));
    }

    fn go(&self, fen_string: String, white_time: Duration, black_time: Duration) {
        if let Ok(fen) = Fen::from_str(&fen_string) {
            let setpos = shakmaty_uci::UciMessage::Position {
                startpos: false,
                fen: Some(fen.clone()),
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
                                let game = Chess::from_setup(
                                    fen.as_setup().clone(),
                                    shakmaty::CastlingMode::Standard,
                                )
                                .expect("argh!");
                                self.update_move(best_move, game);
                            } else {
                                log::debug!("<engine> {line}");
                            }
                        }
                        "OK".to_string()
                    });
            };
        } else {
            log::error!("<uci-engine> failed to produce a `Fen` from fen string:  '{fen_string}'");
        }
    }
}

pub struct EngineConnection {
    tx: Sender<EngineCommand>,
    receiver: Receiver<EngineMessage>,
}

impl EngineConnection {
    fn new(tx: Sender<EngineCommand>, rx: Receiver<EngineMessage>) -> Self {
        Self { tx, receiver: rx }
    }
}

impl Engine for EngineConnection {
    fn new_game(&self) {
        let _ = self.tx.send(EngineCommand::NewGame);
    }

    fn stop(&self) {
        let _ = self.tx.send(EngineCommand::Stop);
    }

    fn go(&self, fen_string: String, white_time: Duration, black_time: Duration) {
        let _ = self.tx.send(EngineCommand::Go {
            fen: fen_string,
            white_time,
            black_time,
        });
    }

    fn recv(&self) -> Result<EngineMessage, RecvError> {
        self.receiver.recv()
    }
}

pub fn connect_engine(
    path: &str,
    args: Option<Vec<String>>,
    options: Vec<(String, Option<String>)>,
) -> EngineConnection {
    let (sender_to, receiver_to) = channel::<EngineCommand>();
    let (sender_from, receiver_from) = channel::<EngineMessage>();
    let cloned_path = String::from(path);
    thread::spawn(move || {
        let engine = UciEngine::new(&cloned_path, receiver_to, sender_from, args, options);
        engine.start();
    });

    EngineConnection::new(sender_to, receiver_from)
}
