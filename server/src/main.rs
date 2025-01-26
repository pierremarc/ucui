mod config;
mod eco;
mod monitor;
mod play;
mod server;
mod state;

fn main() {
    let _ = crate::config::config();
    crate::server::start();
}
