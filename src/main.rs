use chrono::Local;
use clap::Parser;
use crossbeam_channel::unbounded;
use env_logger::Builder;
use log::{LevelFilter, debug, error, info};
use rdev::{Event, EventType, listen};
use serde_json::{Value, json};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use oxideye::processing::process_event;
use oxideye::types::{Cli, Config, Stats};


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

    let state_for_event_listener = Arc::new(Mutex::new(Stats::default()));
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

fn event_listener(stats: Arc<Mutex<Stats>>, rx: crossbeam_channel::Receiver<EventType>) {
    let mut last_mouse_pos = (0.0, 0.0);
    for event in rx {
        let mut s = stats.lock().expect("Mutex poisoned while locking state");
        process_event(event, &mut s, &mut last_mouse_pos);
    }
}

fn logger_thread(log_path: String, log_period: u64, state: Arc<Mutex<Stats>>) {
    loop {
        thread::sleep(std::time::Duration::from_millis(log_period));

        let mut s = state.lock().expect("Mutex poisoned while locking state");

        // Create updated log object
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let json_update = json!({
            "timestamp": timestamp,
            "mouse_distance": s.mouse_distance,
            "wheel_spins": s.wheel_distance,
            "button_presses": s.button_presses,
            "key_presses": s.key_presses
        });
        debug!("Updating JSON with: {}", json_update);

        // Open and read file
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&log_path)
            .expect("Failed to open log file");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let data: Value = if contents.trim().is_empty() {
            debug!(
                "File is empty, creating new JSON object with initial values: {}",
                json_update
            );
            json_update.clone()
        } else {
            let mut val: Value = serde_json::from_str(&contents).unwrap_or(json!({}));
            debug!(
                "Timestamp updated: from {} to {}",
                val["timestamp"], json_update["timestamp"]
            );
            val["timestamp"] = json_update["timestamp"].clone();
            add_field(&mut val, "mouse_distance", &json_update);
            add_field(&mut val, "wheel_spins", &json_update);
            add_field(&mut val, "button_presses", &json_update);
            add_field(&mut val, "key_presses", &json_update);
            val
        };

        // Write updated JSON
        file.set_len(0).expect("Failed to truncate file");
        file.seek(std::io::SeekFrom::Start(0))
            .expect("Failed to seek start");
        write!(file, "{}", data).expect("Failed to write updated JSON");

        *s = Stats::default();
    }
}

fn add_field(val: &mut Value, key: &str, json_update: &Value) {
    let update = json_update.get(key).and_then(Value::as_i64).unwrap_or(0);
    if update == 0 {
        return;
    }
    let current = val.get(key).and_then(Value::as_i64).unwrap_or(0);
    debug!(
        "Adding field `{}` with value {} to {}",
        key, update, current
    );
    val[key] = json!(current + update);
}
