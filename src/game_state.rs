use crate::{
    alien,
    audio::PlayMusicEvent,
    bunker::{self, Bunker},
    common::*,
    lazer::{LazerData}
};
use bevy::prelude::*;
use std::{default::Default, time::Duration};

#[derive(PartialEq, Debug)]
pub enum GameState {
    GameOver,
    InsertCoin,
    LeaderBoard,
    Start,
    PlayerSpawn(u8),
    Play,
    NewWave,
}

#[derive(Resource)]
pub struct Store {
    pub score: u32,
    pub score_new_life: u32,
    pub bullet_interval: f32,
    pub aliens_killed: u8,
    pub alien_speed: f32,
    pub wave: u8,
    pub lives: u8,
    pub player_count_down: f32,
    pub game_state: GameState,
    pub lazers: Vec<LazerData>,
    pub lazer_interval: f32,
    pub show_state: bool,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            score: 0,
            score_new_life: 100,
            bullet_interval: ALIEN_BULLET_INTERVAL,
            aliens_killed: 0,
            alien_speed: ALIENS_SPEED_START,
            wave: 1,
            lives: 0,
            player_count_down: 3.0,
            game_state: GameState::InsertCoin,
            lazers: vec![],
            lazer_interval: 0.0,
            show_state: false,
        }
    }
}

impl Store {
    pub fn reset(&mut self) {
        *self = Self { ..default() }
    }
}

// Store resource and StateTransitionTimer
pub fn setup(mut commands: Commands) {
    commands.insert_resource(Store { ..default() });
    commands.insert_resource(TimerResource(Timer::from_seconds(
        STATE_TRANSITION_MENU,
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

#[derive(Resource, Deref, DerefMut)]
pub struct TimerResource(Timer);

impl TimerResource {
    pub fn set(&mut self, duration: f32) {
        self.0.set_duration(Duration::from_secs_f32(duration));
        self.0.unpause();
        self.0.reset();
    }
}

#[derive(Event, Debug)]
pub enum GameStateEvent {
    PressPlay,
    LooseLife,
    NewWave,
    Info,
}

pub fn game_state_event_system(
    mut game_state_er: EventReader<GameStateEvent>,
    mut play_music_event_writer: EventWriter<PlayMusicEvent>,
    mut store: ResMut<Store>,
    mut timer: ResMut<TimerResource>,
) {
    for event in game_state_er.read() {
        debug!("game state event received : {:?}", event);
        match event {
            GameStateEvent::PressPlay => {
                debug!("press play received");
                play_music_event_writer.send(PlayMusicEvent(false));
                store.reset();
                store.lives = NR_LIVES;
                store.game_state = GameState::Start;
                timer.set(STATE_TRANSITION_START);
            }
            GameStateEvent::LooseLife => {
                if store.game_state == GameState::Play {
                    store.lives -= 1;
                    if store.lives <= 0 {
                        store.game_state = GameState::GameOver;
                        timer.set(STATE_TRANSITION_MENU);
                    } else {
                        store.game_state = GameState::PlayerSpawn(PLAYER_SPAWN_COUNTER);
                        timer.set(STATE_TRANSITION_SPAWN);
                    }
                }
            }
            GameStateEvent::NewWave => {
                store.game_state = GameState::NewWave;
                store.aliens_killed = 0;
                store.alien_speed = ALIENS_SPEED_START + store.wave as f32 * ALIENS_SPEED_WAVE;
                store.wave += 1;
                store.bullet_interval *= BULLET_INTERVAL_WAVE;
                timer.set(STATE_TRANSITION_NEW_WAVE);
            }
            GameStateEvent::Info => {
                debug!("info received");
                store.show_state ^= true;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut store: ResMut<Store>,
    mut timer: ResMut<TimerResource>,

    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    alien_query: Query<Entity, With<alien::Alien>>,
    alien_bullet_query: Query<Entity, With<alien::AlienBullet>>,
    bunker_query: Query<Entity, With<Bunker>>,
) {
    timer.tick(time.delta());

    // extra life(s)
    // I think this is what was causing your race condition. But honestly this could be a cool mechanic
    // Like a final shot to revive mechanic
    if store.score >= store.score_new_life && store.lives >= 1 {
        store.lives += 1;
        store.score_new_life += (store.score_new_life as f32 * SCORE_SCALE) as u32;
    }

    // state transition
    if timer.just_finished() {
        store.game_state = match store.game_state {
            GameState::GameOver => GameState::InsertCoin,
            GameState::InsertCoin => GameState::LeaderBoard,
            GameState::LeaderBoard => GameState::InsertCoin,
            GameState::Start | GameState::NewWave => {
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

                if store.game_state == GameState::Start {
                    debug!("--- Start ---");
                } else {
                    debug!("--- New Wave ---");
                }
                timer.set(STATE_TRANSITION_SPAWN);
                GameState::PlayerSpawn(PLAYER_SPAWN_COUNTER)
            }
            GameState::PlayerSpawn(ref mut p) => {
                debug!("--- Player Spawn ---");
                *p -= 1;
                if *p == 0 {
                    debug!("--- Play, pause timer ---");
                    timer.pause();
                    GameState::Play
                } else {
                    GameState::PlayerSpawn(*p)
                }
            }
            GameState::Play => {
                panic!("timer should be paused, thus unreachable");
            }
        };
    }
}
