use audio::SoundAssets;
use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
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
    AssetLoading,
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
struct StatusText;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Baloo2-ExtraBold.ttf")]
    status: Handle<Font>,
}

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
        .add_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .with_collection::<SoundAssets>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(bricks::BrickPlugin)
        .add_plugin(shape::ShapePlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(tick::TickPlugin)
        .init_resource::<GameStats>()
        .init_collection::<FontAssets>()
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(pause_game)
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(reset)
                .with_system(hide_status),
        )
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_statistics))
        .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(show_paused))
        .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(hide_status))
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(show_game_over))
        .run();
}

fn setup(mut commands: Commands, assets: Res<FontAssets>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let text_style = TextStyle {
        font: assets.status.as_weak(),
        font_size: 70.0,
        color: Color::WHITE,
    };

    let text_transform = Transform {
        translation: Vec3::new(0., 0., 100.),
        ..default()
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("Loading...", text_style.clone())
                .with_alignment(TextAlignment::CENTER),
            transform: text_transform,
            ..default()
        })
        .insert(Visibility { is_visible: true })
        .insert(StatusText);
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
            GameState::AssetLoading => (),
        }
    }
}

fn hide_status(mut query: Query<&mut Visibility, With<StatusText>>) {
    for mut visibility in &mut query {
        visibility.is_visible = false;
    }
}

fn show_paused(mut query: Query<(&mut Text, &mut Visibility), With<StatusText>>) {
    for (mut text, mut visibility) in &mut query {
        text.sections[0].value = "Game paused - press SPACE".into();
        visibility.is_visible = true;
    }
}

fn show_game_over(mut query: Query<(&mut Text, &mut Visibility), With<StatusText>>) {
    for (mut text, mut visibility) in &mut query {
        text.sections[0].value = "Game over - press SPACE".into();
        visibility.is_visible = true;
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
