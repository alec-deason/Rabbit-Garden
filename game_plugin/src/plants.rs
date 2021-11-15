use bevy::prelude::*;
use crate::{
    GameState,
    turn_structure::TurnState,
    scoring::PrizePlantScore,
    map::{GameLayer, TilePos}
};

pub struct RoundsTillMature(pub i32);
pub struct Plant(pub u32);
pub struct Health(pub i32);
pub struct PrizePlant;

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(increment_maturity.system().label("incr_maturity"))
                .with_system(fail_out_to_scoring.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::RoundCleanup)
                .with_system(despawn_mature_plants.system())
        );
    }
}

fn increment_maturity(
    mut plant_query: Query<&mut RoundsTillMature, With<Plant>>,
) {
    for mut rounds_till_mature in plant_query.iter_mut() {
        rounds_till_mature.0 -= 1;
    }
}

fn fail_out_to_scoring(
    mut state: ResMut<State<GameState>>,
    plant_query: Query<&PrizePlant>,
) {
    if plant_query.iter().count() <= 0 {
        state.set(GameState::PrizePlantScoring);
    }
}

fn despawn_mature_plants(
    mut commands: Commands,
    plant_query: Query<(Entity, &RoundsTillMature, Option<&PrizePlant>, &Health), With<Plant>>,
    mut state: ResMut<State<GameState>>,
    mut prize_plant_score: ResMut<PrizePlantScore>,
) {
    for (e, rounds_till_mature, maybe_prize_plant, health) in plant_query.iter() {
        if rounds_till_mature.0 <= 0 {
            commands.entity(e).despawn_recursive();
            if maybe_prize_plant.is_some() {
                prize_plant_score.0 = health.0 as u32;
                state.set(GameState::PrizePlantScoring);
            }
        }
    }
}
