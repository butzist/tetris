use bevy::{audio::*, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{
    bricks::LinesRemoved, controls::ControlEvent, shape::ShapeSpawned, GameState, GameStats,
};

pub struct AudioPlugin;

#[derive(Resource, Deref, DerefMut)]
struct MusicInstanceHandle(Handle<AudioSink>);

#[derive(Resource, AssetCollection)]
pub struct SoundAssets {
    #[asset(path = "sounds/Crowander - Gypsy.ogg")]
    music: Handle<AudioSource>,
    #[asset(path = "sounds/gameover.ogg")]
    gameover: Handle<AudioSource>,
    #[asset(path = "sounds/rotate.ogg")]
    rotate: Handle<AudioSource>,
    #[asset(path = "sounds/drop.ogg")]
    drop: Handle<AudioSource>,
    #[asset(path = "sounds/lines.ogg")]
    lines: Handle<AudioSource>,
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_music.in_schedule(OnExit(GameState::Starting)))
            .add_system(pause_music.in_schedule(OnEnter(GameState::Paused)))
            .add_system(unpause_music.in_schedule(OnExit(GameState::Paused)))
            .add_systems(
                (stop_music, game_over)
                    .chain()
                    .in_schedule(OnEnter(GameState::GameOver)),
            )
            .add_systems(
                (update_playback_speed, sound_effects).in_set(OnUpdate(GameState::InGame)),
            );
    }
}

fn play_music(
    mut commands: Commands,
    assets: Res<SoundAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let weak_handle = audio.play_with_settings(
        assets.music.cast_weak(),
        PlaybackSettings::LOOP.with_volume(0.5).with_speed(0.9),
    );

    let strong_handle = audio_sinks.get_handle(weak_handle);
    commands.insert_resource(MusicInstanceHandle(strong_handle));
}

fn game_over(assets: Res<SoundAssets>, audio: Res<Audio>) {
    audio.play(assets.gameover.cast_weak());
}

fn sound_effects(
    assets: Res<SoundAssets>,
    audio: Res<Audio>,
    mut controls: EventReader<ControlEvent>,
    mut shapes: EventReader<ShapeSpawned>,
    mut lines: EventReader<LinesRemoved>,
) {
    if controls.iter().any(|c| {
        [
            ControlEvent::Left,
            ControlEvent::Right,
            ControlEvent::RotateLeft,
            ControlEvent::RotateRight,
        ]
        .contains(c)
    }) {
        audio.play_with_settings(
            assets.rotate.cast_weak(),
            PlaybackSettings::ONCE.with_volume(0.2),
        );
    }

    if shapes.iter().last().is_some() {
        audio.play_with_settings(
            assets.drop.cast_weak(),
            PlaybackSettings::ONCE.with_volume(0.4),
        );
    }

    if lines.iter().last().is_some() {
        audio.play(assets.lines.cast_weak());
    }
}

fn stop_music(handle: Res<MusicInstanceHandle>, mut audio_sinks: ResMut<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get_mut(&handle) {
        sink.stop();
    }
}

fn pause_music(handle: Res<MusicInstanceHandle>, mut audio_sinks: ResMut<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get_mut(&handle) {
        sink.pause();
    }
}

fn unpause_music(handle: Res<MusicInstanceHandle>, mut audio_sinks: ResMut<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get_mut(&handle) {
        sink.play();
    }
}

fn update_playback_speed(
    handle: Res<MusicInstanceHandle>,
    mut audio_sinks: ResMut<Assets<AudioSink>>,
    stats: Res<GameStats>,
) {
    if stats.is_changed() {
        let speed = 0.9 + stats.shapes_spawned as f32 * 0.005;

        if let Some(sink) = audio_sinks.get_mut(&handle) {
            sink.set_speed(speed);
        }
    }
}
