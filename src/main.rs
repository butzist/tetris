use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

mod shape;
mod tick;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 650.,
            width: 850.,
            resizable: false,
            title: "Tetris".into(),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(shape::ShapePlugin)
        .add_plugin(tick::TickPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
