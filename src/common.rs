use bevy::prelude::*;

// vintage television format
pub const RES_Y: f32 = 1080.0; // well a bit too modern
pub const RES_X: f32 = RES_Y * 4.0 / 3.0;

pub const LEFT_STICK_HYSTERESIS: f32 = 0.01; // tune threshold to avoid drift

// Game mechanics related
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SLOW: f32 = 1.0 / 5.0; // The ratio for slow movement
pub const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 40.0);
pub const PLAYER_HEIGHT: f32 = 50.0; // There should be a way to get this from sprite
pub const PLAYER_SPAWN_COUNTER: u8 = 20;
pub const PLAYER_SPAWN_DURATION: f32 = 0.1;

pub const LAZER_SPEED: f32 = 1250.0;
pub const LAZER_PARTICLE_INTERVAL: f32 = 0.02; // Duration in seconds between particles
pub const LAZER_FIRING_INTERVAL: f32 = 0.5; // Cooldown for lazers

pub const SCENE_WIDTH: f32 = RES_X / 2.0 - 100.0;
pub const SCENE_HEIGHT: f32 = RES_Y / 2.0 - 50.0;
pub const ALIENS_COL: usize = 11;
pub const ALIENS_ROW: usize = 5;
pub const ALIENS_TOTAL: u8 = ALIENS_COL as u8 * ALIENS_ROW as u8;
pub const ALIENS_SPACE: f32 = 80.0; // used for layout
pub const ALIEN_SIZE: Vec2 = Vec2::new(64.0, 40.0); // used for hit box
pub const ALIEN_BULLET_SPEED: f32 = 300.0;
pub const ALIEN_BULLET_INTERVAL: f32 = 0.25; // in seconds
pub const BULLET_INTERVAL_WAVE: f32 = 0.75;
pub const BUNKERS: usize = 5;
pub const BUNKER_SPACE: f32 = SCENE_WIDTH / BUNKERS as f32;
pub const BUNKERS_Y: f32 = 100.0;
pub const BUNKER_SIZE: Vec2 = Vec2::new(16.0, 16.0);

pub const ALIENS_SPEED_START: f32 = 30.0;
pub const ALIENS_SPEED_KILL: f32 = 2.0;
pub const ALIENS_SPEED_WAVE: f32 = 10.0;
pub const ALIENS_SPEED_MAX: f32 = 100.0;

pub const PARTICLE_DURATION: f32 = 0.5; // in seconds

// Game UI related
pub const STATUS_BAR_FONT_SIZE: f32 = 50.0;
pub const GAME_OVER_FONT_SIZE: f32 = 200.0;
pub const INSERT_COIN_FONT_SIZE: f32 = 175.0;
pub const NEW_WAVE_FONT_SIZE: f32 = 200.0;
pub const START_FONT_SIZE: f32 = 200.0;
pub const LEADER_BOARD_FONT_SIZE: f32 = 200.0;

pub const STATE_TRANSITION_MENU: f32 = 6.0;
pub const STATE_TRANSITION_START: f32 = 2.0;
pub const STATE_TRANSITION_NEW_WAVE: f32 = 1.5;
pub const STATE_TRANSITION_SPAWN: f32 = 0.25;

// Game logic related
pub const SCORE_ALIEN: u32 = 10;
pub const SCORE_NEW_LIFE: u32 = 1000;
pub const SCORE_SCALE: f32 = 1.5;

pub const NR_LIVES: u8 = 3;
#[derive(Default, Clone, Copy)]
pub enum Direction3 {
    Left,
    Right,
    #[default]
    None,
}
