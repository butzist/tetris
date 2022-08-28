use std::time::Duration;

use bevy::prelude::*;

use crate::{controls::ControlEvent, GameState, GameStats};

#[derive(Debug)]
pub struct TickTimer {
    timer: Timer,
    in_speedup: bool,
}

impl Default for TickTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, true),
            in_speedup: false,
        }
    }
}

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Tick>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(tick_system)
                    .with_system(speedup),
            );
    }
}

#[derive(Default)]
pub struct Tick;
fn tick_system(
    time: Res<Time>,
    mut writer: EventWriter<Tick>,
    mut timer: ResMut<TickTimer>,
    stats: Res<GameStats>,
) {
    let speed = 1.0 + stats.shapes_spawned as f32 * 0.02;
    let time_step = if timer.in_speedup { 0.03 } else { 1.0 / speed };

    timer.timer.set_duration(Duration::from_secs_f32(time_step));
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        writer.send_default();
    }
}

fn reset(mut commands: Commands) {
    commands.insert_resource(TickTimer::default())
}

fn speedup(mut timer: ResMut<TickTimer>, mut control_events: EventReader<ControlEvent>) {
    for event in control_events.iter() {
        match event {
            ControlEvent::SpeedupStart => timer.in_speedup = true,
            ControlEvent::SpeedupEnd => timer.in_speedup = false,
            _ => (),
        }
    }
}
