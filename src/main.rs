use crossbeam_channel::{Sender, unbounded};
use rdev::{Event, EventType, listen};
use std::thread;
use std::sync::{Arc, Mutex};
use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use std::fs;


#[derive(Deserialize)]
struct Config {
    logging: LoggingConfig,
}

#[derive(Deserialize)]
struct LoggingConfig {
    path: String,
    period_ms: i32,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,
}

#[derive(Default)]
struct State {
    mouse_distance: usize,
    wheel_distance: usize,
    button_presses: usize,
    key_presses: usize,
}


fn main() {
    let args = Cli::parse();
    let config_contents = fs::read_to_string(args.config).unwrap();
    let config: Config = toml::from_str(&config_contents).unwrap();
    println!("Logging to {} every {} milliseconds", config.logging.path, config.logging.period_ms);

    let log_path = config.logging.path.clone();
    let log_period = config.logging.period_ms;

    let state = Arc::new(Mutex::new(State::default()));
    let state_clone = Arc::clone(&state);

    // logger thread
    thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_millis(log_period as u64));

            let mut s = state_clone.lock().unwrap();
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log = format!(
                "[{}] Mouse Distance: {}, Wheel Spins: {}, Button Presses: {}, Key Presses: {}\n",
                timestamp, s.mouse_distance, s.wheel_distance, s.button_presses, s.key_presses
            );
    
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&log_path)
                .and_then(|mut f| std::io::Write::write_all(&mut f, log.as_bytes()))
                .expect("Failed to write log");
    
            // Reset counters
            println!("Resetting state after logging");
            *s = State::default();
        }
    });

    // event listener thread
    let (tx, rx) = unbounded::<rdev::EventType>();
    thread::spawn(move || {
        let mut first_mouse_move = true;
        let mut last_mouse_pos = (0.0, 0.0);
        for event in rx {
            let mut s = state.lock().unwrap();
            process_event(event, &mut s, &mut first_mouse_move, &mut last_mouse_pos);
        }
    });

    // callback for event listener
    let callback = move |ev: Event| send_event(&tx, &ev);
    listen(callback).unwrap();
}

fn send_event(tx: &Sender<EventType>, ev: &Event) {
    tx.send(ev.event_type.clone()).ok();
}

fn process_event(event: EventType, s: &mut State, first_mouse_move: &mut bool, last_mouse_pos: &mut (f64, f64)) {
    match event {
        EventType::MouseMove { x, y } => {
            let mut distance = 0.0;
            if !*first_mouse_move {
                distance = ((x - last_mouse_pos.0).powi(2) + (y - last_mouse_pos.1).powi(2)).sqrt();
                println!(
                    "Mouse moved from ({}, {}) to ({}, {}), Distance: {}",
                    last_mouse_pos.0, last_mouse_pos.1, x, y, distance
                );
                *last_mouse_pos = (x, y);
            } else {
                *last_mouse_pos = (x, y);
                *first_mouse_move = false;
            }
            s.mouse_distance += distance as usize;
        }
        EventType::Wheel { delta_x, delta_y } => {
            s.wheel_distance += (delta_x.abs() + delta_y.abs()) as usize;
            println!("Wheel moved by ({}, {})", delta_x, delta_y);
        }
        EventType::ButtonPress(_) => s.button_presses += 1,
        EventType::KeyPress(_) => s.key_presses += 1,
        _ => {}
    }
    println!(
        "Mouse Distance: {}, Wheel Spins: {}, Button Presses: {}, Key Presses: {}",
        s.mouse_distance, s.wheel_distance, s.button_presses, s.key_presses
    );
}
