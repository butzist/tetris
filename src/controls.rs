use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControlEvent>().add_system(controls);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ControlEvent {
    SpeedupStart,
    SpeedupEnd,
    Pause,
    Left,
    Right,
    RotateRight,
    RotateLeft,
}

fn controls(
    keys: Res<Input<KeyCode>>,
    mut events: EventWriter<ControlEvent>,
    time: Res<Time>,
    mut repeat_timer: Local<Timer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        events.send(ControlEvent::Pause);
    }

    if keys.just_pressed(KeyCode::Down) {
        events.send(ControlEvent::SpeedupStart);
    }

    if keys.just_released(KeyCode::Down) {
        events.send(ControlEvent::SpeedupEnd);
    }

    let shift = keys.any_pressed([KeyCode::LShift, KeyCode::RShift]);
    let mut handle_repeating_key = |key_code: KeyCode, control_event: ControlEvent, delay: f32| {
        if keys.just_pressed(key_code) {
            events.send(control_event);
            *repeat_timer = Timer::from_seconds(delay, false);
            true
        } else if keys.pressed(key_code) {
            repeat_timer.tick(time.delta());
            if repeat_timer.just_finished() {
                events.send(control_event);
                *repeat_timer = Timer::from_seconds(0.1, false);
            }
            true
        } else {
            false
        }
    };

    let _ = handle_repeating_key(KeyCode::Right, ControlEvent::Right, 0.3)
        || handle_repeating_key(KeyCode::Left, ControlEvent::Left, 0.3)
        || handle_repeating_key(KeyCode::Up, ControlEvent::RotateRight, 0.6)
        || (shift && handle_repeating_key(KeyCode::Up, ControlEvent::RotateLeft, 0.6));
}
