use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, update_high_scores)
            .add_systems(Update, (score_updated, high_scores_updated));
    }
}
