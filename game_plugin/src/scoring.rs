use bevy::prelude::*;
use crate::{
    GameState,
    turn_structure::TurnState,
    plants::{Plant, RoundsTillMature},
};

pub struct ScoringPlugin;
#[derive(Default)]
pub struct PrizePlantScore(pub u32);

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PrizePlantScore>();
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(score.system().after("incr_maturity"))
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::PrizePlantScoring)
                .with_system(score_prize_plant.system().after("incr_maturity"))
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

fn score_prize_plant(
    score: Res<PrizePlantScore>,
    mut state: ResMut<State<GameState>>,
) {
    println!("Prize Plant Score: {}", score.0);
    state.set(GameState::Menu);
}
