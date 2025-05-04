use oxideye::processing::{calculate_mouse_distance, calculate_wheel_spins, process_event};
use oxideye::types::Stats;
use rdev::EventType;

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
