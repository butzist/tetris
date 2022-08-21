use bevy::prelude::*;

use crate::tick::{Tick, TickTimer};

const BRICK_SIZE: f32 = 50.;

pub(crate) struct ShapePlugin;

#[derive(Component)]
struct Shape;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_shape)
            .add_system(shape_falling)
            .add_system(controls);
    }
}

fn shape_falling(mut query: Query<&mut Transform, With<Shape>>, mut ticks: EventReader<Tick>) {
    let mut transform = query.single_mut();

    for _ in ticks.iter() {
        transform.translation.y -= BRICK_SIZE;
    }
}

fn spawn_shape(mut commands: Commands) {
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
        .insert(Shape)
        .with_children(|parent| {
            let mut make_block = |x: i8, y: i8| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32 * BRICK_SIZE,
                            y: y as f32 * BRICK_SIZE,
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

fn controls(
    keys: Res<Input<KeyCode>>,
    mut tick_timer: ResMut<TickTimer>,
    mut query: Query<&mut Transform, With<Shape>>,
) {
    let mut transform = query.single_mut();

    if keys.just_pressed(KeyCode::Down) {
        tick_timer.speedup()
    }
    if keys.just_released(KeyCode::Down) {
        tick_timer.end_speedup()
    }

    if keys.just_pressed(KeyCode::Right) {
        transform.translation.x += BRICK_SIZE;
    }

    if keys.just_pressed(KeyCode::Left) {
        transform.translation.x -= BRICK_SIZE;
    }

    if keys.just_pressed(KeyCode::Up) {
        transform.rotate(Quat::from_rotation_z(-std::f32::consts::PI / 2.0));
    }
}
