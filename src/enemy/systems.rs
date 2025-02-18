use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::events::*;
use crate::player::components::*;
use crate::score::resources::*;

use super::components::*;
use super::resources::*;

pub const NUMBER_OF_ENEMIES: usize = 4;

// TODO: Make it so that the enemies spawn in a circle around the player, one at a time with a
// delay between each spawn.
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = rand::rng();

    let width = window.width();
    let height = window.height();

    for _ in 0..NUMBER_OF_ENEMIES {
        let enemy_size = 64.0;
        let random_x = rng.random_range((width - enemy_size) / -2.0..(width - enemy_size) / 2.0);
        let random_y = rng.random_range((height - enemy_size) / -2.0..(height - enemy_size) / 2.0);

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random::<f32>(), rng.random::<f32>()).normalize(),
                speed: 200.0,
                size: enemy_size,
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_secs();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let half_enemy_size = enemy.size / 2.0;
        let translation = transform.translation;

        let x_min = window.width() / -2.0 + half_enemy_size;
        let x_max = window.width() / 2.0 - half_enemy_size;
        let y_min = window.height() / -2.0 + half_enemy_size;
        let y_max = window.height() / 2.0 - half_enemy_size;

        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = "sounds/pluck_001.ogg";
            let sound_effect_2 = "sounds/pluck_002.ogg";

            let sound_effect = if rand::random() {
                sound_effect_1
            } else {
                sound_effect_2
            };
            commands.spawn(AudioPlayer::new(asset_server.load(sound_effect)));
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

// BUG: If the player is in the center, the enemy will spawn on top of the player.
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if !enemy_spawn_timer.timer.finished() {
        return;
    }

    if let Ok(player) = player_query.get_single() {
        let player_translation = player.translation;
        let mut rng = rand::rng();

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Transform::from_xyz(
                player_translation.x * -1.0,
                player_translation.y * -1.0,
                0.1,
            ),
            Enemy {
                direction: Vec2::new(rng.random::<f32>(), rng.random::<f32>()).normalize(),
                speed: 200.0,
                size: 64.0,
            },
        ));
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform, &Player)>,
    enemy_query: Query<(&Transform, &Enemy)>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform, player)) = player_query.get_single_mut() {
        for (enemy_transform, enemy) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = player.size / 2.0;
            let enemy_radius = enemy.size / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Game over!");
                commands.spawn(AudioPlayer::new(
                    asset_server.load("sounds/explosionCrunch_000.ogg"),
                ));
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}
