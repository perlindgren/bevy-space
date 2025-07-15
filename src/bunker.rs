use crate::{common::*, game_state::*};
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Bunker;

#[inline(always)]
pub fn hit_bunker(commands: &mut Commands, entity: Entity, mut atlas: Mut<TextureAtlas>) {
    if atlas.index < 10 {
        atlas.index += 5;
    } else {
        commands.entity(entity).despawn();
    }
}

pub fn setup_borrowed(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    // Builds and spawns the bunker sprites
    let texture = asset_server.load("sprites/defense.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The sprite index layout of the bunker
    let bunker_matrix = [
        [0, 1, 1, 1, 1, 2],
        [1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1],
        [1, 3, 5, 5, 4, 1],
    ];

    for b in 0..BUNKERS {
        let mut bunker = vec![];
        for (r, row) in bunker_matrix.iter().enumerate() {
            for (c, data) in row.iter().enumerate() {
                if *data < 5 {
                    bunker.push((
                        Bunker,
                        Sprite::from_atlas_image(
                            texture.clone(),
                            TextureAtlas {
                                layout: texture_atlas_layout.clone(),
                                index: *data,
                            },
                        ),
                        Transform::from_xyz(
                            (c as f32 - (row.len() as f32 - 1.0) / 2.0) * 16.0
                                + (2.0 * b as f32 - (BUNKERS as f32 - 1.0)) * BUNKER_SPACE,
                            BUNKERS_Y - SCENE_HEIGHT - (r as f32) * 16.0,
                            0.0,
                        ),
                    ));
                }
            }
        }
        commands.spawn_batch(bunker);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    setup_borrowed(&mut commands, &asset_server, &mut texture_atlas_layouts);
}

pub fn reset(
    // reset the bunkers
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    bunker_query: Query<Entity, With<Bunker>>,
) {
    cleanup_state(commands, bunker_query);
    setup_borrowed(commands, asset_server, texture_atlas_layout);
}
