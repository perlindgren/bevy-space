//! Renders a 2D scene containing a single, moving sprite.
//! RUST_LOG="game=info" cargo run --example game

use bevy::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
const LAZER_SPEED: f32 = 500.0;
const SCENE_WIDTH: f32 = 400.0;
const SCENE_HEIGHT: f32 = 300.0;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    None,
}

#[derive(Component)]
struct Player;

#[derive(Component, PartialEq, Clone, Copy)]
enum Lazer {
    Fire,
    Fired,
    Idle,
}

/// keyboard input
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_match: Query<&mut Direction>,
    mut lazer_match: Query<&mut Lazer>,
) {
    for mut direction in &mut direction_match {
        let mut new_direction = Direction::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            trace!("'A' / <-");
            new_direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            trace!("'D' / ->");
            new_direction = Direction::Right;
        }
        *direction = new_direction;
    }

    for mut lazer in &mut lazer_match {
        if *lazer == Lazer::Idle
            && (keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp))
        {
            trace!("' ' ^");
            *lazer = Lazer::Fire;
        }
    }
}

/// player movement
fn player_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform), With<Player>>,
) {
    for (direction, mut transform) in &mut sprite_position {
        match *direction {
            Direction::Left => {
                if transform.translation.x > -SCENE_WIDTH {
                    transform.translation.x -= PLAYER_SPEED * time.delta_seconds()
                }
            }
            Direction::Right => {
                if transform.translation.x < SCENE_WIDTH {
                    transform.translation.x += PLAYER_SPEED * time.delta_seconds()
                }
            }
            _ => {}
        }
    }
}

/// lazer movement
fn lazer_movement(
    time: Res<Time>,
    player_position: Query<&Transform, With<Player>>,
    mut lazer_position: Query<(&mut Lazer, &mut Visibility, &mut Transform), Without<Player>>,
) {
    let player_position = player_position.iter().next().unwrap();
    for (mut lazer, mut visibility, mut transform) in &mut lazer_position {
        match *lazer {
            Lazer::Fire => {
                transform.translation =
                    player_position.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);
                *lazer = Lazer::Fired;
                *visibility = Visibility::Visible;
            }
            Lazer::Fired => {
                if transform.translation.y > SCENE_HEIGHT {
                    *lazer = Lazer::Idle;
                    *visibility = Visibility::Hidden;
                } else {
                    transform.translation.y += LAZER_SPEED * time.delta_seconds()
                }
            }
            _ => {}
        }
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
        Player,
        Direction::None,
    ));
    commands.spawn((
        Lazer::Idle,
        SpriteBundle {
            texture: asset_server.load("sprites/lazer.png"),

            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));

    commands.spawn((
        Direction::None,
        SpriteBundle {
            texture: asset_server.load("sprites/alien.png"),
            transform: Transform::from_xyz(0., SCENE_HEIGHT, 0.),
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (keyboard_input_system, player_movement, lazer_movement),
        )
        .run();
}
