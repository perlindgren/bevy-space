use crate::{common::*, game_state::*};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

// high level keyboard actions based on user input
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Left,
    Right,
    Shoot,
    Slow1,
    Slow2,
    Slow3,
}

/// player movement
pub fn update_system(
    time: Res<Time>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_query.single_mut();

    let action_state = action_query.single();
    // Each action has a button-like state of its own that you can check
    let mut direction = if action_state.pressed(&PlayerAction::Left)
        && transform.translation.x > -SCENE_WIDTH
    {
        -1.0
    } else if action_state.pressed(&PlayerAction::Right) && transform.translation.x < SCENE_WIDTH {
        1.0
    } else {
        0.0
    };

    if action_state.pressed(&PlayerAction::Slow1) {
        direction *= PLAYER_SLOW;
    }
    if action_state.pressed(&PlayerAction::Slow2) {
        direction *= PLAYER_SLOW;
    }
    if action_state.pressed(&PlayerAction::Slow3) {
        direction *= PLAYER_SLOW;
    }

    transform.translation.x += direction * PLAYER_SPEED * time.delta_seconds();

    if action_state.just_pressed(&PlayerAction::Shoot) {
        println!("Shoot");
    }
}

// it uses the shared game_state to determine if visible
// alternatively one could declare an event to determine state changes
// but the cost is low so we don't do that
pub fn blink_update_system(
    store: Res<Store>,
    mut player_query: Query<&mut Visibility, With<Player>>,
) {
    let mut visibility = player_query.single_mut();

    *visibility = match store.game_state {
        GameState::PlayerSpawn(spawn_count) => {
            if spawn_count % 2u8 == 0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
        _ => Visibility::Visible,
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("sprites/space.png"),
            transform: Transform::from_xyz(0., -SCENE_HEIGHT, 0.),
            ..default()
        },
    ));

    // Describes how to convert from player inputs into those actions
    let input_map = InputMap::new([
        (PlayerAction::Left, KeyCode::ArrowLeft),
        (PlayerAction::Left, KeyCode::KeyA),
        (PlayerAction::Right, KeyCode::ArrowRight),
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Shoot, KeyCode::ArrowUp),
        (PlayerAction::Shoot, KeyCode::Space),
        (PlayerAction::Slow1, KeyCode::ShiftLeft),
        (PlayerAction::Slow1, KeyCode::ShiftRight),
        (PlayerAction::Slow2, KeyCode::AltRight),
        (PlayerAction::Slow2, KeyCode::AltLeft),
        (PlayerAction::Slow3, KeyCode::ControlLeft),
        (PlayerAction::Slow3, KeyCode::ControlRight),
    ]);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player);
}
