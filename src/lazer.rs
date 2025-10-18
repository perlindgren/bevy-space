use crate::{common::*, particle::*, player::Player};
use bevy::prelude::*;
use rand::random;
use std::time::Duration;

#[derive(Component, PartialEq, Clone, Debug)]
pub enum Lazer {
    Fire,
    Fired(Timer),
    Idle,
}

#[derive(Message)]
pub struct FireLazerMessage;

pub fn fire_lazer_system(
    mut fire_lazer_event: MessageReader<FireLazerMessage>,
    mut lazer_query: Query<&mut Lazer>,
) {
    if !fire_lazer_event.is_empty() {
        info!("-- fire lazer event received --");
        fire_lazer_event.clear();
        if let Ok(mut lazer) = lazer_query.single_mut() {
            if *lazer == Lazer::Idle {
                *lazer = Lazer::Fire;
            }
            info!("lazer {:?}", lazer);
        }
    }
}

/// lazer movement
pub fn update_system(
    mut commands: Commands,
    time: Res<Time>,
    image: Res<CrossImage>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut lazer_position: Query<(&mut Lazer, &mut Visibility, &mut Transform), Without<Player>>,
) {
    if let (Ok(player_transform), Ok((mut lazer, mut visibility, mut transform))) =
        (player_query.single_mut(), lazer_position.single_mut())
    {
        match &mut *lazer {
            Lazer::Fire => {
                info!("you are fired");
                transform.translation =
                    player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);
                *lazer = Lazer::Fired(Timer::new(
                    Duration::from_secs_f32(LAZER_PARTICLE_INTERVAL),
                    TimerMode::Repeating,
                ));
                *visibility = Visibility::Visible;
                spawn_explosion(
                    &mut commands,
                    &image,
                    50,
                    (
                        player_transform.translation.x,
                        player_transform.translation.y,
                    )
                        .into(),
                    100.0,
                    0.0,
                    (10.0, 10.0).into(),
                );
            }
            Lazer::Fired(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    spawn_particle(
                        commands,
                        image,
                        (transform.translation.x, transform.translation.y).into(),
                        (30.0 * (random::<f32>() - 0.5), -LAZER_SPEED * 0.1).into(),
                        (0.0, 0.0).into(),
                    );
                }

                if transform.translation.y > SCENE_HEIGHT {
                    *lazer = Lazer::Idle;
                } else {
                    transform.translation.y += LAZER_SPEED * time.delta_secs()
                }
            }
            _ => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Lazer::Idle,
        Sprite::from_image(asset_server.load("sprites/lazer.png")),
        Transform::from_xyz(0., SCENE_HEIGHT, 0.),
        Visibility::Hidden,
    ));
}
