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

    #[arg(long)]
    option_debug_log: Option<String>,
    // #[arg(long)]
    // option_contempt: Option<String>,
    // #[arg(long)]
    // option_analysis_contempt: Option<String>,
    #[arg(long)]
    option_threads: Option<String>,
    #[arg(long)]
    option_hash: Option<String>,
    // #[arg(long)]
    // option_clear_hash: Option<String>,
    // #[arg(long)]
    // option_ponder: Option<String>,
    // #[arg(long)]
    // option_multi_pv: Option<String>,
    #[arg(long)]
    option_skill_level: Option<String>,
    #[arg(long)]
    option_move_overhead: Option<String>,
    #[arg(long)]
    option_slow_mover: Option<String>,
    #[arg(long)]
    option_nodestime: Option<String>,
    // #[arg(long)]
    // option_uci_chess960: Option<String>,
    // #[arg(long)]
    // option_uci_analyse_mode: Option<String>,
    #[arg(long)]
    option_uci_limit_strength: Option<String>,
    #[arg(long)]
    option_uci_elo: Option<String>,
    // #[arg(long)]
    // option_uci_show_wdl: Option<String>,
    #[arg(long)]
    option_syzygy_path: Option<String>,
    #[arg(long)]
    option_syzygy_probe_depth: Option<String>,
    #[arg(long)]
    option_syzygy50_move_rule: Option<String>,
    #[arg(long)]
    option_syzygy_probe_limit: Option<String>,
    #[arg(long)]
    option_use_nnue: Option<String>,
    #[arg(long)]
    option_eval_file: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Play,
}

#[derive(Clone)]
pub enum UciOption {
    DebugLog(String),
    // Contempt(String),
    // AnalysisContempt(String),
    Threads(String),
    Hash(String),
    // ClearHash(String),
    // Ponder(String),
    // MultiPV(String),
    SkillLevel(String),
    MoveOverhead(String),
    SlowMover(String),
    Nodestime(String),
    // UCIChess960(String),
    // UCIAnalyseMode(String),
    UCILimitStrength(String),
    UCIElo(String),
    // UCIShowWDL(String),
    SyzygyPath(String),
    SyzygyProbeDepth(String),
    Syzygy50MoveRule(String),
    SyzygyProbeLimit(String),
    UseNNUE(String),
    EvalFile(String),
}

impl UciOption {
    pub fn id(&self) -> &'static str {
        match self {
            UciOption::DebugLog(_) => "Debug Log",
            // UciOption::Contempt(_) => "Contempt",
            // UciOption::AnalysisContempt(_) => "Analysis Contempt",
            UciOption::Threads(_) => "Threads",
            UciOption::Hash(_) => "Hash",
            // UciOption::ClearHash(_) => "Clear Hash",
            // UciOption::Ponder(_) => "Ponder",
            // UciOption::MultiPV(_) => "MultiPV",
            UciOption::SkillLevel(_) => "Skill Level",
            UciOption::MoveOverhead(_) => "Move Overhead",
            UciOption::SlowMover(_) => "Slow Mover",
            UciOption::Nodestime(_) => "nodestime",
            // UciOption::UCIChess960(_) => "UCI_Chess960",
            // UciOption::UCIAnalyseMode(_) => "UCI_AnalyseMode",
            UciOption::UCILimitStrength(_) => "UCI_LimitStrength",
            UciOption::UCIElo(_) => "UCI_Elo",
            // UciOption::UCIShowWDL(_) => "UCI_ShowWDL",
            UciOption::SyzygyPath(_) => "SyzygyPath",
            UciOption::SyzygyProbeDepth(_) => "SyzygyProbeDepth",
            UciOption::Syzygy50MoveRule(_) => "Syzygy50MoveRule",
            UciOption::SyzygyProbeLimit(_) => "SyzygyProbeLimit",
            UciOption::UseNNUE(_) => "Use NNUE",
            UciOption::EvalFile(_) => "EvalFil",
        }
    }
    pub fn value(&self) -> &String {
        match self {
            UciOption::DebugLog(value) => value,
            // UciOption::Contempt(value) => value,
            // UciOption::AnalysisContempt(value) => value,
            UciOption::Threads(value) => value,
            UciOption::Hash(value) => value,
            // UciOption::ClearHash(value) => value,
            // UciOption::Ponder(value) => value,
            // UciOption::MultiPV(value) => value,
            UciOption::SkillLevel(value) => value,
            UciOption::MoveOverhead(value) => value,
            UciOption::SlowMover(value) => value,
            UciOption::Nodestime(value) => value,
            // UciOption::UCIChess960(value) => value,
            // UciOption::UCIAnalyseMode(value) => value,
            UciOption::UCILimitStrength(value) => value,
            UciOption::UCIElo(value) => value,
            // UciOption::UCIShowWDL(value) => value,
            UciOption::SyzygyPath(value) => value,
            UciOption::SyzygyProbeDepth(value) => value,
            UciOption::Syzygy50MoveRule(value) => value,
            UciOption::SyzygyProbeLimit(value) => value,
            UciOption::UseNNUE(value) => value,
            UciOption::EvalFile(value) => value,
        }
    }
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

