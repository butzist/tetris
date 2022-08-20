use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
        .add_startup_system(setup)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //         .with_system(player_movement_system)
        //         .with_system(snap_to_player_system)
        //         .with_system(rotate_to_player_system),
        // )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    let origin = Vec3 {
        x: 0.,
        y: 250.,
        z: 1.,
    };

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: origin,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let mut make_block = |x: i8, y: i8| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32 * 50.,
                            y: y as f32 * 50.,
                            z: 0.,
                        },
                        scale: Vec3 {
                            x: 45.,
                            y: 45.,
                            z: 1.,
                        },
                        ..default()
                    },
                    ..default()
                });
            };

            make_block(0, 0);
            make_block(1, 0);
            make_block(-1, 0);
            make_block(0, -1);
        });
}
