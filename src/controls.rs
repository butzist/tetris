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

fn controls(keys: Res<Input<KeyCode>>, mut events: EventWriter<ControlEvent>) {
    if keys.just_pressed(KeyCode::Space) {
        events.send(ControlEvent::Pause);
    }

    if keys.just_pressed(KeyCode::Down) {
        events.send(ControlEvent::SpeedupStart);
    }

    if keys.just_released(KeyCode::Down) {
        events.send(ControlEvent::SpeedupEnd);
    }

    if keys.just_pressed(KeyCode::Right) {
        events.send(ControlEvent::Right);
    }

    if keys.just_pressed(KeyCode::Left) {
        events.send(ControlEvent::Left);
    }

    if keys.just_pressed(KeyCode::Up) {
        events.send(ControlEvent::RotateRight);
    }

    let shift = keys.any_pressed([KeyCode::LShift, KeyCode::RShift]);
    if shift && keys.pressed(KeyCode::Up) {
        events.send(ControlEvent::RotateLeft);
    }
}
