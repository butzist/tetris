use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};

use crate::{GameState, GameStats, BRICK_COLS, BRICK_ROWS, BRICK_SIZE};

const UI_BG_COLOR: Color = Color::DARK_GRAY;

#[derive(Component, Clone, Debug)]
struct StatusText;

#[derive(Component, Clone, Debug)]
pub struct StatisticsText;

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Baloo2-ExtraBold.ttf")]
    status: Handle<Font>,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .init_collection::<FontAssets>()
            .add_system(hide_status.in_schedule(OnEnter(GameState::InGame)))
            .add_system(show_paused.in_schedule(OnEnter(GameState::Paused)))
            .add_system(show_game_over.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(update_statistics.in_set(OnUpdate(GameState::InGame)));
    }
}

const CENTER_COLUMN_HEIGHT: f32 = BRICK_SIZE * (BRICK_ROWS + 1) as f32;
const CENTER_COLUMN_WIDTH: f32 = BRICK_SIZE * BRICK_COLS as f32 + 10.;
const SIDE_COLUMN_WIDTH: f32 = (800. - CENTER_COLUMN_WIDTH) / 2.;

fn setup(mut commands: Commands, assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(SIDE_COLUMN_WIDTH), Val::Percent(100.0)),
                    ..default()
                },
                background_color: UI_BG_COLOR.into(),
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(CENTER_COLUMN_WIDTH), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Auto),
                            flex_grow: 1.0,
                            ..default()
                        },
                        background_color: UI_BG_COLOR.into(),
                        ..default()
                    });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(CENTER_COLUMN_HEIGHT)),
                                margin: UiRect::all(Val::Px(5.0)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "Loading...",
                                        TextStyle {
                                            font: assets.status.cast_weak(),
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center)
                                    .with_style(Style {
                                        align_self: AlignSelf::Center,
                                        ..Default::default()
                                    }),
                                )
                                .insert(Visibility::Visible)
                                .insert(StatusText);
                        });

                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Auto),
                            flex_grow: 1.0,
                            ..default()
                        },
                        background_color: UI_BG_COLOR.into(),
                        ..default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(SIDE_COLUMN_WIDTH), Val::Percent(100.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    background_color: UI_BG_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: assets.status.cast_weak(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_alignment(TextAlignment::Right),
                        )
                        .insert(StatisticsText);
                });
        });
}

fn hide_status(mut query: Query<&mut Visibility, With<StatusText>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}

fn show_paused(mut query: Query<(&mut Text, &mut Visibility), With<StatusText>>) {
    for (mut text, mut visibility) in &mut query {
        text.sections[0].value = "Game paused\nPress SPACE".into();
        *visibility = Visibility::Visible;
    }
}

fn show_game_over(mut query: Query<(&mut Text, &mut Visibility), With<StatusText>>) {
    for (mut text, mut visibility) in &mut query {
        text.sections[0].value = "Game over\nPress SPACE".into();
        *visibility = Visibility::Visible;
    }
}

fn update_statistics(mut query: Query<(&mut Text, With<StatisticsText>)>, res: Res<GameStats>) {
    for (mut text, _) in &mut query {
        text.sections[0].value = format!(
            "Shapes spawned: {}\n\nLines removed:\n1: {}\n2: {}\n3: {}\n4: {}",
            res.shapes_spawned,
            res.lines_removed.0[0],
            res.lines_removed.0[1],
            res.lines_removed.0[2],
            res.lines_removed.0[3]
        );
    }
}
