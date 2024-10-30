use crate::game_state::*;
use bevy::prelude::*;

/// keyboard input
pub fn update_system(
    mut game_state_ew: EventWriter<GameStateEvent>,
    store: Res<Store>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    match store.game_state {
        GameState::InsertCoin | GameState::LeaderBoard => {
            if keyboard_input.just_pressed(KeyCode::Enter) {
                game_state_ew.send(GameStateEvent::PressPlay);
            }
        }

        _ => {}
    }
}
