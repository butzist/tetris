use bevy::prelude::*;
use ignore_result::Ignore;
use rand::{thread_rng, Rng};

use crate::{
    bricks::{
        brick_bundle, spawn_brick, to_brick_coordinates, to_brick_translation, Brick, Bricks,
    },
    controls::ControlEvent,
    tick::Tick,
    GameState, BRICK_COLS_RANGE, BRICK_ROWS, BRICK_SIZE,
};

#[derive(Component, Default, Clone, Debug)]
struct Shape;

#[derive(Component, Clone, Debug)]
struct ShapeBrick;

#[derive(Debug, Clone, Default)]
pub struct ShapeSpawned;

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShapeSpawned>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(check_game_over.before(move_shape))
                    .with_system(move_shape),
            );
    }
}

fn check_game_over(
    mut state: ResMut<State<GameState>>,
    query: Query<(&Transform, &Children), (With<Shape>, Changed<Transform>)>,
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

fn move_shape(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Children), With<Shape>>,
    child_query: Query<(&Transform, &Sprite), (With<ShapeBrick>, Without<Shape>)>,
    mut control_events: EventReader<ControlEvent>,
    mut tick_events: EventReader<Tick>,
    mut spawn_events: EventWriter<ShapeSpawned>,
    mut bricks: ResMut<Bricks>,
) {
    let commands = &mut commands;

    if let Ok((entity, mut transform, children)) = query.get_single_mut() {
        let children: Vec<_> = children
            .into_iter()
            .map(|child| child_query.get(*child).unwrap())
            .collect();

        for next_transform in control_events
            .iter()
            .filter_map(transform_from_control_event)
        {
            try_move_shape(next_transform, &mut transform, &children, &mut bricks).ignore();
        }

        for next_transform in tick_events.iter().map(|_| Transform {
            translation: Vec3::Y * -BRICK_SIZE,
            ..default()
        }) {
            let result = try_move_shape(next_transform, &mut transform, &children, &mut bricks);

            if result.is_err() {
                // if shape could not move down
                shape_to_bricks(commands, &mut bricks, &*transform, &children);
                commands.entity(entity).despawn_recursive();
                spawn_shape(commands);
                spawn_events.send_default();
                return;
            }
        }
    }
}

fn try_move_shape(
    next_transform: Transform,
    transform: &mut Mut<Transform>,
    children: &Vec<(&Transform, &Sprite)>,
    bricks: &mut ResMut<Bricks>,
) -> Result<(), ()> {
    let next_transform = Transform {
        translation: next_transform.translation + transform.translation,
        rotation: next_transform.rotation * transform.rotation,
        ..default()
    };
    if !children
        .iter()
        .any(|(child_transform, _)| collides(&next_transform, child_transform, bricks))
    {
        **transform = next_transform;
        Ok(())
    } else {
        Err(())
    }
}

fn transform_from_control_event(event: &ControlEvent) -> Option<Transform> {
    match event {
        ControlEvent::SpeedupStart | ControlEvent::SpeedupEnd | ControlEvent::Pause => None,
        ControlEvent::Left => Some(Transform {
            translation: Vec3::X * -BRICK_SIZE,
            ..default()
        }),
        ControlEvent::Right => Some(Transform {
            translation: Vec3::X * BRICK_SIZE,
            ..default()
        }),
        ControlEvent::RotateRight => Some(Transform {
            rotation: Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
            ..default()
        }),
        ControlEvent::RotateLeft => Some(Transform {
            rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            ..default()
        }),
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
        let coords = to_brick_coordinates(child_position.translation);
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

fn collides(parent_transform: &Transform, child_transform: &Transform, bricks: &Bricks) -> bool {
    let position = parent_transform.mul_transform(*child_transform);
    let (x, y) = to_brick_coordinates(position.translation.clone());

    if !BRICK_COLS_RANGE.contains(&x) {
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

fn spawn_shape(commands: &mut Commands) {
    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: to_brick_translation(0, BRICK_ROWS),
                ..default()
            },
            ..default()
        })
        .insert(Shape {})
        .with_children(|parent| {
            let color = Color::hsl(thread_rng().gen_range(0.0..360.0), 1.0, 0.6);
            let make_brick = |x: i8, y: i8| {
                parent
                    .spawn(brick_bundle(
                        Vec3::new(x as f32 * BRICK_SIZE, y as f32 * BRICK_SIZE, 1.),
                        color,
                    ))
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

fn reset(mut commands: Commands, query: Query<Entity, With<Shape>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    spawn_shape(&mut commands);
}
