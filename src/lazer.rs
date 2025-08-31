use crate::{common::*, particle::*, player::Player, game_state::Store};
use bevy::prelude::*;
use rand::random;
use std::time::Duration;

#[derive(Component, PartialEq, Clone)]
pub enum Lazer {
    Fire,
    Fired(Timer),
    Idle,
}

pub struct LazerData {
    pub lazer: Lazer,
    pub transform: Transform
}

#[derive(Event)]
pub struct FireLazerEvent;

pub fn fire_lazer_system(
    mut fire_lazer_event: EventReader<FireLazerEvent>,
    time: Res<Time>,
    mut store: ResMut<Store>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !fire_lazer_event.is_empty() {
        debug!("-- fire lazer event received --");
        fire_lazer_event.clear();
        if store.lazer_interval <= 0.0 {
            if let Ok(player_transform) = player_query.get_single() {
                store.lazers.push(LazerData {
                    lazer: Lazer::Fire,
                    transform: Transform::from_translation(player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0)),
                });

                store.lazer_interval = LAZER_FIRING_INTERVAL;
            }
        }
    }

    store.lazer_interval -= time.delta_seconds();
}


/// lazer movement
pub fn update_system(
    mut commands: Commands,
    time: Res<Time>,
    image: Res<CrossImage>,
    player_query: Query<&Transform, With<Player>>,
    mut store: ResMut<Store>,
) {
    let player_transform = player_query.single();

    store.lazers.retain_mut(|lazer_data| {
        match &mut lazer_data.lazer {
            Lazer::Fire => {
                lazer_data.lazer = Lazer::Fired(Timer::new(
                    Duration::from_secs_f32(LAZER_PARTICLE_INTERVAL),
                    TimerMode::Repeating,
                ));
                lazer_data.transform.translation = player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);

                spawn_explosion(
                    &mut commands,
                    &image,
                    50,
                    lazer_data.transform.translation.truncate(),
                    100.0,
                    0.0,
                    (10.0, 10.0).into(),
                );
            }
            Lazer::Fired(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    spawn_particle(
                        &mut commands,
                        &image,
                        lazer_data.transform.translation.truncate(),
                        (30.0 * (random::<f32>() - 0.5), -LAZER_SPEED * 0.1).into(),
                        (0.0, 0.0).into(),
                    );
                }

                lazer_data.transform.translation.y += LAZER_SPEED * time.delta_seconds();

                if lazer_data.transform.translation.y > SCENE_HEIGHT {
                    return false;
                }
            }
            Lazer::Idle => {
                return false;
            }
        }
        true
    });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
 // Nothing to do here
}
