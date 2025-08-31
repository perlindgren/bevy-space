use crate::common::*;
use bevy::prelude::*;
use rand::random;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component)]
pub struct Particle {
    timer: Timer,
    delta: Vec2,
    delta_random: Vec2,
}

pub fn update_system(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Sprite, &mut Transform, &mut Particle)>,
) {
    for (entity, mut sprite, mut transform, mut particle) in &mut bullet_query {
        particle.timer.tick(time.delta());

        let ratio = 1.0 - particle.timer.elapsed().as_secs_f32() / PARTICLE_DURATION;
        sprite.color.set_alpha(ratio);

        if particle.timer.just_finished() {
            commands.entity(entity).despawn();
        } else {
            let translation = &mut transform.translation;
            translation.x += (particle.delta.x + (random::<f32>() - 0.5) * particle.delta_random.x)
                * time.delta_seconds();
            translation.y += (particle.delta.y + (random::<f32>() - 0.5) * particle.delta_random.y)
                * time.delta_seconds();
        }
    }
}


pub fn spawn_particle(
    commands: &mut Commands,
    image: &Res<CrossImage>,
    pos: Vec2,
    delta: Vec2,
    delta_random: Vec2,
) {
    commands.spawn((
        Particle {
            timer: Timer::new(Duration::from_secs_f32(PARTICLE_DURATION), TimerMode::Once),
            delta,
            delta_random,
        },
        SpriteBundle {
            texture: image.0.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
    ));
}

pub fn spawn_explosion(
    commands: &mut Commands,
    image: &Res<CrossImage>,
    nr_rays: usize,
    pos: Vec2,
    speed: f32,
    _speed_random: f32,
    delta_random: Vec2,
) {
    for i in 0..nr_rays {
        let angle = TAU * (i as f32) / nr_rays as f32;
        commands.spawn((
            Particle {
                timer: Timer::new(Duration::from_secs_f32(PARTICLE_DURATION), TimerMode::Once),
                delta: (speed * angle.sin(), speed * angle.cos()).into(),
                delta_random,
            },
            SpriteBundle {
                texture: image.0.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
        ));
    }
}

// Here we can provide different particle shapes, just a cross for now

#[derive(Resource, Clone)]
pub struct CrossImage(Handle<Image>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Loads bullet sprite and store resource
    commands.insert_resource(CrossImage(asset_server.load("sprites/cross.png")))
}
