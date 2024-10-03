//! Space Invaders revisited, why not?
//! RUST_LOG="bevy-space=info" cargo run

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_space::{
    alien, bunker, common::*, game_state, hit_detection, keyboard_input, lazer, overlay, particle,
    player,
};

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
        .insert_resource(ClearColor(Color::BLACK))
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
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                keyboard_input::update_system,
                hit_detection::update_system,
                player::update_system,
                player::blink_update_system,
                lazer::update_system,
                alien::alien_movement_system,
                alien::alien_bullet_movement,
                alien::animate_alien_system,
                overlay::text_update_system,
                overlay::score_update_system,
                overlay::state_update_system,
                game_state::state_transition_system,
                particle::particle_update_system,
            ),
        )
        .run();
}
