use std::time::Duration;

use bevy::prelude::*;

pub struct TickTimer(Timer);

impl TickTimer {
    fn new() -> TickTimer {
        TickTimer(Timer::from_seconds(1.0, true))
    }

    pub fn speedup(&mut self) {
        self.0.set_duration(Duration::from_secs_f32(0.03));
    }

    pub fn end_speedup(&mut self) {
        self.0.set_duration(Duration::from_secs_f32(1.0));
    }
}

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer::new())
            .add_event::<Tick>()
            .add_system(tick_system);
    }
}

pub struct Tick;
fn tick_system(time: Res<Time>, mut writer: EventWriter<Tick>, mut timer: ResMut<TickTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        writer.send(Tick);
    }
}
