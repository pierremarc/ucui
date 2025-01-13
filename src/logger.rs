use log::{Level, Metadata, Record};

use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    config::get_log_level,
    state::{Gateway, LogState},
    util::RotatingList,
};

pub struct Logger {
    logs: RotatingList<String>,
    rx: Receiver<String>,
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
    fn new(cap: usize) -> Result<Logger, String> {
        let (tx, rx) = channel();
        log::set_boxed_logger(Box::new(LoggerProxy { tx }))
            .map(|_| log::set_max_level(get_log_level()))
            .map_err(|e| format!("{}", e))?;
        Ok(Logger {
            logs: RotatingList::new(cap),
            rx,
        })
    }

    pub fn logs(&self) -> Vec<String> {
        self.logs.iter().map(String::clone).collect()
    }

    pub fn init(cap: usize) -> Self {
        Logger::new(cap).expect("Installing logger failed")
    }

    pub fn check_logs(&mut self, store: &Gateway) {
        for log in self.rx.try_iter() {
            self.logs.push(log);
            store.update_log(LogState::new(self.logs()));
        }
    }
}

// impl log::Log for Logger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= Level::Info
//     }

//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             if let Ok(mut logs) = self.logs.lock() {
//                 let elem = format!("{} - {}", record.level(), record.args());
//                 logs.push(elem);
//             }
//         }
//     }

//     fn flush(&self) {}
// }
