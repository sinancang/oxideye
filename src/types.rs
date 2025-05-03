use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub path: String,
    pub period_ms: u64,
}

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    pub config: String,

    #[arg(long, default_value = "info", value_parser = ["debug", "info"])]
    pub log_level: String,
}

#[derive(Default)]
pub struct Stats {
    pub mouse_distance: i64,
    pub wheel_distance: i64,
    pub button_presses: i64,
    pub key_presses: i64,
}