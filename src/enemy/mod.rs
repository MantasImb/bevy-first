use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (enemy_movement, update_enemy_direction).chain())
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, (tick_enemy_spawn_timer, spawn_enemies_over_time));
    }
}
