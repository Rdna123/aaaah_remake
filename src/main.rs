use bevy::audio::{PlaybackMode, Volume};
use bevy::math::vec3;
use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "AH".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (make_mark_scream, aah_window_title, screaming_face))
        .run();
}

fn aah_window_title(input: Res<ButtonInput<KeyCode>>, mut window: Query<&mut Window>) {
    if input.pressed(KeyCode::KeyA) {
        let mut window = window.single_mut();
        let count = window.title.len();
        let mut title = "".to_string();

        for _ in 0..count {
            title += "A";
        }

        title += "H";

        window.title = title;
    }
}

#[derive(Component)]
struct MarkScream;

#[derive(Component, PartialEq)]
enum State {
    Stop,
    Go,
}

#[derive(Component)]
struct ScreamTimer {
    timer: Timer,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        AudioBundle {
            source: asset_server.load("sound/markie-screaming.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(3.0),
                paused: true,
                ..default()
            },
        },
        MarkScream,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/mark.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(vec3(0.5, 0.5, 0.5)),

            ..default()
        },
        State::Stop,
        ScreamTimer {
            timer: Timer::new(Duration::from_secs_f32(6.59), TimerMode::Repeating),
        },
    ));
}

fn make_mark_scream(
    input: Res<ButtonInput<KeyCode>>,
    sound_controller: Query<&AudioSink, With<MarkScream>>,
    mut scream: Query<(&mut State, &mut ScreamTimer)>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        if let Ok(sink) = sound_controller.get_single() {
            sink.toggle();
        }
        if let Ok((mut s, mut t)) = scream.get_single_mut() {
            match *s {
                State::Stop => {
                    *s = State::Go;
                    t.timer.unpause()
                }
                State::Go => {
                    *s = State::Stop;
                    t.timer.pause()
                }
            }
        }
    }
}

fn screaming_face(mut image: Query<(&mut Transform, &State, &mut ScreamTimer)>, time: Res<Time>) {
    for (mut transform, state, mut scream) in &mut image {
        debug!("aaah");
        if state == &State::Go {
            transform.scale.x += 0.5;
            transform.scale.y += 0.5;
        }
        if scream.timer.tick(time.delta()).just_finished() {
            transform.scale = Vec3::ZERO;
        }
    }
}
