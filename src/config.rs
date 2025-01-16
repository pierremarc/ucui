use std::{path::PathBuf, str::FromStr, sync::OnceLock};

use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;
use shakmaty::{fen::Fen, Chess, Color, FromSetup};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path to a UCI engine
    #[arg(short, long, value_name = "ENGINE")]
    engine: Option<PathBuf>,

    /// White time in seconds
    #[arg(short, long, value_name = "TIME", default_value = "600")]
    white_time: i64,

    /// Black time in seconds
    #[arg(short, long, value_name = "TIME", default_value = "600")]
    black_time: i64,

    /// set engine color
    #[arg(short = 'c', long, value_name = "COLOR", default_value = "black")]
    engine_color: EngineColor,

    /// Optional starting position in FEN format
    #[arg(short, long, value_name = "FEN")]
    fen: Option<String>,

    /// Optional arguments to pass to the engine (separated by ";")
    ///
    /// Example: --engine-args '--uci;--quiet'
    #[arg(long, value_name = "ARGS", allow_hyphen_values = true)]
    engine_args: Option<String>,

    /// set log level
    #[arg(long, value_name = "LOG_LEVEL", default_value = "info")]
    log_level: LogLevel,

    /// UCI option
    ///
    /// This argument can be repeated. UCI options are of the form KEY:VALUE.
    /// See the engine's documentation for available options and their default values.
    ///
    /// Example: --uci-option 'Threads:2' --uci-option 'Skill Level:12'
    #[arg(long)]
    uci_option: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Play,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EngineColor {
    /// Engine takes white
    White,
    /// Engine takes black
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn init_table() -> Config {
    Config::parse()
}

pub fn config() -> &'static Config {
    CONFIG.get_or_init(init_table)
}

pub fn get_engine() -> Option<String> {
    config()
        .engine
        .clone()
        .and_then(|path| path.as_os_str().to_str().map(String::from))
}

pub fn get_engine_args() -> Option<Vec<String>> {
    config()
        .engine_args
        .clone()
        .map(|args| args.split(";").map(|arg| arg.to_string()).collect())
}

pub fn get_engine_color() -> Color {
    match config().engine_color {
        EngineColor::Black => Color::Black,
        EngineColor::White => Color::White,
    }
}

pub fn get_time_white() -> i64 {
    std::cmp::max(0, config().white_time)
}

pub fn get_time_black() -> i64 {
    std::cmp::max(0, config().black_time)
}

pub fn get_start_pos() -> Option<Chess> {
    config()
        .fen
        .clone()
        .and_then(|fen| Fen::from_str(&fen).ok())
        .and_then(|fen| {
            Chess::from_setup(fen.as_setup().clone(), shakmaty::CastlingMode::Standard).ok()
        })
}

pub fn get_log_level() -> LevelFilter {
    match config().log_level {
        LogLevel::Off => LevelFilter::Off,
        LogLevel::Error => LevelFilter::Error,
        LogLevel::Warn => LevelFilter::Warn,
        LogLevel::Info => LevelFilter::Info,
        LogLevel::Debug => LevelFilter::Debug,
        LogLevel::Trace => LevelFilter::Trace,
    }
}

pub fn get_engine_options() -> Vec<(String, String)> {
    config()
        .uci_option
        .iter()
        .map(|opt| {
            let parts: Vec<String> = opt.split(":").map(|s| s.to_string()).collect();
            if parts.len() != 2 {
                (parts[0].clone(), parts[1].clone())
            } else {
                (String::new(), String::new())
            }
        })
        .collect()
}
