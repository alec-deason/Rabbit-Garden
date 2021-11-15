use bevy::prelude::*;
use crate::{
    turn_structure::TurnState,
    map::{GameLayer, TilePos}
};

pub struct RoundsTillMature(pub i32);
pub struct Plant(pub u32);

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::EndOfRound)
                .with_system(increment_maturity.system().label("incr_maturity"))
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

fn despawn_mature_plants(
    mut commands: Commands,
    plant_query: Query<(Entity, &RoundsTillMature), With<Plant>>,
) {
    for (e, rounds_till_mature) in plant_query.iter() {
        if rounds_till_mature.0 <= 0 {
            commands.entity(e).despawn_recursive();
        }
    }
}
