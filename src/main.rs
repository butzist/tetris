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
mod ui;

const BRICK_SIZE: f32 = 30.;
const OFFSET_X: f32 = 0.;
const OFFSET_Y: f32 = 0.;
const BRICK_ROWS: i8 = 20;
const BRICK_ROWS_RANGE: std::ops::Range<i8> = 0..BRICK_ROWS;
const BRICK_COLS: i8 = 11;
const BRICK_COLS_RANGE: std::ops::RangeInclusive<i8> = {
    let half = (BRICK_COLS - 1) / 2;
    -half..=half
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetLoading,
    InGame,
    Paused,
    GameOver,
}

#[derive(Resource, Default)]
struct GameStats {
    lines_removed: LineStats,
    shapes_spawned: usize,
}

#[derive(Default)]
struct LineStats([usize; 4]);

impl LineStats {
    fn add(&mut self, lines: usize) {
        self.0[lines - 1] += 1;
    }
}

fn main() {
    App::new()
        .add_state(GameState::AssetLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .with_collection::<SoundAssets>(),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                height: 800.,
                width: 1000.,
                resizable: false,
                title: "Tetris".into(),
                present_mode: PresentMode::Fifo,
                canvas: Some("#bevy".into()),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ui::UiPlugin)
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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

fn update_statistics(
    mut stats: ResMut<GameStats>,
    mut shapes: EventReader<ShapeSpawned>,
    mut lines: EventReader<LinesRemoved>,
) {
    for _ in shapes.iter() {
        stats.shapes_spawned += 1;
    }

    for event in lines.iter() {
        stats.lines_removed.add(**event as usize);
    }
}

fn reset(mut commands: Commands) {
    commands.insert_resource(GameStats::default())
}
