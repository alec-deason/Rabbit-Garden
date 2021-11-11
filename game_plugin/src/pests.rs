use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::{
    turn_structure::TurnState,
    map::GameLayer,
};
use rand::prelude::*;

pub struct Rabbit;

pub struct PestPlugin;

impl Plugin for PestPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_exit(TurnState::RoundSetup)
                .with_system(spawn_rabbits.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::PestTurn)
                .with_system(rabbit_movement.system())
        );
    }
}

fn rabbit_movement(
    mut rabbit_query: Query<&mut TilePos, With<Rabbit>>,
    mut map_query: MapQuery,
) {
    for mut pos in rabbit_query.iter_mut() {
        if pos.0 > 0 {
            let mut new_pos = pos.clone();
            new_pos.0 -= 1;
            if map_query.get_tile_entity(
                new_pos,
                0u16,
                GameLayer::Fences
            ).is_err() {
                *pos = new_pos;
            }
            map_query.notify_chunk_for_tile(new_pos, 0u16, GameLayer::Pests);
        }
    }
}

fn spawn_rabbits(
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    let mut rng = thread_rng();
    for _ in 0..4 {
        let mut position = TilePos(15,rng.gen_range(0..15));
        while map_query.get_tile_entity(
            position,
            0u16,
            GameLayer::Fences
        ).is_ok() {
            position = TilePos(15,rng.gen_range(0..15));
        }
        let e = map_query.set_tile(
            &mut commands,
            position,
            Tile {
                texture_index: 1,
                ..Default::default()
            },
            0u16,
            GameLayer::Pests,
        );
        commands.entity(e.unwrap()).insert(Rabbit);
        map_query.notify_chunk_for_tile(position, 0u16, GameLayer::Pests);
    }
}
