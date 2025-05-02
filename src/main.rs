use chrono::Local;
use clap::Parser;
use crossbeam_channel::unbounded;
use env_logger::Builder;
use log::{LevelFilter, debug, error, info};
use rdev::{Event, EventType, listen};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Deserialize)]
struct Config {
    logging: LoggingConfig,
}

#[derive(Deserialize)]
struct LoggingConfig {
    path: String,
    period_ms: u64,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,

    #[arg(long, default_value = "info", value_parser = ["debug", "info"])]
    log_level: String,
}

#[derive(Default)]
struct State {
    mouse_distance: i64,
    wheel_distance: i64,
    button_presses: i64,
    key_presses: i64,
}

fn init_logger(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info, // default
    };

    Builder::new().filter(None, level).init();
}

fn main() {
    let args = Cli::parse();
    init_logger(&args.log_level);

    if !std::path::Path::new(&args.config).exists() {
        error!("Config file does not exist: {}", args.config);
        std::process::exit(1);
    }
    let config_contents = fs::read_to_string(args.config.clone())
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", args.config));
    let config: Config =
        toml::from_str(&config_contents).expect("Failed to parse config from TOML");

    let log_path: String = config.logging.path;
    let log_period: u64 = config.logging.period_ms;
    info!("Log level: {}", args.log_level);
    info!("Logging to {} every {} milliseconds", log_path, log_period);

    let state_for_event_listener = Arc::new(Mutex::new(State::default()));
    let state_for_logger = Arc::clone(&state_for_event_listener);

    thread::spawn(move || {
        logger_thread(log_path, log_period, state_for_logger);
    });

    let (tx, rx) = unbounded::<EventType>();
    thread::spawn(move || {
        event_listener(state_for_event_listener, rx);
    });

    let callback = {
        move |ev: Event| {
            tx.send(ev.event_type).ok();
        }
    };
    match listen(callback) {
        Ok(_) => {}
        Err(error) => {
            error!("Error occurred while listening for events: {:?}", error);
        }
    }
}

fn event_listener(state: Arc<Mutex<State>>, rx: crossbeam_channel::Receiver<EventType>) {
    let mut first_mouse_move = true;
    let mut last_mouse_pos = (0.0, 0.0);
    for event in rx {
        let mut s = state.lock().expect("Mutex poisoned while locking state");
        process_event(event, &mut s, &mut first_mouse_move, &mut last_mouse_pos);
    }
}

fn logger_thread(log_path: String, log_period: u64, state: Arc<Mutex<State>>) {
    loop {
        thread::sleep(std::time::Duration::from_millis(log_period));

        let s = state.lock().expect("Mutex poisoned while locking state");

        // Create updated log object
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let updated_log = json!({
            "timestamp": timestamp,
            "mouse_distance": s.mouse_distance,
            "wheel_spins": s.wheel_distance,
            "button_presses": s.button_presses,
            "key_presses": s.key_presses
        });
        debug!("Updated log: {}", updated_log);

        // Open and read file
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&log_path)
            .expect("Failed to open log file");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let data: Value = if contents.trim().is_empty() {
            updated_log.clone()
        } else {
            let mut val: Value = serde_json::from_str(&contents).unwrap_or(json!({}));
            val["timestamp"] = updated_log["timestamp"].clone();
            val["mouse_distance"] = updated_log["mouse_distance"].clone();
            val["wheel_spins"] = updated_log["wheel_spins"].clone();
            val["button_presses"] = updated_log["button_presses"].clone();
            val["key_presses"] = updated_log["key_presses"].clone();
            val
        };

        // Write updated JSON
        file.set_len(0).expect("Failed to truncate file");
        file.seek(std::io::SeekFrom::Start(0))
            .expect("Failed to seek start");
        write!(file, "{}", data.to_string()).expect("Failed to write updated JSON");
    }
}

fn process_event(
    event: EventType,
    s: &mut State,
    first_mouse_move: &mut bool,
    last_mouse_pos: &mut (f64, f64),
) {
    match event {
        EventType::MouseMove { x, y } => {
            let mut distance = 0.0;
            if !*first_mouse_move {
                distance = ((x - last_mouse_pos.0).powi(2) + (y - last_mouse_pos.1).powi(2)).sqrt();
                debug!(
                    "Mouse moved from ({}, {}) to ({}, {}), Distance: {}",
                    last_mouse_pos.0, last_mouse_pos.1, x, y, distance
                );
                *last_mouse_pos = (x, y);
            } else {
                *last_mouse_pos = (x, y);
                *first_mouse_move = false;
            }
            s.mouse_distance += distance as i64; // losing 1 pixel of precision here doesn't matter
        }
        EventType::Wheel { delta_x, delta_y } => {
            s.wheel_distance += delta_x.abs() + delta_y.abs();
            debug!("Wheel moved by ({}, {})", delta_x, delta_y);
        }
        EventType::ButtonPress(_) => s.button_presses += 1,
        EventType::KeyPress(_) => s.key_presses += 1,
        _ => {}
    }
    debug!(
        "Mouse Distance: {}, Wheel Spins: {}, Button Presses: {}, Key Presses: {}",
        s.mouse_distance, s.wheel_distance, s.button_presses, s.key_presses
    );
}
