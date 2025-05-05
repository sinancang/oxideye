use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub stats: StatsConfig,
}

#[derive(Deserialize)]
pub struct StatsConfig {
    pub dir: String,
    pub postfix: String,
}

#[derive(Parser)]
#[command(name = "peripheral-daemon")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(short, long, default_value = "config/default.toml")]
        config: String,

        #[arg(short, long, default_value = "daemon_log.txt")]
        log_file: String,

        #[arg(long, default_value = "info", value_parser = ["debug", "info"])]
        log_level: String,

        #[arg(long, default_value = "1000")]
        period_ms: u64,
    },

    Stop,
}

#[derive(Default)]
pub struct Stats {
    pub mouse_distance: i64,
    pub wheel_distance: i64,
    pub button_presses: i64,
    pub key_presses: i64,
}
