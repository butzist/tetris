use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    bricks::{spawn_brick, Brick, Bricks, BRICK_SIZE},
    tick::{Tick, TickTimer},
    GameState,
};

#[derive(Component, Default, Clone, Debug)]
struct Shape {
    next_transform: Transform,
}

#[derive(Component, Clone, Debug)]
struct ShapeBrick;

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(controls.before(fall_shape))
                    .with_system(fall_shape.before(move_shape))
                    .with_system(check_game_over.before(move_shape))
                    .with_system(move_shape),
            );
    }
}

fn move_shape(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Shape, &Children)>,
    child_query: Query<(&Transform, &Sprite), (With<ShapeBrick>, Without<Shape>)>,
    mut bricks: ResMut<Bricks>,
) {
    let commands = &mut commands;
    let bricks = &mut *bricks;

    for (entity, mut transform, mut shape, children) in &mut query {
        let children: Vec<_> = children
            .into_iter()
            .map(|child| child_query.get(*child).unwrap())
            .collect();

        if !children
            .iter()
            .any(|(child_transform, _)| collides(&shape.next_transform, child_transform, bricks))
        {
            *transform = shape.next_transform;
        } else if shape.next_transform.translation.y < transform.translation.y {
            // if shape could not move down
            shape_to_bricks(commands, bricks, &*transform, &children);
            commands.entity(entity).despawn_recursive();
            spawn_shape(commands);
        }

        // reset next transform
        shape.next_transform = *transform;
    }
}

fn check_game_over(
    mut state: ResMut<State<GameState>>,
    query: Query<(&Transform, &Children), With<Shape>>,
    child_query: Query<(&Transform, &Sprite), (With<ShapeBrick>, Without<Shape>)>,
    bricks: Res<Bricks>,
) {
    let bricks = &*bricks;

    for (transform, children) in &query {
        let children: Vec<_> = children
            .into_iter()
            .map(|child| child_query.get(*child).unwrap())
            .collect();

        if children
            .iter()
            .any(|(child_transform, _)| collides(transform, child_transform, bricks))
        {
            println!("game over");
            state
                .replace(GameState::GameOver)
                .expect("cannot change state");
        }
    }
}

fn shape_to_bricks(
    commands: &mut Commands,
    bricks: &mut Bricks,
    transform: &Transform,
    children: &[(&Transform, &Sprite)],
) {
    for (&child, sprite) in children {
        let child_position = transform.mul_transform(child);
        let coords = to_brick_coordinates(child_position);
        spawn_brick(
            commands,
            &mut *bricks,
            Brick {
                x: coords.0,
                y: coords.1,
            },
            sprite.color,
        );
    }
}

fn fall_shape(mut query: Query<(&mut Shape, &Transform)>, mut ticks: EventReader<Tick>) {
    for (mut shape, transform) in &mut query {
        if !ticks.is_empty() {
            // reset other queued transforms
            shape.next_transform = *transform;
            for _ in ticks.iter() {
                shape.next_transform.translation.y -= BRICK_SIZE;
            }
        }
    }
}

fn collides(parent_transform: &Transform, child_transform: &Transform, bricks: &Bricks) -> bool {
    let position = parent_transform.mul_transform(*child_transform);

    let (x, y) = to_brick_coordinates(position);

    if x > 8 || x < -8 {
        return true;
    }

    if y < 0 {
        return true;
    }

    if bricks.contains_key(&(x, y)) {
        return true;
    }

    false
}

fn to_brick_coordinates(position: Transform) -> (i8, i8) {
    let x = (position.translation.x / BRICK_SIZE).round() as i8;
    let y = ((position.translation.y + 300.) / BRICK_SIZE).round() as i8;
    (x, y)
}

fn spawn_shape(commands: &mut Commands) {
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
            let color = Color::hsl(thread_rng().gen_range(0.0..360.0), 1.0, 0.6);
            let make_brick = |x: i8, y: i8| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite { color, ..default() },
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
                    .insert(ShapeBrick);
            };

            make_random_shape(make_brick);
        });
}

fn make_random_shape(mut make_brick: impl FnMut(i8, i8)) {
    let choice = thread_rng().gen_range(0..=6);

    match choice {
        0 => {
            // T
            make_brick(0, 0);
            make_brick(1, 0);
            make_brick(-1, 0);
            make_brick(0, -1);
        }
        1 => {
            // I
            make_brick(-1, 0);
            make_brick(0, 0);
            make_brick(1, 0);
            make_brick(2, 0);
        }
        2 => {
            // L
            make_brick(-2, 0);
            make_brick(-1, 0);
            make_brick(0, 0);
            make_brick(0, -1);
        }
        3 => {
            // L'
            make_brick(2, 0);
            make_brick(1, 0);
            make_brick(0, 0);
            make_brick(0, -1);
        }
        4 => {
            // S
            make_brick(0, 0);
            make_brick(1, 0);
            make_brick(-1, -1);
            make_brick(0, -1);
        }
        5 => {
            // Z
            make_brick(0, 0);
            make_brick(-1, 0);
            make_brick(1, -1);
            make_brick(0, -1);
        }
        6 => {
            // O
            make_brick(0, 0);
            make_brick(1, 0);
            make_brick(0, -1);
            make_brick(1, -1);
        }
        _ => unreachable!(),
    }
}

fn controls(
    keys: Res<Input<KeyCode>>,
    mut tick_timer: ResMut<TickTimer>,
    mut query: Query<&mut Shape>,
) {
    if keys.just_pressed(KeyCode::Down) {
        tick_timer.speedup()
    }
    if keys.just_released(KeyCode::Down) {
        tick_timer.end_speedup()
    }

    for mut shape in &mut query {
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
}

fn reset(mut commands: Commands, query: Query<Entity, With<Shape>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    spawn_shape(&mut commands);
}
