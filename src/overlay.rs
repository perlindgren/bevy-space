//! It displays the current FPS in the top left corner and score top right
use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{DARK_CYAN, GOLD, MAGENTA, RED, WHITE, YELLOW},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    common::*,
    game_state::{GameState, Store, TimerResource},
};

//
#[derive(Component)]
pub struct ShowFps;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct StatusBar;

#[derive(Component, Debug)]
pub struct Overlay {
    game_state: GameState,
}

pub fn setup(mut commands: Commands) {
    // Show FPS
    commands
        .spawn((
            ShowFps,
            Text::new("FPS"),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..Default::default()
            },
            TextColor(WHITE.into()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(15.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..default()
            },
            TextColor(GOLD.into()),
            FpsText,
        ));

    // Show Score
    commands
        .spawn((
            Text::new("SCORE:"),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..Default::default()
            },
            TextColor(WHITE.into()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..default()
            },
            TextColor(GOLD.into()),
            ScoreText,
        ));

    // Status Bar
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("LIVES:"),
                    TextFont {
                        font_size: STATUS_BAR_FONT_SIZE,
                        ..Default::default()
                    },
                    TextColor(WHITE.into()),
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: STATUS_BAR_FONT_SIZE,
                        ..default()
                    },
                    TextColor(GOLD.into()),
                    LivesText,
                ));
            parent
                .spawn((
                    Text::new("WAVE:"),
                    TextFont {
                        font_size: STATUS_BAR_FONT_SIZE,
                        ..Default::default()
                    },
                    TextColor(WHITE.into()),
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: STATUS_BAR_FONT_SIZE,
                        ..default()
                    },
                    TextColor(GOLD.into()),
                    WaveText,
                ));
        });

    // Show GameOver
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,

            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Overlay {
                    game_state: GameState::GameOver,
                },
                Text::new("Game Over"),
                TextFont {
                    font_size: INSERT_COIN_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(RED.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

    // Show Insert Coin
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            min_width: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Overlay {
                    game_state: GameState::InsertCoin,
                },
                Text::new("Press Enter\nto\nInsert Coin"),
                TextFont {
                    font_size: INSERT_COIN_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(MAGENTA.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

    // Show Start
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            min_width: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Overlay {
                    game_state: GameState::Start,
                },
                Text::new("Let's Go"),
                TextFont {
                    font_size: INSERT_COIN_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(YELLOW.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

    // Show New Wave
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            min_width: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Overlay {
                    game_state: GameState::NewWave,
                },
                Text::new("New Wave"),
                TextFont {
                    font_size: INSERT_COIN_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(YELLOW.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });

    // Show Leader Board
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            min_width: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Overlay {
                    game_state: GameState::LeaderBoard,
                },
                Text::new("Leader Board"),
                TextFont {
                    font_size: INSERT_COIN_FONT_SIZE,
                    ..Default::default()
                },
                TextColor(DARK_CYAN.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

pub fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}

pub fn score_update_system(
    store: Res<Store>,

    mut state: ParamSet<(
        Query<&mut TextSpan, With<LivesText>>,
        Query<&mut TextSpan, With<WaveText>>,
        Query<&mut TextSpan, With<ScoreText>>,
    )>,
) {
    for mut span in state.p0().iter_mut() {
        **span = format!("{:1}  ", store.lives);
    }

    for mut span in state.p1().iter_mut() {
        **span = format!("{:1}  ", store.wave);
    }

    for mut span in &mut state.p2().iter_mut() {
        **span = format!("{:06}", store.score);
    }
}
pub fn state_update_system(
    store: ResMut<Store>,
    game_state_timer: Res<TimerResource>,

    show_state_query: Query<&mut Visibility, With<ShowFps>>,
    game_state_query: Query<(&mut Visibility, &mut TextColor, &Overlay), Without<ShowFps>>,
) {
    for mut show_state_visibilty in show_state_query {
        *show_state_visibilty = if store.show_state {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    // compute alpha from sinus of ratio between elapse time and timer duration
    let ratio =
        game_state_timer.elapsed().as_secs_f32() / game_state_timer.duration().as_secs_f32();
    let alpha = (PI * ratio).sin();
    for (mut visibility, mut text, overlay) in game_state_query {
        text.set_alpha(alpha);

        if overlay.game_state == store.game_state {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
