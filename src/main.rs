use audio::SoundAssets;
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, window::PresentMode};
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Starting,
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
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Starting),
        )
        .add_collection_to_loading_state::<_, SoundAssets>(GameState::AssetLoading)
        .insert_resource(Msaa::Sample2)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 1000.).into(),
                resizable: false,
                title: "Tetris".into(),
                present_mode: PresentMode::Fifo,
                canvas: Some("#bevy".into()),
                ..Default::default()
            }),
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
        .add_system(pause_resume_game)
        .add_system(reset.in_schedule(OnEnter(GameState::Starting)))
        .add_system(update_statistics.in_set(OnUpdate(GameState::InGame)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.25,
            ..BloomSettings::NATURAL
        },
    ));
}

fn pause_resume_game(
    mut control_events: EventReader<ControlEvent>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for &event in control_events.iter() {
        if event != ControlEvent::Pause {
            continue;
        }

        match current_state.0 {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            GameState::GameOver => next_state.set(GameState::Starting),
            GameState::AssetLoading | GameState::Starting => (),
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

fn reset(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(GameStats::default());
    next_state.set(GameState::InGame);
}
