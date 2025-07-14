//! This example illustrates how to load and play an audio file, and control how it's played.

use bevy::prelude::*;

/// Play a one shot sound sample
#[derive(Event)]
pub enum PlaySoundEvent {
    AlienHit,
}

/// Control continuous playback
#[derive(Event, Debug)]
pub struct PlayMusicEvent(pub bool);

#[derive(Resource, Clone)]
pub struct AudioResource {
    hit_sample: Handle<AudioSource>,
}

#[derive(Component)]
pub struct Music;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Sound
    let hit_sample = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(AudioResource { hit_sample });

    commands.spawn((
        Music,
        AudioPlayer::<AudioSource>(asset_server.load("sounds/Windless Slopes.ogg")),
    ));
}

pub fn audio_hit_system(
    mut commands: Commands,
    mut play_sound_er: EventReader<PlaySoundEvent>,
    sound: Res<AudioResource>,
) {
    for event in play_sound_er.read() {
        let sample = match event {
            PlaySoundEvent::AlienHit => &sound.hit_sample,
        };
        // commands.spawn(AudioBundle {
        //     source: sample.clone(), // this is ugly, why owned?
        //     settings: PlaybackSettings::DESPAWN,
        // });
        commands.spawn(AudioPlayer::<AudioSource>(sample.clone()));
    }
}

pub fn play_music_system(
    mut play_music_events: EventReader<PlayMusicEvent>,
    mut music_controller_query: Query<&mut AudioSink, With<Music>>,
) {
    for event in play_music_events.read() {
        debug!("play_music_event {:?}", event);
        if let Ok(sink) = music_controller_query.single_mut() {
            if event.0 {
                sink.play();
            } else {
                sink.pause();
            }
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
