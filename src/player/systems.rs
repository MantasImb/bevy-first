use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    // window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // let window = window_query.get_single().unwrap();

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            speed: 500.0,
            size: 64.0,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * player.speed * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = player.size / 2.0;
        let x_min = window.width() / -2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = window.height() / -2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

        let mut translation = player_transform.translation;

        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);

        player_transform.translation = translation;
    }
}
