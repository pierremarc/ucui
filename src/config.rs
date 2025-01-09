use std::{path::PathBuf, str::FromStr, sync::OnceLock};

use clap::{Parser, Subcommand};
use shakmaty::fen::Fen;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path to a UCI engine
    #[arg(short, long, value_name = "ENGINE")]
    engine: PathBuf,

    /// White time in seconds
    #[arg(short, long, value_name = "WHITE_TIME")]
    white_time: i64,

    /// Black time in seconds
    #[arg(short, long, value_name = "WHITE_TIME")]
    black_time: i64,

    /// Optional starting position in FEN format
    #[arg(short, long, value_name = "FEN")]
    fen: Option<String>,

    /// Command: [play; ...]
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Play,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn init_table() -> Config {
    Config::parse()
}

pub fn config() -> &'static Config {
    CONFIG.get_or_init(init_table)
}

pub fn get_engine() -> &'static str {
    config()
        .engine
        .as_os_str()
        .to_str()
        .expect("Engine to have a good path")
}

pub fn get_time_white() -> i64 {
    std::cmp::max(0, config().white_time)
}

pub fn get_time_black() -> i64 {
    std::cmp::max(0, config().black_time)
}

pub fn get_start_pos() -> Option<Fen> {
    config()
        .fen
        .clone()
        .and_then(|fen| Fen::from_str(&fen).ok())
}

// pub fn get_name() -> Option<String> {
//     config().name.clone()
// }
