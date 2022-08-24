use bevy::{prelude::*, time::FixedTimestep, utils::hashbrown::HashMap};

pub const BRICK_SIZE: f32 = 50.;

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Brick {
    pub x: i8,
    pub y: i8,
}

#[derive(Default, Debug, Deref, DerefMut)]
pub struct Bricks(pub HashMap<(i8, i8), Entity>);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bricks::default())
            .register_type::<Brick>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 10.))
                    .with_system(remove_lines)
                    .with_system(move_lines_down),
            );
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

pub fn remove_lines(commands: Commands, bricks: ResMut<Bricks>) {
    for y in 0..13 {
        if (-8..=8).all(|x| bricks.contains_key(&(x, y))) {
            remove_line(commands, bricks, y);
            return;
        }
    }
}

pub fn remove_line(mut commands: Commands, mut bricks: ResMut<Bricks>, y: i8) {
    for x in -8..=8 {
        let coords = (x, y);
        let entity = bricks.remove(&coords);

        if let Some(entity) = entity {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_lines_down(mut bricks: ResMut<Bricks>, mut query: Query<(&mut Brick, &mut Transform)>) {
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
