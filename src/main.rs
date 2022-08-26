use bevy::{prelude::*, window::PresentMode};

mod audio;
mod bricks;
mod shape;
mod tick;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    Paused,
    GameOver,
}

#[derive(Component, Clone, Debug)]
struct GameOverText;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 650.,
            width: 850.,
            resizable: false,
            title: "Tetris".into(),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_state(GameState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(bricks::BrickPlugin)
        .add_plugin(shape::ShapePlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(tick::TickPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(restart_game))
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
}

fn restart_game(keys: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        state
            .replace(GameState::InGame)
            .expect("cannot change state");
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
