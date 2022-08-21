use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod shape;
mod tick;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 600.,
            width: 800.,
            resizable: false,
            title: "Tetris".into(),
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
