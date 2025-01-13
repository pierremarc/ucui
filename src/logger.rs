use log::{Level, Metadata, Record};

use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::{
    config::get_log_level,
    state::{LogState, Store},
    util::RotatingList,
};

pub struct Logger {
    cap: usize,
    rx: Option<Receiver<String>>,
}

struct LoggerProxy {
    tx: Sender<String>,
}

impl log::Log for LoggerProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let elem = format!("{} - {}", record.level(), record.args());
            let _ = self.tx.send(elem);
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn try_new(cap: usize) -> Result<Logger, String> {
        let (tx, rx) = channel();
        log::set_boxed_logger(Box::new(LoggerProxy { tx }))
            .map(|_| log::set_max_level(get_log_level()))
            .map_err(|e| format!("{}", e))?;
        Ok(Logger { cap, rx: Some(rx) })
    }

    pub fn init(&mut self, store: Store) {
        let rx = self.rx.take().expect("Logger::init should be called once");
        let cap = self.cap;
        thread::spawn(move || {
            let mut logs = RotatingList::new(cap);

            loop {
                if let Ok(line) = rx.recv() {
                    logs.push(line);
                    store.update_log(LogState::new(logs.iter().map(String::clone).collect()));
                }
            }
        });
    }
}
