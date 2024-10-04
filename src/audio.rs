//! This example illustrates how to load and play an audio file, and control how it's played.

use bevy::prelude::*;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Event, Debug)]
pub struct PlayMusicEvent(pub bool);

#[derive(Resource, Deref)]
pub struct CollisionSound(Handle<AudioSource>);

#[derive(Component)]
pub struct Music;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Sound
    let collision_audio_source = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(collision_audio_source));

    commands.spawn((
        Music,
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            ..default()
        },
    ));
}

pub fn play_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),

            settings: PlaybackSettings::DESPAWN,
        });
    }
}

pub fn play_music_system(
    mut play_music_events: EventReader<PlayMusicEvent>,
    mut music_controller_query: Query<&mut AudioSink, With<Music>>,
) {
    for event in play_music_events.read() {
        println!("play_music_event {:?}", event);
        let sink = music_controller_query.single_mut();
        if event.0 {
            sink.play();
        } else {
            sink.pause();
        }
    }
}

// example snippets
// fn update_speed(music_controller: Query<&AudioSink, With<Music>>, time: Res<Time>) {
//     if let Ok(sink) = music_controller.get_single() {
//         sink.set_speed(((time.elapsed_seconds() / 5.0).sin() + 1.0).max(0.1));
//     }
// }
//
// fn pause(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     music_controller: Query<&AudioSink, With<Music>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         if let Ok(sink) = music_controller.get_single() {
//             sink.toggle();
//         }
//     }
// }
//
// fn volume(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     music_controller: Query<&AudioSink, With<Music>>,
// ) {
//     if let Ok(sink) = music_controller.get_single() {
//         if keyboard_input.just_pressed(KeyCode::KeyA) {
//             sink.set_volume(sink.volume() + 0.1);
//         } else if keyboard_input.just_pressed(KeyCode::KeyB) {
//             sink.set_volume(sink.volume() - 0.1);
//         }
//     }
// }
