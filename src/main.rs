use rdev::{Event, EventType, listen};

struct State {
    mouse_clicks: usize,
    key_strokes: usize,
    mouse_miles: usize,
    mouse_wheel_miles: usize,
}

fn callback(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            println!("Mouse moved to x = {:.0}, y = {:.0}", x, y);
        }
        EventType::Wheel { delta_x, delta_y } => {
            println!(
                "Mouse wheel: horizontal ticks = {}, vertical ticks = {}",
                delta_x, delta_y
            );
        }
        EventType::ButtonPress(button) => {
            println!("Mouse button pressed: {:?}", button);
        }
        EventType::ButtonRelease(button ) => {
            println!("Mouse button released: {:?}", button);
        }
        EventType::KeyPress(key) => {
            println!("Key pressed: {:?}", key);
        }
        EventType::KeyRelease(key) => {
            println!("Key released: {:?}", key);
        }
        _ => {} // ignore other events
    }
}

fn main() {
    println!("Listening…");
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:#?}", error);
    }
}
