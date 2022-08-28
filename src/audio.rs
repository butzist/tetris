use bevy::{audio::*, prelude::*};

use crate::{
    bricks::LinesRemoved, controls::ControlEvent, shape::ShapeSpawned, GameState, GameStats,
};

pub struct AudioPlugin;

#[derive(Deref, DerefMut)]
struct MusicInstanceHandle(Handle<AudioSink>);

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(play_music))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(stop_music))
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(pause_music))
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(unpause_music))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(update_playback_speed)
                    .with_system(sound_effects),
            );
    }
}

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let weak_handle = audio.play_with_settings(
        asset_server.load("sounds/Crowander - Gypsy.ogg"),
        PlaybackSettings::LOOP.with_volume(0.5).with_speed(0.9),
    );

    let strong_handle = audio_sinks.get_handle(weak_handle);
    commands.insert_resource(MusicInstanceHandle(strong_handle));
}

fn game_over(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/gameover.ogg"));
}

fn sound_effects(
    asset_server: Res<AssetServer>,
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
            asset_server.load("sounds/rotate.ogg"),
            PlaybackSettings::ONCE.with_volume(0.2),
        );
    }

    if shapes.iter().last().is_some() {
        audio.play_with_settings(
            asset_server.load("sounds/drop.ogg"),
            PlaybackSettings::ONCE.with_volume(0.4),
        );
    }

    if lines.iter().last().is_some() {
        audio.play(asset_server.load("sounds/lines.ogg"));
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
