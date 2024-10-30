use crate::{common::*, game_state::*};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

// high level keyboard actions based on user input
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerKeyboardAction {
    Left,
    Right,
    Shoot,
    Slow1,
    Slow2,
    Slow3,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerGamePadAction {
    #[actionlike(DualAxis)]
    Move,
    Shoot,
}

/// player movement
pub fn keyboard_update_system(
    time: Res<Time>,
    keyboard_action_query: Query<&ActionState<PlayerKeyboardAction>, With<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_query.single_mut();

    let action_state = keyboard_action_query.single();
    // Each action has a button-like state of its own that you can check
    let mut direction = if action_state.pressed(&PlayerKeyboardAction::Left)
        && transform.translation.x > -SCENE_WIDTH
    {
        -1.0
    } else if action_state.pressed(&PlayerKeyboardAction::Right)
        && transform.translation.x < SCENE_WIDTH
    {
        1.0
    } else {
        0.0
    };

    if action_state.pressed(&PlayerKeyboardAction::Slow1) {
        direction *= PLAYER_SLOW;
    }
    if action_state.pressed(&PlayerKeyboardAction::Slow2) {
        direction *= PLAYER_SLOW;
    }
    if action_state.pressed(&PlayerKeyboardAction::Slow3) {
        direction *= PLAYER_SLOW;
    }

    transform.translation.x += direction * PLAYER_SPEED * time.delta_seconds();
}

pub fn gamepad_update_system(
    time: Res<Time>,
    query: Query<&ActionState<PlayerGamePadAction>, With<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let action_state = query.single();
    let mut transform = player_query.single_mut();

    let axis_pair = action_state.clamped_axis_pair(&PlayerGamePadAction::Move);
    transform.translation.x += axis_pair.x * PLAYER_SPEED * time.delta_seconds();
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

    // Describes how to convert keyboard inputs into actions
    let input_map = InputMap::new([
        (PlayerKeyboardAction::Left, KeyCode::ArrowLeft),
        (PlayerKeyboardAction::Left, KeyCode::KeyA),
        (PlayerKeyboardAction::Right, KeyCode::ArrowRight),
        (PlayerKeyboardAction::Right, KeyCode::KeyD),
        (PlayerKeyboardAction::Shoot, KeyCode::ArrowUp),
        (PlayerKeyboardAction::Shoot, KeyCode::Space),
        (PlayerKeyboardAction::Slow1, KeyCode::ShiftLeft),
        (PlayerKeyboardAction::Slow1, KeyCode::ShiftRight),
        (PlayerKeyboardAction::Slow2, KeyCode::AltRight),
        (PlayerKeyboardAction::Slow2, KeyCode::AltLeft),
        (PlayerKeyboardAction::Slow3, KeyCode::ControlLeft),
        (PlayerKeyboardAction::Slow3, KeyCode::ControlRight),
    ]);
    commands.spawn((Player, InputManagerBundle::with_map(input_map)));
    // Describes how to convert gamepad inputs into actions
    let input_map = InputMap::default()
        // Let's bind the left stick for the move action
        .with_dual_axis(PlayerGamePadAction::Move, GamepadStick::LEFT)
        // And then bind the right gamepad trigger to the throttle action
        .with(PlayerGamePadAction::Shoot, GamepadButtonType::South)
        .with(PlayerGamePadAction::Shoot, GamepadButtonType::East);
    commands.spawn((Player, InputManagerBundle::with_map(input_map)));
}
