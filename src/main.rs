use std::io;

mod app;
mod clock;
mod config;
mod eco;
mod engine;
mod export;
mod turn;
mod ui;
mod util;

fn main() -> io::Result<()> {
    env_logger::init();
    let _ = crate::config::config();
    app::start_app()
}
