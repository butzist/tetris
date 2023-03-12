use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    GameState, BRICK_COLS_RANGE, BRICK_ROWS, BRICK_ROWS_RANGE, BRICK_SIZE, OFFSET_X, OFFSET_Y,
};

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Brick {
    pub x: i8,
    pub y: i8,
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct Bricks(pub HashMap<(i8, i8), Entity>);

#[derive(Debug, Clone, Deref)]
pub struct LinesRemoved(u8);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LinesRemoved>()
            .init_resource::<Bricks>()
            .register_type::<Brick>()
            .add_systems(
                (remove_lines, move_lines_down)
                    .chain()
                    .in_set(OnUpdate(GameState::InGame))
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .add_system(reset.in_schedule(OnEnter(GameState::InGame)));
    }
}

pub fn to_brick_coordinates(translation: Vec3) -> (i8, i8) {
    let x = ((translation.x - OFFSET_X) / BRICK_SIZE).round() as i8;
    let y = (((translation.y - OFFSET_Y) + BRICK_ROWS as f32 / 2. * BRICK_SIZE) / BRICK_SIZE)
        .round() as i8;
    (x, y)
}

pub fn to_brick_translation(x: i8, y: i8) -> Vec3 {
    Vec3 {
        x: x as f32 * BRICK_SIZE + OFFSET_X,
        y: (y as f32 - (BRICK_ROWS as f32 / 2.)) * BRICK_SIZE + OFFSET_Y,
        z: 1.,
    }
}

pub fn brick_bundle(translation: Vec3, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: Transform {
            translation,
            scale: Vec3 {
                x: BRICK_SIZE * 0.9,
                y: BRICK_SIZE * 0.9,
                z: 1.,
            },
            ..default()
        },
        ..default()
    }
}

pub fn spawn_brick(commands: &mut Commands, bricks: &mut Bricks, brick: Brick, color: Color) {
    let entity = commands
        .spawn(brick_bundle(to_brick_translation(brick.x, brick.y), color))
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

    for y in BRICK_ROWS_RANGE {
        if BRICK_COLS_RANGE
            .clone()
            .all(|x| bricks.contains_key(&(x, y)))
        {
            remove_line(&mut commands, &mut bricks, y);
            removed_lines += 1;
        }
    }

    if removed_lines > 0 {
        events.send(LinesRemoved(removed_lines))
    }
}

pub fn remove_line(commands: &mut Commands, bricks: &mut Bricks, y: i8) {
    for x in BRICK_COLS_RANGE {
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

    let mut from_ys = BRICK_ROWS_RANGE.into_iter();
    for to_y in BRICK_ROWS_RANGE {
        if BRICK_COLS_RANGE
            .clone()
            .all(|x| !bricks.contains_key(&(x, to_y)))
        {
            while let Some(from_y) = from_ys.next() {
                if from_y > to_y
                    && BRICK_COLS_RANGE
                        .clone()
                        .any(|x| bricks.contains_key(&(x, from_y)))
                {
                    for x in BRICK_COLS_RANGE {
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
                            transform.translation = to_brick_translation(brick.x, brick.y);
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

#[cfg(test)]
mod tests {
    use crate::BRICK_ROWS;
    use bevy::prelude::*;

    use super::{to_brick_coordinates, to_brick_translation};

    #[test]
    fn screen_center() {
        let translation = to_brick_translation(0, BRICK_ROWS / 2);

        assert_eq!(
            translation,
            Vec3 {
                x: 0.,
                y: 0.,
                z: 1.
            }
        );
    }

    #[test]
    fn top() {
        let translation = to_brick_translation(0, BRICK_ROWS);

        assert_eq!(
            translation,
            Vec3 {
                x: 0.,
                y: 300.,
                z: 1.
            }
        );
    }

    #[test]
    fn convert_and_back() {
        let translation = to_brick_translation(-3, 7);
        let coords = to_brick_coordinates(translation);

        assert_eq!(coords, (-3, 7));
    }
}
