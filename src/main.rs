//! Space Invaders revisited, why not?
//! RUST_LOG="bevy-space=info" cargo run

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_space::{
    alien, audio, bunker, common::*, game_state, hit_detection, lazer, overlay, particle, player,
};
use leafwing_input_manager::prelude::*;

fn setup(mut commands: Commands) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RES_X, RES_Y),
                resizable: false,
                title: "Bevy-Space".to_string(),
                desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(InputManagerPlugin::<player::PlayerKeyboardAction>::default())
        .add_plugins(InputManagerPlugin::<player::PlayerGamePadAction>::default())
        .add_plugins(InputManagerPlugin::<game_state::GameStateAction>::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_event::<audio::PlaySoundEvent>()
        .add_event::<audio::PlayMusicEvent>()
        .add_event::<lazer::FireLazerEvent>()
        .add_event::<game_state::GameStateEvent>()
        // .add_event::<player::PlayerEvent>()
        .add_systems(
            Startup,
            (
                setup,
                game_state::setup,
                player::setup,
                lazer::setup,
                alien::setup,
                bunker::setup,
                overlay::setup,
                particle::setup,
                audio::setup,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    hit_detection::update_system,
                    player::keyboard_update_system,
                    player::gamepad_update_system,
                    player::blink_update_system,
                    lazer::update_system,
                    alien::update_system,
                    alien::bullet_update_system,
                    alien::animate_update_system,
                    overlay::text_update_system,
                    overlay::score_update_system,
                    overlay::state_update_system,
                    game_state::update_system,
                    game_state::action_update_system,
                    particle::update_system,
                )
                    .before(audio::audio_hit_system),
                (
                    audio::audio_hit_system,
                    audio::play_music_system,
                    lazer::fire_lazer_system.run_if(game_state::is_playing),
                    game_state::game_state_event_system,
                ),
            ),
        )
        .run();
}
