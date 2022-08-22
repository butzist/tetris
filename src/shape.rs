use bevy::prelude::*;

use crate::tick::{Tick, TickTimer};

const BRICK_SIZE: f32 = 50.;

pub(crate) struct ShapePlugin;

#[derive(Component, Default, Clone, Debug)]
struct Shape {
    next_transform: Transform,
}

#[derive(Component, Clone, Debug)]
struct ShapeBlock;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_shape)
            .add_system(controls.before(fall_shape))
            .add_system(fall_shape.before(move_shape))
            .add_system(move_shape);
    }
}

fn move_shape(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Shape, &Children)>,
    child_query: Query<&Transform, (With<ShapeBlock>, Without<Shape>)>,
) {
    let (entity, mut transform, mut shape, children) = query.single_mut();
    if !children
        .into_iter()
        .map(|child| child_query.get(*child).unwrap())
        .any(|child_transform| collides(&shape.next_transform, child_transform))
    {
        *transform = shape.next_transform;
    } else if shape.next_transform.translation.y < transform.translation.y {
        // if shape could not move down
        commands.entity(entity).despawn_recursive();
        spawn_shape(commands);
    }

    // reset next transform
    shape.next_transform = *transform;
}

fn fall_shape(mut query: Query<(&mut Shape, &Transform)>, mut ticks: EventReader<Tick>) {
    let (mut shape, transform) = query.single_mut();

    if !ticks.is_empty() {
        // reset other queued transforms
        shape.next_transform = *transform;
        for _ in ticks.iter() {
            shape.next_transform.translation.y -= BRICK_SIZE;
        }
    }
}

fn collides(parent_transform: &Transform, child_transform: &Transform) -> bool {
    let position = parent_transform.mul_transform(*child_transform);

    let x = (position.translation.x / BRICK_SIZE) as i8;
    let y = ((position.translation.y + 300.) / BRICK_SIZE) as i8;

    if x > 8 || x < -8 {
        return true;
    }

    if y < 0 {
        return true;
    }

    false
}

fn spawn_shape(mut commands: Commands) {
    let origin = Transform {
        translation: Vec3 {
            x: 0.,
            y: 300.,
            z: 1.,
        },
        ..default()
    };

    commands
        .spawn_bundle(SpatialBundle {
            transform: origin,
            ..default()
        })
        .insert(Shape {
            next_transform: origin,
        })
        .with_children(|parent| {
            let mut make_block = |x: i8, y: i8| {
                parent
                    .spawn_bundle(SpriteBundle {
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
                    })
                    .insert(ShapeBlock);
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
    mut query: Query<&mut Shape>,
) {
    let mut shape = query.single_mut();

    if keys.just_pressed(KeyCode::Down) {
        tick_timer.speedup()
    }
    if keys.just_released(KeyCode::Down) {
        tick_timer.end_speedup()
    }

    if keys.just_pressed(KeyCode::Right) {
        shape.next_transform.translation.x += BRICK_SIZE;
    }

    if keys.just_pressed(KeyCode::Left) {
        shape.next_transform.translation.x -= BRICK_SIZE;
    }

    if keys.just_pressed(KeyCode::Up) {
        shape
            .next_transform
            .rotate(Quat::from_rotation_z(-std::f32::consts::PI / 2.0));
    }
}
