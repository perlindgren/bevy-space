// use crate::{common::*, game_state::*, lazer::FireLazerMessage, player::PlayerMessage};
use crate::{common::*, game_state::*, lazer::FireLazerMessage, player::PlayerMessage};
use bevy::prelude::*;

/// keyboard input
pub fn update_system(
    mut fire_lazer_ew: MessageWriter<FireLazerMessage>,
    mut game_state_ew: MessageWriter<GameStateMessage>,
    mut player_ew: MessageWriter<PlayerMessage>,
    store: Res<Store>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // slow or fast
    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_SLOW
    } else {
        1.0
    };
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_ew.write(PlayerMessage(-speed));
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        player_ew.write(PlayerMessage(speed));
    }

    if keyboard_input.just_pressed(KeyCode::KeyI) {
        info!("Key I pressed (info)");
        game_state_ew.write(GameStateMessage::Info);
    }

    match store.game_state {
        GameState::InsertCoin | GameState::LeaderBoard => {
            if keyboard_input.just_pressed(KeyCode::Enter) {
                game_state_ew.write(GameStateMessage::PressPlay);
            }
        }
        GameState::PlayerSpawn(_) | GameState::Play => {
            if keyboard_input.just_pressed(KeyCode::Space)
                || keyboard_input.pressed(KeyCode::ArrowUp)
            {
                debug!("-- fire lazer event sent --");
                fire_lazer_ew.write(FireLazerMessage);
            }
        }
        _ => {}
    }
}
