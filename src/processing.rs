use log::debug;
use rdev::EventType;

use crate::types::Stats;

pub fn calculate_mouse_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn calculate_wheel_spins(delta_x: i64, delta_y: i64) -> i64 {
    delta_x.abs() + delta_y.abs()
}

pub fn process_event(event: EventType, s: &mut Stats, last_mouse_pos: &mut (f64, f64)) {
    match event {
        EventType::MouseMove { x, y } => {
            let distance = calculate_mouse_distance(last_mouse_pos.0, last_mouse_pos.1, x, y);
            if distance < 1.0 {
                return; // Ignore small movements
            }
            debug!(
                "Mouse moved from ({}, {}) to ({}, {}), Distance: {}",
                last_mouse_pos.0, last_mouse_pos.1, x, y, distance
            );
            *last_mouse_pos = (x, y);

            s.mouse_distance += distance as i64; // losing 1 pixel of precision here doesn't matter
        }
        EventType::Wheel { delta_x, delta_y } => {
            s.wheel_distance += calculate_wheel_spins(delta_x, delta_y);
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
