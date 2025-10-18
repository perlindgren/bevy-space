use crate::{common::*, game_state::*, lazer::FireLazerMessage, player::PlayerMessage};
use bevy::prelude::*;

pub fn update_system(
    mut fire_lazer_ew: MessageWriter<FireLazerMessage>,
    mut game_state_ew: MessageWriter<GameStateMessage>,
    mut player_ew: MessageWriter<PlayerMessage>,

    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,

    axes: Res<Axis<GamepadAxis>>,
    store: ResMut<Store>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            trace!("{:?} just pressed South", gamepad);
            match store.game_state {
                GameState::InsertCoin | GameState::LeaderBoard => {
                    game_state_ew.send(GameStateMessage::PressPlay);
                }
                GameState::PlayerSpawn(_) | GameState::Play => {
                    fire_lazer_ew.send(FireLazerMessage);
                }
                _ => {}
            }
        }

        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
            trace!("{:?} just pressed North", gamepad);
            game_state_ew.send(GameStateMessage::Info);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        // hysteresis set at 0.01 to avoid drift
        if left_stick_x.abs() > LEFT_STICK_HYSTERESIS {
            trace!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
            player_ew.send(PlayerMessage(left_stick_x));
        }
    }
}
