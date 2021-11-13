use bevy::prelude::*;
use crate::{
    turn_structure::TurnState,
    map::Plant,
};

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(score.system())
        );
    }
}

fn score(
    plant_query: Query<&Plant>,
) {
    println!("Round score: {}", plant_query.iter().count());
}
