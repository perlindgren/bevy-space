use crate::{
    alien::*,
    audio::*,
    bunker::*,
    common::*,
    // game_state::{GameState, StateTransitionTimer, Store},
    game_state::{GameState, GameStateEvent, Store},
    lazer::Lazer,
    particle::*,
    player::Player,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn update_system(
    mut commands: Commands,
    mut store: ResMut<Store>,
    image: Res<CrossImage>,
    mut game_state_ew: EventWriter<GameStateEvent>,
    mut play_sound_ew: EventWriter<PlaySoundEvent>,
    alien_query: Query<(Entity, &Transform), With<Alien>>,
    mut lazer_query: Query<(&mut Lazer, &Transform)>,
    mut bunker_query: Query<(&mut Sprite, Entity, &Transform), With<Bunker>>,
    alien_bullet_query: Query<(Entity, &Transform), With<AlienBullet>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    // check if point:&Transform is in &target:Transform with size:Vec2
    #[inline(always)]
    fn in_rect(point: &Transform, target: &Transform, size: Vec2) -> bool {
        let t_vec: Vec2 = (target.translation.x, target.translation.y).into();
        let p_vec: Vec2 = (point.translation.x, point.translation.y).into();
        let rect = Rect::from_center_size(t_vec, size);
        rect.contains(p_vec)
    }

    let commands = &mut commands;

    // get lazer and player singletons
    if let (Ok((mut lazer, lazer_transform)), Ok(player_transform)) =
        (lazer_query.single_mut(), player_query.single_mut())
    {
        // alien bullets
        for (bullet_entity, bullet_transform) in &alien_bullet_query {
            // hit player missile
            if in_rect(bullet_transform, lazer_transform, (16.0, 32.0).into()) {
                commands.entity(bullet_entity).despawn();
                *lazer = Lazer::Idle;
                spawn_explosion(
                    commands,
                    &image,
                    10,
                    (
                        bullet_transform.translation.x,
                        bullet_transform.translation.y,
                    )
                        .into(),
                    150.0,
                    0.0,
                    (10.0, 10.0).into(),
                );
            } else
            // hit player
            if in_rect(bullet_transform, player_transform, PLAYER_SIZE) {
                commands.entity(bullet_entity).despawn();
                game_state_ew.write(GameStateEvent::LooseLife);
                // to prevent the rare race-condition when outstanding missile would cause an extra life

                *lazer = Lazer::Idle;

                spawn_explosion(
                    commands,
                    &image,
                    100,
                    (
                        bullet_transform.translation.x,
                        bullet_transform.translation.y,
                    )
                        .into(),
                    1000.0,
                    0.0,
                    (10.0, 10.0).into(),
                );
            } else {
                // hit bunker?
                for (mut bunker_sprite, bunker_entity, bunker_transform) in &mut bunker_query {
                    if in_rect(bullet_transform, bunker_transform, BUNKER_SIZE) {
                        commands.entity(bullet_entity).despawn();

                        if store.game_state == GameState::Play {
                            if let Some(ref mut bunker_atlas) = bunker_sprite.texture_atlas {
                                hit_bunker(commands, bunker_entity, bunker_atlas);
                            }
                        }

                        spawn_explosion(
                            commands,
                            &image,
                            10,
                            Vec2::from((
                                bullet_transform.translation.x,
                                bullet_transform.translation.y,
                            )),
                            150.0,
                            0.0,
                            (10.0, 10.0).into(),
                        );
                    }
                }
            }
        }

        if let Lazer::Fired(_) = *lazer {
            // check bunkers
            for (mut bunker_sprite, entity, bunker_transform) in &mut bunker_query {
                if in_rect(lazer_transform, bunker_transform, BUNKER_SIZE) {
                    if let Some(ref mut bunker_atlas) = bunker_sprite.texture_atlas {
                        hit_bunker(commands, entity, bunker_atlas);
                    }

                    *lazer = Lazer::Idle;
                    spawn_explosion(
                        commands,
                        &image,
                        5,
                        (lazer_transform.translation.x, lazer_transform.translation.y).into(),
                        50.0,
                        0.0,
                        (10.0, 10.0).into(),
                    );
                }
            }

            // check aliens
            for (alien_entity, enemy_transform) in &alien_query {
                // Collision check
                if in_rect(lazer_transform, enemy_transform, ALIEN_SIZE) {
                    play_sound_ew.write(PlaySoundEvent::AlienHit);
                    commands.entity(alien_entity).despawn();
                    *lazer = Lazer::Idle;
                    store.aliens_killed += 1;
                    store.alien_speed += ALIENS_SPEED_KILL;
                    store.score += SCORE_ALIEN;

                    spawn_explosion(
                        commands,
                        &image,
                        10,
                        (lazer_transform.translation.x, lazer_transform.translation.y).into(),
                        500.0,
                        0.0,
                        (10.0, 10.0).into(),
                    );

                    if store.aliens_killed == ALIENS_TOTAL {
                        debug!("-- send new wave --");
                        game_state_ew.write(GameStateEvent::NewWave);
                    }
                }
            }
        }
    }
}
