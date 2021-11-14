use bevy::prelude::*;
use crate::{
    turn_structure::TurnState,
    plants::{Plant, RoundsTillMature},
};

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(score.system().after("incr_maturity"))
        );
    }
}

fn score(
    plant_query: Query<(&Plant, &RoundsTillMature)>,
) {
    let mut score = 0;
    for (plant, rounds_till_mature) in plant_query.iter() {
        if rounds_till_mature.0 == 0 {
            score += plant.0;
        }
    }
    println!("Round score: {}", score);
}
