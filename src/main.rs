use std::io;

mod app;
mod clock;
mod config;
mod eco;
mod engine;
mod export;
mod logger;
mod simple;
mod state;
mod turn;
mod ui;
mod util;

fn main() -> io::Result<()> {
    let _ = crate::config::config();
    app::start_app()
}
