use bevy::{prelude::*, utils::hashbrown::HashMap};

pub const BRICK_SIZE: f32 = 50.;

#[derive(Component, Default, Clone, Debug)]
pub struct Brick {
    pub x: i8,
    pub y: i8,
}

#[derive(Default, Debug)]
pub struct Bricks(pub HashMap<(i8, i8), Entity>);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bricks::default());
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
