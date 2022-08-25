use std::time::Duration;

use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, Deref, DerefMut)]
pub struct TickTimer(Timer);

impl TickTimer {
    fn new() -> TickTimer {
        TickTimer(Timer::from_seconds(1.0, true))
    }

    pub fn speedup(&mut self) {
        self.set_duration(Duration::from_secs_f32(0.03));
    }

    pub fn end_speedup(&mut self) {
        self.set_duration(Duration::from_secs_f32(1.0));
    }
}

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Tick>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(tick_system));
    }
}

pub struct Tick;
fn tick_system(time: Res<Time>, mut writer: EventWriter<Tick>, mut timer: ResMut<TickTimer>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        writer.send(Tick);
    }
}

fn reset(mut commands: Commands) {
    commands.insert_resource(TickTimer::new())
}
