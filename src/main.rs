use bevy::audio::PlaybackMode;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "AH".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (make_mark_scream,aah))
        .run();
}

fn aah(input: Res<Input<KeyCode>>,mut window: Query<&mut Window>){
    if input.pressed(KeyCode::A){
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
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sound/markie-screaming.ogg"),
            settings: PlaybackSettings{
                mode: PlaybackMode::Loop,
                ..default()
            }
        },
        MarkScream,
    ));
}

fn make_mark_scream(input: Res<Input<KeyCode>>,sound_controller: Query<&AudioSink, With<MarkScream>>){
    if input.just_pressed(KeyCode::S){
        if let Ok(sink) = sound_controller.get_single(){
            sink.toggle();
        }
    }
}