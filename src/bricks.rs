use bevy::{prelude::*, time::FixedTimestep, utils::hashbrown::HashMap};

use crate::GameState;

pub const BRICK_SIZE: f32 = 50.;

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Brick {
    pub x: i8,
    pub y: i8,
}

#[derive(Default, Debug, Deref, DerefMut)]
pub struct Bricks(pub HashMap<(i8, i8), Entity>);

#[derive(Debug, Clone, Deref)]
pub struct LinesRemoved(u8);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LinesRemoved>()
            .init_resource::<Bricks>()
            .register_type::<Brick>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_run_criteria(FixedTimestep::step(1. / 10.))
                    .with_system(remove_lines)
                    .with_system(move_lines_down),
            )
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset));
    }
}

pub fn spawn_brick(commands: &mut Commands, bricks: &mut Bricks, brick: Brick, color: Color) {
    let entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation: Vec3 {
                    x: brick.x as f32 * BRICK_SIZE,
                    y: brick.y as f32 * BRICK_SIZE - 300.,
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
        .insert(brick.clone())
        .id();

    bricks.0.insert((brick.x, brick.y), entity);
}

pub fn remove_lines(
    mut commands: Commands,
    mut bricks: ResMut<Bricks>,
    mut events: EventWriter<LinesRemoved>,
) {
    if !bricks.is_changed() {
        return;
    }

    let mut removed_lines = 0;

    for y in 0..13 {
        if (-8..=8).all(|x| bricks.contains_key(&(x, y))) {
            remove_line(&mut commands, &mut bricks, y);
            removed_lines += 1;
        }
    }

    if removed_lines > 0 {
        events.send(LinesRemoved(removed_lines))
    }
}

pub fn remove_line(commands: &mut Commands, bricks: &mut Bricks, y: i8) {
    for x in -8..=8 {
        let coords = (x, y);
        let entity = bricks.remove(&coords);

        if let Some(entity) = entity {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_lines_down(
    mut bricks: ResMut<Bricks>,
    mut done: Local<bool>,
    mut query: Query<(&mut Brick, &mut Transform)>,
) {
    if *done && !bricks.is_changed() {
        return;
    } else {
        // do not run again unless there's a change
        *done = true;
    }

    let mut from_ys = (0..13).into_iter();
    for to_y in 0..13 {
        if (-8..=8).all(|x| !bricks.contains_key(&(x, to_y))) {
            while let Some(from_y) = from_ys.next() {
                if from_y > to_y && (-8..=8).any(|x| bricks.contains_key(&(x, from_y))) {
                    for x in -8..=8 {
                        let coords = (x, from_y);
                        let entity = bricks.remove(&coords);

                        if let Some(entity) = entity {
                            let (mut brick, mut transform) =
                                query.get_mut(entity).expect("no such entity");
                            let new_coords = (x, to_y);

                            // run again
                            *done = false;

                            bricks.insert(new_coords, entity);
                            brick.y = to_y;
                            transform.translation.y = brick.y as f32 * BRICK_SIZE - 300.;
                        }
                    }
                    break; // proceed to next empty line
                }
            }
            return; // nothing left to move
        }
    }
}

pub fn reset(mut commands: Commands, mut query: Query<Entity, With<Brick>>) {
    commands.insert_resource(Bricks::default());

    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}
