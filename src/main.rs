use crossbeam_channel::{Sender, unbounded};
use rdev::{Event, EventType, listen};
use std::thread;

#[derive(Default)]
struct State {
    mouse_moves: usize,
    wheel_spins: usize,
    button_presses: usize,
    key_presses: usize,
}

enum Msg {
    MouseMove,
    WheelSpin,
    ButtonPress,
    KeyPress,
}

fn main() {
    // 1. spawn the worker that *owns* State
    let (tx, rx) = unbounded::<Msg>();
    thread::spawn(move || {
        let mut s = State::default();
        for msg in rx {
            match msg {
                Msg::MouseMove => s.mouse_moves += 1,
                Msg::WheelSpin => s.wheel_spins += 1,
                Msg::ButtonPress => s.button_presses += 1,
                Msg::KeyPress => s.key_presses += 1,
            }
            println!(
                "Mouse Moves: {}, Wheel Spins: {}, Button Presses: {}, Key Presses: {}",
                s.mouse_moves, s.wheel_spins, s.button_presses, s.key_presses
            );

            // TODO: down the line, we will log to a DB or file periodically (every N messages/seconds)
        }
    });

    // 2. lightweight callback
    let callback = move |ev: Event| send_event(&tx, &ev);
    listen(callback).unwrap();
}

fn send_event(tx: &Sender<Msg>, ev: &Event) {
    match ev.event_type {
        EventType::MouseMove { x, y } => {
            tx.send(Msg::MouseMove).ok();
        }
        EventType::Wheel { delta_x, delta_y } => {
            tx.send(Msg::WheelSpin).ok();
        }
        EventType::ButtonPress(_) => {
            tx.send(Msg::ButtonPress).ok();
        }
        EventType::KeyPress(_) => {
            tx.send(Msg::KeyPress).ok();
        }
        _ => {}
    }
}
