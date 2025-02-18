use bevy::prelude::*;

use crate::events::*;
use crate::score::resources::*;

pub fn score_updated(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1));
        high_scores.scores.truncate(5);
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores:");
        for (i, (name, score)) in high_scores.scores.iter().enumerate() {
            println!("{}. {}: {}", i + 1, name, score);
        }
    }
}
