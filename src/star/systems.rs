use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::player::components::*;
use crate::score::resources::*;

use super::components::*;
use super::resources::*;

pub const NUMBER_OF_STARS: usize = 10;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::rng();

    let width = window.width();
    let height = window.height();

    for _ in 0..NUMBER_OF_STARS {
        let star_size = 64.0;
        let random_x = rng.random_range((width - star_size) / -2.0..(width - star_size) / 2.0);
        let random_y = rng.random_range((height - star_size) / -2.0..(height - star_size) / 2.0);

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.1),
            Star { size: star_size },
        ));
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Player)>,
    star_query: Query<(Entity, &Transform, &Star)>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok((player_transform, player)) = player_query.get_single_mut() {
        for (star_entity, star_transform, star) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let player_radius = player.size / 2.0;
            let star_radius = star.size / 2.0;

            if distance < player_radius + star_radius {
                score.value += 1;
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/laserLarge_000.ogg"),
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let mut rng = rand::rng();
        let random_x = rng.random_range(window.width() / -2.0..window.width() / 2.0);
        let random_y = rng.random_range(window.height() / -2.0..window.height() / 2.0);

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.1),
            Star { size: 64.0 },
        ));
    }
}
