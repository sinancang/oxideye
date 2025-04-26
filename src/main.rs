use crossbeam_channel::{Sender, unbounded};
use rdev::{Event, EventType, listen};
use std::thread;

#[derive(Default)]
struct State {
    mouse_distance: usize,
    wheel_distance: usize,
    button_presses: usize,
    key_presses: usize,
    // TODO: key and button press total durations
    // potential TODO: per-key/button stats
}

fn main() {
    let (tx, rx) = unbounded::<rdev::EventType>();
    thread::spawn(move || {
        let mut s = State::default();
        let mut first_mouse_move = true;
        let mut last_mouse_pos = (0.0, 0.0);
        for event in rx {
            process_event(event, &mut s, &mut first_mouse_move, &mut last_mouse_pos);
        }
    });

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
