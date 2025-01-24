mod config;
mod eco;
mod play;
mod server;

fn main() {
    let _ = crate::config::config();
    crate::server::start();
}
