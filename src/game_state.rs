use crate::common::*;
use crate::{
    alien,
    bunker::{self, Bunker},
    player::Player,
};
use bevy::prelude::*;
use std::{default::Default, time::Instant};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameState {
    GameOver,
    InsertCoin,
    LeaderBoard,
    Start,
    Play,
    NewWave,
}

#[derive(Resource)]
pub struct Store {
    pub texture_handler: Option<Handle<Image>>,
    pub instant: Instant,
    pub score: u32,
    pub aliens_killed: u8,
    pub alien_speed: f32,
    pub wave: u8,
    pub lives: u8,
    pub player_count_down: f32,
    pub game_state: GameState,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            texture_handler: None,
            instant: Instant::now(),
            score: 0,
            aliens_killed: 0,
            alien_speed: ALIENS_SPEED_START,
            wave: 1,
            lives: 0,
            player_count_down: 3.0,
            game_state: GameState::InsertCoin,
        }
    }
}

impl Store {
    pub fn reset(&mut self) {
        *self = Self {
            texture_handler: self.texture_handler.clone(), // will this leak?
            ..default()
        }
    }
}

// Loads the drop sprite and stores handle in the Store resource
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loads bullet sprite
    commands.insert_resource(Store {
        texture_handler: Some(asset_server.load("sprites/drop.png")),
        ..default()
    });
    commands.spawn(StateTransitionTimer(Timer::from_seconds(
        STATE_TRANSITION_DURATION,
        TimerMode::Repeating,
    )));
}

pub fn cleanup_state<T>(commands: &mut Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for item in &query {
        commands.entity(item).despawn_recursive();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct StateTransitionTimer(Timer);

#[allow(clippy::too_many_arguments)]
pub fn state_transition_system(
    mut commands: Commands,
    time: Res<Time>,
    mut store: ResMut<Store>,
    mut query: Query<&mut StateTransitionTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    alien_query: Query<Entity, With<alien::Alien>>,
    alien_bullet_query: Query<Entity, With<alien::AlienBullet>>,
    bunker_query: Query<Entity, With<Bunker>>,
    mut player_query: Query<&mut Player>,
) {
    let mut timer = query.single_mut();
    timer.tick(time.delta());
    let mut player = player_query.single_mut();
    if timer.just_finished() {
        println!("state transition");
        store.game_state = match store.game_state {
            GameState::GameOver => GameState::InsertCoin,
            GameState::InsertCoin => GameState::LeaderBoard,
            GameState::LeaderBoard => GameState::GameOver,
            GameState::Start => {
                println!("--- Start ---");
                alien::reset(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlas_layout,
                    alien_query,
                    alien_bullet_query,
                );
                bunker::reset(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlas_layout,
                    bunker_query,
                );
                store.reset();
                store.lives = NR_LIVES;
                player.spawn_counter = PLAYER_SPAWN_COUNTER;
                GameState::Play
            }
            GameState::Play => GameState::Play,
            GameState::NewWave => {
                println!("transition to new wave");
                GameState::Play
            }
        }
    }
}
