use bevy::math::vec3;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "AH".into(),
                    ..default()
                }),
                ..default()
            }),
            bevy_kira_audio::AudioPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_mark_scream,
                aah_window_title,
                screaming_face,
                sync_mark_scream,
            ),
        )
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

#[derive(Resource)]
struct MarkScream(Handle<AudioInstance>);

#[derive(Component, PartialEq)]
enum State {
    Stop,
    Go,
}

#[derive(Component)]
struct ScreamTimer {
    timer: Timer,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(Camera2dBundle::default());

    let scream_handle = audio
        .play(asset_server.load("sound/mark-screaming.ogg"))
        .looped()
        .with_volume(3.0)
        .handle();

    commands.insert_resource(MarkScream(scream_handle));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/mark.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(vec3(0.25, 0.25, 0.25)),

            ..default()
        },
        State::Stop,
        ScreamTimer {
            timer: Timer::new(Duration::from_secs_f32(5.99), TimerMode::Repeating),
        },
    ));
}

fn sync_mark_scream(
    scream: Res<MarkScream>,
    mut scream_timer: Query<&mut ScreamTimer>,
    time: Res<Time>,
    mut audio_instance: ResMut<Assets<AudioInstance>>,
) {
    let mut timer = scream_timer.get_single_mut().unwrap();

    if timer.timer.tick(time.delta()).just_finished() {
        let instance = audio_instance.get_mut(&scream.0).unwrap();
        instance.seek_to(0.0);
    }
}

fn toggle_mark_scream(
    input: Res<ButtonInput<KeyCode>>,
    sound: Res<MarkScream>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut scream: Query<(&mut State, &mut ScreamTimer)>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        if let Some(instance) = audio_instances.get_mut(&sound.0) {
            instance.pause(AudioTween::default());
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
            scream.timer.tick(time.delta());
            let t = (63.75 + 329.375 * scream.timer.elapsed_secs()) / 255.0;
            transform.scale.x = t;
            transform.scale.y = t;
        }
        if scream.timer.tick(time.delta()).just_finished() {
            transform.scale = Vec3::ZERO;
        }
    }
}