pub fn get_engine_options() -> Vec<UciOption> {
    let mut options: Vec<UciOption> = Vec::new();
    let cfg = config();
    if let Some(value) = cfg.option_debug_log.clone() {
        options.push(UciOption::DebugLog(value));
    };
    // if let Some(value) = cfg.option_contempt.clone() {
    //     options.push(UciOption::Contempt(value));
    // };
    // if let Some(value) = cfg.option_analysis_contempt.clone() {
    //     options.push(UciOption::AnalysisContempt(value));
    // };
    if let Some(value) = cfg.option_threads.clone() {
        options.push(UciOption::Threads(value));
    };
    if let Some(value) = cfg.option_hash.clone() {
        options.push(UciOption::Hash(value));
    };
    // if let Some(value) = cfg.option_clear_hash.clone() {
    //     options.push(UciOption::ClearHash(value));
    // };
    // if let Some(value) = cfg.option_ponder.clone() {
    //     options.push(UciOption::Ponder(value));
    // };
    // if let Some(value) = cfg.option_multi_pv.clone() {
    //     options.push(UciOption::MultiPV(value));
    // };
    if let Some(value) = cfg.option_skill_level.clone() {
        options.push(UciOption::SkillLevel(value));
    };
    if let Some(value) = cfg.option_move_overhead.clone() {
        options.push(UciOption::MoveOverhead(value));
    };
    if let Some(value) = cfg.option_slow_mover.clone() {
        options.push(UciOption::SlowMover(value));
    };
    if let Some(value) = cfg.option_nodestime.clone() {
        options.push(UciOption::Nodestime(value));
    };
    // if let Some(value) = cfg.option_uci_chess960.clone() {
    //     options.push(UciOption::UCIChess960(value));
    // };
    // if let Some(value) = cfg.option_uci_analyse_mode.clone() {
    //     options.push(UciOption::UCIAnalyseMode(value));
    // };
    if let Some(value) = cfg.option_uci_limit_strength.clone() {
        options.push(UciOption::UCILimitStrength(value));
    };
    if let Some(value) = cfg.option_uci_elo.clone() {
        options.push(UciOption::UCIElo(value));
    };
    // if let Some(value) = cfg.option_uci_show_wdl.clone() {
    //     options.push(UciOption::UCIShowWDL(value));
    // };
    if let Some(value) = cfg.option_syzygy_path.clone() {
        options.push(UciOption::SyzygyPath(value));
    };
    if let Some(value) = cfg.option_syzygy_probe_depth.clone() {
        options.push(UciOption::SyzygyProbeDepth(value));
    };
    if let Some(value) = cfg.option_syzygy50_move_rule.clone() {
        options.push(UciOption::Syzygy50MoveRule(value));
    };
    if let Some(value) = cfg.option_syzygy_probe_limit.clone() {
        options.push(UciOption::SyzygyProbeLimit(value));
    };
    if let Some(value) = cfg.option_use_nnue.clone() {
        options.push(UciOption::UseNNUE(value));
    };
    if let Some(value) = cfg.option_eval_file.clone() {
        options.push(UciOption::EvalFile(value));
    };
    options
}