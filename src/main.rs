use clap::Parser;
use daemonize::Daemonize;
use log::{debug, info};
use oxideye::types::{Cli, Commands};

fn start_daemon(config: String, log_file: String, log_level: String, period_ms: u64) {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .unwrap_or_else(|e| panic!("Failed to open log file {}: {}", log_file, e));

    let daemonize = Daemonize::new()
        .pid_file("/tmp/daemon.pid")
        .stdout(
            log_file
                .try_clone()
                .expect("Failed to clone log file for stdout"),
        )
        .stderr(log_file);

    match daemonize.start() {
        Ok(_) => {
            info!("Daemon started successfully.");
            run_daemon(config, log_level, period_ms);
        }
        Err(e) => error!("Error daemonizing: {}", e),
    }
}

fn stop_daemon() {
    let pid_path = "/tmp/daemon.pid";
    let pid_str = fs::read_to_string(pid_path)
        .unwrap_or_else(|_| {
            error!("Could not read PID file at {}", pid_path);
            process::exit(1);
        })
        .trim()
        .to_string();

    let pid: i32 = pid_str.parse().unwrap_or_else(|_| {
        error!("Invalid PID in file: {}", pid_str);
        process::exit(1);
    });

    if let Err(e) = signal::kill(Pid::from_raw(pid), Signal::SIGTERM) {
        error!("Failed to send SIGTERM to PID {}: {}", pid, e);
        process::exit(1);
    }

    info!("Sent SIGTERM to daemon (PID {}).", pid);
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start {
            config,
            log_file,
            log_level,
            period_ms,
        } => {
            info!("Starting daemon");
            debug!("  Config: {}", config);
            debug!("  Log file: {}", log_file);
            debug!("  Log level: {}", log_level);
            debug!("  Period: {}ms", period_ms);
            start_daemon(config, log_file, log_level, period_ms);
        }

        Commands::Stop => {
            info!("Stopping daemon...");
            stop_daemon();
        }
    }
}
