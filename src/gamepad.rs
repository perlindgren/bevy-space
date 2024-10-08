use crate::{common::*, game_state::*, lazer::FireLazerEvent, player::PlayerEvent};
use bevy::prelude::*;

pub fn update_system(
    mut fire_lazer_ew: EventWriter<FireLazerEvent>,
    mut game_state_ew: EventWriter<GameStateEvent>,
    mut player_ew: EventWriter<PlayerEvent>,

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
                    game_state_ew.send(GameStateEvent::PressPlay);
                }
                GameState::PlayerSpawn(_) | GameState::Play => {
                    fire_lazer_ew.send(FireLazerEvent);
                }
                _ => {}
            }
        }

        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
            trace!("{:?} just pressed North", gamepad);
            game_state_ew.send(GameStateEvent::Info);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        // hysteresis set at 0.01 to avoid drift
        if left_stick_x.abs() > LEFT_STICK_HYSTERESIS {
            trace!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
            player_ew.send(PlayerEvent(left_stick_x));
        }
    }
}
