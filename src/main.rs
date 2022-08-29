use bevy::{prelude::*, window::PresentMode};
use bricks::LinesRemoved;
use controls::ControlEvent;
use shape::ShapeSpawned;

mod audio;
mod bricks;
mod controls;
mod shape;
mod tick;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    Paused,
    GameOver,
}

#[derive(Default)]
struct GameStats {
    lines_removed: u32,
    shapes_spawned: u32,
}

#[derive(Component, Clone, Debug)]
struct GameOverText;

#[derive(Component, Clone, Debug)]
struct PausedText;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 650.,
            width: 850.,
            resizable: false,
            title: "Tetris".into(),
            present_mode: PresentMode::Fifo,
            canvas: Some("#bevy".into()),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_state(GameState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(bricks::BrickPlugin)
        .add_plugin(shape::ShapePlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(tick::TickPlugin)
        .init_resource::<GameStats>()
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(pause_game)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_statistics))
        .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(show_paused))
        .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(hide_paused))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(show_game_over))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(hide_game_over))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Game over - press SPACE",
                TextStyle {
                    font: asset_server.load("fonts/Baloo2-ExtraBold.ttf"),
                    font_size: 70.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Visibility { is_visible: false })
        .insert(GameOverText);

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Game paused - press SPACE",
                TextStyle {
                    font: asset_server.load("fonts/Baloo2-ExtraBold.ttf"),
                    font_size: 70.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Visibility { is_visible: false })
        .insert(PausedText);
}

fn pause_game(mut control_events: EventReader<ControlEvent>, mut state: ResMut<State<GameState>>) {
    for &event in control_events.iter() {
        if event != ControlEvent::Pause {
            continue;
        }

        match state.current() {
            GameState::InGame => state.push(GameState::Paused).expect("cannot change state"),
            GameState::Paused => state.pop().expect("cannot change state"),
            GameState::GameOver => state
                .replace(GameState::InGame)
                .expect("cannot change state"),
        }
    }
}

fn show_game_over(mut query: Query<&mut Visibility, With<GameOverText>>) {
    for mut visibility in &mut query {
        visibility.is_visible = true;
    }
}

fn hide_game_over(mut query: Query<&mut Visibility, With<GameOverText>>) {
    for mut visibility in &mut query {
        visibility.is_visible = false;
    }
}

fn show_paused(mut query: Query<&mut Visibility, With<PausedText>>) {
    for mut visibility in &mut query {
        visibility.is_visible = true;
    }
}

fn hide_paused(mut query: Query<&mut Visibility, With<PausedText>>) {
    for mut visibility in &mut query {
        visibility.is_visible = false;
    }
}

fn update_statistics(
    mut stats: ResMut<GameStats>,
    mut shapes: EventReader<ShapeSpawned>,
    mut lines: EventReader<LinesRemoved>,
) {
    for _ in shapes.iter() {
        stats.shapes_spawned += 1;
    }

    for event in lines.iter() {
        stats.lines_removed += **event as u32;
    }
}

fn reset(mut commands: Commands) {
    commands.insert_resource(GameStats::default())
}
