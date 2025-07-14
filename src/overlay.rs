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
pub struct ShowState;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

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
            ShowState,
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
            Text::new("SCORE"),
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
        .spawn((
            Text::new("--"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
        ))
        .with_child((
            Text::new("LIVES"),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..Default::default()
            },
            TextColor(WHITE.into()),
            // Node {
            //     position_type: PositionType::Absolute,
            //     top: Val::Px(5.0),
            //     left: Val::Px(15.0),
            //     ..default()
            // },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: STATUS_BAR_FONT_SIZE,
                ..default()
            },
            TextColor(GOLD.into()),
            Node {
                position_type: PositionType::Relative,
                ..default()
            },
            LivesText,
        ));

    // TextBundle::from_sections([
    //     TextSection::new(
    //         "LIVES: ",
    //         TextStyle {
    //             font_size: STATUS_BAR_FONT_SIZE,
    //             ..default()
    //         },
    //     ),
    //     // Lives = 1
    //     TextSection::from_style(TextStyle {
    //         font_size: STATUS_BAR_FONT_SIZE,
    //         color: GOLD.into(),
    //         ..default()
    //     }),
    //     TextSection::new(
    //         "WAVE: ",
    //         TextStyle {
    //             font_size: STATUS_BAR_FONT_SIZE,
    //             ..default()
    //         },
    //     ),
    //     // Wave = 3
    //     TextSection::from_style(TextStyle {
    //         font_size: STATUS_BAR_FONT_SIZE,
    //         color: GOLD.into(),
    //         ..default()
    //     }),
    // ])
    // .with_style(Style {
    //     position_type: PositionType::Absolute,
    //     top: Val::Px(5.0),
    //     left: Val::Px(15.0),
    //     ..default()
    // }),

    // GameOver
    commands.spawn((
        Overlay {
            game_state: GameState::GameOver,
        },
        // TextBundle::from_section(
        //     "   GAME OVER", // Ugly, but works
        //     TextStyle {
        //         font_size: GAME_OVER_FONT_SIZE,
        //         color: RED.into(),
        //         ..default()
        //     },
        // )
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     align_self: AlignSelf::Center,
        //     ..default()
        // }),
    ));
    // Insert Coin
    commands.spawn((
        Overlay {
            game_state: GameState::InsertCoin,
        },
        // TextBundle::from_section(
        //     "   Press Enter\n       to\n   Insert Coin", // Ugly, but works
        //     TextStyle {
        //         font_size: INSERT_COIN_FONT_SIZE,
        //         color: MAGENTA.into(),
        //         ..default()
        //     },
        // )
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     align_self: AlignSelf::Center,
        //     ..default()
        // }),
    ));

    // Start
    commands.spawn((
        Overlay {
            game_state: GameState::Start,
        },
        // TextBundle::from_section(
        //     "   Let's Go", // Ugly, but works
        //     TextStyle {
        //         font_size: START_FONT_SIZE,
        //         color: YELLOW.into(),
        //         ..default()
        //     },
        // )
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     align_self: AlignSelf::Center,
        //     ..default()
        // }),
    ));

    // New Wave
    commands.spawn((
        Overlay {
            game_state: GameState::NewWave,
        },
        // TextBundle::from_section(
        //     "    New Wave", // Ugly, but works
        //     TextStyle {
        //         font_size: NEW_WAVE_FONT_SIZE,
        //         color: YELLOW.into(),
        //         ..default()
        //     },
        // )
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     align_self: AlignSelf::Center,
        //     ..default()
        // }),
    ));

    // Leader Board
    commands.spawn((
        Overlay {
            game_state: GameState::LeaderBoard,
        },
        // TextBundle::from_section(
        //     " Leader Board", // Ugly, but works
        //     TextStyle {
        //         font_size: LEADER_BOARD_FONT_SIZE,
        //         color: DARK_CYAN.into(),
        //         ..default()
        //     },
        // )
        // .with_style(Style {
        //     position_type: PositionType::Absolute,
        //     align_self: AlignSelf::Center,
        //     ..default()
        // }),
    ));
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
    // mut status_query: Query<&mut Text, With<StatusBar>>,
    // mut score_query: Query<&mut Text, (With<Score>, Without<StatusBar>)>,
    mut lives_query: Query<&mut TextSpan, With<LivesText>>,
    //mut score_query: Query<&mut TextSpan, (With<ScoreText>, Without<LivesText>)>,
) {
    // let mut status_text = status_query.single_mut();
    // let mut score_text = score_query.single_mut();

    // status_text.sections[1].value = format!("{:1}  ", store.lives);
    // status_text.sections[3].value = format!("{:1}  ", store.wave);
    // score_text.sections[1].value = format!("{:06}", store.score);

    for mut span in &mut lives_query {
        info!("-");
        **span = "0".to_string(); //format!("{:1}  ", store.lives);
    }

    // for mut span in &mut score_query {
    //     **span = format!("{:06}", store.score);
    // }
}
pub fn state_update_system(
    store: ResMut<Store>,
    game_state_timer: Res<TimerResource>,

    mut show_state_query: Query<&mut Visibility, With<ShowState>>,
    // mut game_state_query: Query<(&mut Visibility, &mut Text, &Overlay), Without<ShowState>>,
) {
    for mut show_state_visibilty in show_state_query {
        *show_state_visibilty = if store.show_state {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }

    // // compute alpha from sinus of ratio between elapse time and timer duration

    // let ratio =
    //     game_state_timer.elapsed().as_secs_f32() / game_state_timer.duration().as_secs_f32();
    // let alpha = (PI * ratio).sin();
    // for (mut visibility, mut text, overlay) in &mut game_state_query {
    //     // text.sections[0].style.color.set_alpha(alpha);

    //     if overlay.game_state == store.game_state {
    //         *visibility = Visibility::Visible;
    //     } else {
    //         *visibility = Visibility::Hidden;
    //     }
    // }
}
