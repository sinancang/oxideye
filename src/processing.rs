use log::debug;
use rdev::EventType;

use serde_json::{Value, json};

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

/// Merge the latest sample (`update`) into the running total (`existing`).
/// • timestamp is always **replaced**  
/// • counters are **accumulated** (ignored if the increment is 0)
pub fn merge_stats(mut existing: Value, update: &Value) -> Value {
    if let Some(ts) = update.get("timestamp") {
        existing["timestamp"] = ts.clone();
    }

    for k in [
        "mouse_distance",
        "wheel_spins",
        "button_presses",
        "key_presses",
    ] {
        if let Some(add) = update.get(k).and_then(Value::as_i64) {
            if add != 0 {
                let cur = existing.get(k).and_then(Value::as_i64).unwrap_or(0);
                existing[k] = json!(cur + add);
            }
        }
    }
    existing
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        stats: Stats,
        last_mouse_pos: (f64, f64),
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                stats: Stats::default(),
                last_mouse_pos: (0.0, 0.0),
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            println!(
                "TestContext teardown -> mouse: {}, wheel: {}, buttons: {}, keys: {}",
                self.stats.mouse_distance,
                self.stats.wheel_distance,
                self.stats.button_presses,
                self.stats.key_presses,
            );
        }
    }

    #[test]
    fn test_calculate_mouse_distance() {
        let dist = calculate_mouse_distance(0.0, 0.0, 3.0, 4.0);
        assert_eq!(dist, 5.0);
    }

    #[test]
    fn test_calculate_wheel_spins() {
        assert_eq!(calculate_wheel_spins(5, -2), 7);
        assert_eq!(calculate_wheel_spins(0, 0), 0);
    }

    #[test]
    fn test_process_mouse_move() {
        let mut ctx = TestContext::new();

        process_event(
            EventType::MouseMove { x: 0.0, y: 0.0 },
            &mut ctx.stats,
            &mut ctx.last_mouse_pos,
        );
        assert_eq!(ctx.stats.mouse_distance, 0);

        process_event(
            EventType::MouseMove { x: 3.0, y: 4.0 },
            &mut ctx.stats,
            &mut ctx.last_mouse_pos,
        );
        assert_eq!(ctx.stats.mouse_distance, 5);
    }

    #[test]
    fn test_process_wheel_event() {
        let mut ctx = TestContext::new();

        process_event(
            EventType::Wheel {
                delta_x: 3,
                delta_y: -4,
            },
            &mut ctx.stats,
            &mut ctx.last_mouse_pos,
        );
        assert_eq!(ctx.stats.wheel_distance, 7);
    }

    #[test]
    fn test_process_button_press() {
        let mut ctx = TestContext::new();

        process_event(
            EventType::ButtonPress(rdev::Button::Left),
            &mut ctx.stats,
            &mut ctx.last_mouse_pos,
        );
        assert_eq!(ctx.stats.button_presses, 1);
    }

    #[test]
    fn test_process_key_press() {
        let mut ctx = TestContext::new();

        process_event(
            EventType::KeyPress(rdev::Key::KeyA),
            &mut ctx.stats,
            &mut ctx.last_mouse_pos,
        );
        assert_eq!(ctx.stats.key_presses, 1);
    }

    #[test]
    fn into_empty() {
        let res = merge_stats(
            json!({}),
            &json!({
                "timestamp": 1, "mouse_distance": 5, "wheel_spins": 2,
                "button_presses": 3, "key_presses": 4
            }),
        );
        assert_eq!(res["timestamp"], 1);
        assert_eq!(res["wheel_spins"], 2);
        assert_eq!(res["button_presses"], 3);
        assert_eq!(res["key_presses"], 4);
        assert_eq!(res["mouse_distance"], 5);
    }

    #[test]
    fn accumulates_counters() {
        let res = merge_stats(
            json!({ "timestamp": 0, "wheel_spins": 1 }),
            &json!({ "timestamp": 9, "wheel_spins": 2 }),
        );
        assert_eq!(res["wheel_spins"], 3);
        assert_eq!(res["timestamp"], 9);
    }

    #[test]
    fn updates_timestamp() {
        let res = merge_stats(
            json!({ "timestamp": 0, "wheel_spins": 1 }),
            &json!({ "timestamp": 9, "wheel_spins": 2 }),
        );
        assert_eq!(res["timestamp"], 9);
    }

    #[test]
    fn skips_zero() {
        let res = merge_stats(
            json!({ "timestamp": 0, "key_presses": 7 }),
            &json!({ "timestamp": 3, "key_presses": 0 }),
        );
        assert_eq!(res["key_presses"], 7);
    }
}
