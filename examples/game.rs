//! Renders a 2D scene containing a single, moving sprite.
//! RUST_LOG="game=info" cargo run --example game

use bevy::{prelude::*, window::WindowResolution};

// vintage television format
const RES_Y: f32 = 1080.0; // well a bit too modern
const RES_X: f32 = RES_Y * 4.0 / 3.0;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
const LAZER_SPEED: f32 = 1000.0;

const SCENE_WIDTH: f32 = RES_X / 2.0 - 100.0;
const SCENE_HEIGHT: f32 = RES_Y / 2.0 - 50.0;
const ALIENS_COL: usize = 11;
const ALIENS_ROW: usize = 5;
const ALIENS_WIDTH: f32 = 64.0; // used for hit box
const ALIENS_HEIGHT: f32 = 10.0; // used for hit box
const ALIENS_SPACE: f32 = 80.0; // used for layout

const HALF_W: f32 = ALIENS_WIDTH / 2.0;
const HALF_H: f32 = ALIENS_HEIGHT / 2.0;

const BUNKERS: usize = 5;
const BUNKER_SPACE: f32 = SCENE_WIDTH / BUNKERS as f32;
const BUNKERS_Y: f32 = 150.0;

const ALIENS_SPEED: f32 = 30.0;

#[derive(Component)]
enum Player {
    Left,
    Right,
    None,
}

#[derive(Component, PartialEq, Clone, Copy)]
enum Lazer {
    Fire,
    Fired,
    Idle,
}

/// keyboard input
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction_match: Query<&mut Player>,
    mut lazer_match: Query<&mut Lazer>,
) {
    for mut direction in &mut direction_match {
        let mut new_direction = Player::None;
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_direction = Player::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            new_direction = Player::Right;
        }
        *direction = new_direction;
    }

    for mut lazer in &mut lazer_match {
        if *lazer == Lazer::Idle
            && (keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp))
        {
            *lazer = Lazer::Fire;
        }
    }
}

/// player movement
fn player_movement(time: Res<Time>, mut player: Query<(&mut Player, &mut Transform)>) {
    for (direction, mut transform) in &mut player {
        match *direction {
            Player::Left => {
                if transform.translation.x > -SCENE_WIDTH {
                    transform.translation.x -= PLAYER_SPEED * time.delta_seconds()
                }
            }
            Player::Right => {
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
    player_query: Query<&Transform, With<Player>>,
    mut lazer_position: Query<(&mut Lazer, &mut Visibility, &mut Transform), Without<Player>>,
) {
    // get a player_transform singleton
    let mut player_iterator = player_query.iter();
    let player_transform = player_iterator.next().unwrap();
    assert!(player_iterator.next().is_none());

    let mut lazer_iterator = lazer_position.iter_mut();
    let (mut lazer, mut visibility, mut transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    match *lazer {
        Lazer::Fire => {
            transform.translation =
                player_transform.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0);
            *lazer = Lazer::Fired;
            *visibility = Visibility::Visible;
        }
        Lazer::Fired => {
            if transform.translation.y > SCENE_HEIGHT {
                *lazer = Lazer::Idle;
            } else {
                transform.translation.y += LAZER_SPEED * time.delta_seconds()
            }
        }
        _ => {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}
#[derive(Component)]
struct Alien {
    direction: Direction,
}

/// alien movement
fn alien_movement(time: Res<Time>, mut aliens: Query<(&mut Alien, &mut Transform)>) {
    let mut new_direction = None;
    for (alien, mut transform) in &mut aliens {
        match alien.direction {
            Direction::Left => {
                transform.translation.x -= ALIENS_SPEED * time.delta_seconds();
                if transform.translation.x < -SCENE_WIDTH {
                    new_direction = Some(Direction::Right);
                }
            }
            Direction::Right => {
                transform.translation.x += ALIENS_SPEED * time.delta_seconds();
                if transform.translation.x > SCENE_WIDTH {
                    new_direction = Some(Direction::Left);
                }
            }
        }
    }

    if let Some(direction) = new_direction {
        for (mut alien, mut transform) in &mut aliens {
            transform.translation.y -= ALIENS_HEIGHT;
            alien.direction = direction;
        }
    }
}

fn hit_detection(
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
) {
    // get lazer singleton
    let mut lazer_iterator = lazer_query.iter_mut();
    let (mut lazer, lazer_transform) = lazer_iterator.next().unwrap();
    assert!(lazer_iterator.next().is_none());

    if *lazer == Lazer::Fired {
        let lazer_x = lazer_transform.translation.x;
        let lazer_y = lazer_transform.translation.y;

        for (entity, enemy_transform) in alien_query.iter() {
            let x = enemy_transform.translation.x;
            let y = enemy_transform.translation.y;

            // hit box
            let x_range = (x - HALF_W)..(x + HALF_W);
            let y_range = (y - HALF_H)..(y + HALF_H);

            // Collision check
            if x_range.contains(&lazer_x) && (y_range.contains(&lazer_y)) {
                println!(
                    "hit at x {}, y {}, lazer_x {}, lazer_y {}, x_range {:?}, y_range {:?}",
                    x, y, lazer_x, lazer_y, x_range, y_range
                );
                commands.entity(entity).despawn();
                *lazer = Lazer::Idle;
                break; // ensure only one hit
            }
        }
    }
}

#[derive(Component)]
struct Defense(u8); // index in sprite atlas

#[derive(Component)]
struct HitBox {
    rect: Rect,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // we might want to setup a custom camera, for now just default
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player::None,
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
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

    // Builds and spawns the Alien sprites
    let sprite_handle = asset_server.load("sprites/alien.png");

    let mut aliens = vec![];
    let step_x = ALIENS_SPACE;
    let step_y = ALIENS_SPACE;
    for y in 0..ALIENS_ROW {
        for x in 0..ALIENS_COL {
            aliens.push((
                Alien {
                    direction: Direction::Right,
                },
                SpriteBundle {
                    texture: sprite_handle.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - ALIENS_COL as f32 / 2.0) * step_x,
                        SCENE_HEIGHT - (y as f32 * step_y),
                        -1.0, // behind in scene
                    ),
                    ..Default::default()
                },
            ));
        }
    }
    commands.spawn_batch(aliens);

    // Builds and spawns the Defense spites
    let texture = asset_server.load("sprites/defense.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    //
    let bunker_matrix = [
        [0, 1, 1, 1, 1, 2],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
    ];

    for b in 0..BUNKERS {
        let mut bunker = vec![];
        for (r, row) in bunker_matrix.iter().enumerate() {
            for (c, data) in row.iter().enumerate() {
                bunker.push((
                    Defense(*data as u8),
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (c as f32 - (row.len() as f32 - 1.0) / 2.0) * 16.0
                                + (2.0 * b as f32 - (BUNKERS as f32 - 1.0)) * BUNKER_SPACE,
                            BUNKERS_Y - SCENE_HEIGHT - (r as f32) * 16.0,
                            0.0,
                        ),
                        texture: texture.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: *data,
                        ..default()
                    },
                ));
            }
        }
        commands.spawn_batch(bunker);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RES_X, RES_Y),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_input_system,
                hit_detection,
                player_movement,
                lazer_movement,
                alien_movement,
            ), // now all systems parallel
               // .chain(), // all systems in sequential order to keep it simple
        )
        .run();
}
