// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub mod events;
pub mod systems;

mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use events::*;
use systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "First game".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1000., 1000.).into(),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins((PlayerPlugin, EnemyPlugin, StarPlugin, ScorePlugin))
        .add_event::<GameOver>()
        .add_systems(Startup, setup)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
