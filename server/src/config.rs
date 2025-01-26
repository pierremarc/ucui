use clap::Parser;
// use log::LevelFilter;
use std::{net::IpAddr, path::PathBuf, sync::OnceLock};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Interface to bind to  
    #[arg(short, long, value_name = "INTERFACE", default_value = "0.0.0.0")]
    interface: IpAddr,

    /// Port to bind to  
    #[arg(short, long, value_name = "PORT", default_value = "8000")]
    port: u16,

    /// Path to the static files directory
    #[arg(short, long, value_name = "DIR")]
    static_dir: Option<PathBuf>,

    /// Path to a UCI engine
    #[arg(short, long, value_name = "ENGINE")]
    engine: String,

    /// Optional arguments to pass to the engine (separated by ";")
    ///
    /// Example: --engine-args '--uci;--quiet'
    #[arg(long, value_name = "ARGS", allow_hyphen_values = true)]
    engine_args: Option<String>,

    // set log level
    // #[arg(long, value_name = "LOG_LEVEL", default_value = "info")]
    // log_level: LogLevel,
    /// UCI option
    ///
    /// This argument can be repeated. UCI options are of the
    /// form "ID[:VALUE]". VALUE can be missing if not needed (buttons).  
    /// See the engine's documentation for available options and their
    /// default values.
    ///
    /// Example: --uci-option 'Threads:2' --uci-option 'Skill Level:12'
    #[arg(long)]
    uci_option: Vec<String>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(Config::parse)
}

pub fn get_interface() -> IpAddr {
    config().interface
}

pub fn get_port() -> u16 {
    config().port
}

pub fn get_static_dir() -> String {
    config()
        .static_dir
        .clone()
        .and_then(|path| path.as_os_str().to_str().map(String::from))
        .unwrap_or(format!(
            "{}/../clients/apps/dist",
            env!("CARGO_MANIFEST_DIR")
        ))
}

pub fn get_engine() -> String {
    config().engine.clone()
}

pub fn get_engine_args() -> Option<Vec<String>> {
    config()
        .engine_args
        .clone()
        .map(|args| args.split(";").map(|arg| arg.to_string()).collect())
}

pub fn get_engine_options() -> Vec<(String, Option<String>)> {
    config()
        .uci_option
        .iter()
        .map(|opt| {
            let parts: Vec<String> = opt.split(":").take(2).map(|s| s.to_string()).collect();
            match parts.len() {
                0 => (String::new(), None),
                1 => (parts[0].clone(), None),
                _ => (parts[0].clone(), Some(parts[1].clone())),
            }
        })
        .collect()
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum LogLevel {
//     /// A level lower than all log levels.
//     Off,
//     /// Corresponds to the `Error` log level.
//     Error,
//     /// Corresponds to the `Warn` log level.
//     Warn,
//     /// Corresponds to the `Info` log level.
//     Info,
//     /// Corresponds to the `Debug` log level.
//     Debug,
//     /// Corresponds to the `Trace` log level.
//     Trace,
// }

// pub fn get_log_level() -> LevelFilter {
//     match config().log_level {
//         LogLevel::Off => LevelFilter::Off,
//         LogLevel::Error => LevelFilter::Error,
//         LogLevel::Warn => LevelFilter::Warn,
//         LogLevel::Info => LevelFilter::Info,
//         LogLevel::Debug => LevelFilter::Debug,
//         LogLevel::Trace => LevelFilter::Trace,
//     }
// }
