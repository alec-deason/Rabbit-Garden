use std::collections::HashMap;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::{
    turn_structure::TurnState,
    map::{GameLayer, MAP_SIZE},
};
use rand::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Rabbit {
    move_idx: usize,
    ticks_since_move: usize,
}
pub struct IdlePest;

pub struct PestPlugin;

impl Plugin for PestPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_exit(TurnState::RoundSetup)
                .with_system(move_idle_pests_in.system().label("move idle"))
                .with_system(spawn_rabbits.system().after("move idle"))
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::RoundCleanup)
                .with_system(despawn_rabbits.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::PestTurnA)
                .with_system(rabbit_movement.system())
                .with_system(maybe_end_pest_turn.system())
        );
    }
}

fn rabbit_movement(
    mut commands: Commands,
    mut rabbit_query: Query<(Entity, &mut Rabbit, &mut TilePos), Without<IdlePest>>,
    mut map_query: MapQuery,
) {
    let mut current_positions = HashMap::with_capacity(10);
    for (e, _, pos) in rabbit_query.iter_mut() {
        current_positions.insert(*pos, e);
    }
    for (e, mut rabbit, mut pos) in rabbit_query.iter_mut() {
        let movement = [
            IVec2::new(-1, 0),
            IVec2::new(-1, 0),
            IVec2::new(0, -1),
        ][rabbit.move_idx];
        rabbit.move_idx = (rabbit.move_idx + 1) % 3;
        let new_pos = IVec2::new(pos.0 as i32, pos.1 as i32) + movement;
        if new_pos.x >= 0 && new_pos.y >= 0 && new_pos.x < MAP_SIZE as i32 && new_pos.y < MAP_SIZE as i32 {
            let new_pos = TilePos(new_pos.x as u32, new_pos.y as u32);
            let mut did_move = false;
            if map_query.get_tile_entity(
                new_pos,
                0u16,
                GameLayer::Fences
            ).is_err() {
                if !current_positions.contains_key(&new_pos) {
                    current_positions.remove(&*pos);
                    map_query.despawn_tile(
                        &mut commands,
                        *pos,
                        0u16,
                        GameLayer::Pests
                    );
                    map_query.notify_chunk_for_tile(*pos, 0u16, GameLayer::Pests);
                    // FIXME: I thought moving tiles was clever but that didn't actually work
                    // with bevy_ecs_tilemap so now I'm despawning and spawning them.
                    // Better to rewrite this to use non-tile sprites for moving stuff I think.
                    let e = map_query.set_tile(
                        &mut commands,
                        new_pos,
                        Tile {
                            texture_index: 1,
                            ..Default::default()
                        },
                        0u16,
                        GameLayer::Pests
                    ).unwrap();
                    let mut rabbit = rabbit.clone();
                    rabbit.ticks_since_move = 0;
                    did_move = true;
                    commands.entity(e).insert(rabbit);
                    map_query.notify_chunk_for_tile(new_pos, 0u16, GameLayer::Pests);
                    current_positions.insert(new_pos, e);

                    map_query.despawn_tile(
                        &mut commands,
                        new_pos,
                        0u16,
                        GameLayer::Plants
                    );
                    map_query.notify_chunk_for_tile(new_pos, 0u16, GameLayer::Plants);
                }
            }
            if !did_move {
                rabbit.ticks_since_move += 1;
            }
        } else {
            map_query.despawn_tile(
                &mut commands,
                *pos,
                0u16,
                GameLayer::Pests
            );
            map_query.notify_chunk_for_tile(*pos, 0u16, GameLayer::Pests);
        }
    }
}

fn maybe_end_pest_turn(
    mut state: ResMut<State<TurnState>>,
    mut rabbit_query: Query<&Rabbit, Without<IdlePest>>,
) {
    for rabbit in rabbit_query.iter() {
        if rabbit.ticks_since_move < 3 {
            return
        }
    }
    // If we've reached this point then every rabbit on the board
    // has been unable to move for a full cycle of their motion
    // pattern which means they are all permanently blocked and
    // the round can end.
    state.set(TurnState::EndOfRound);
}

fn move_idle_pests_in(
    mut commands: Commands,
    mut rabbit_query: Query<(&Rabbit, &TilePos), With<IdlePest>>,
    mut map_query: MapQuery,
) {
    for (rabbit, pos) in rabbit_query.iter() {
        map_query.despawn_tile(
            &mut commands,
            *pos,
            0u16,
            GameLayer::Pests
        );
        map_query.notify_chunk_for_tile(*pos, 0u16, GameLayer::Pests);
        let mut new_pos = pos.clone();
        new_pos.0 -= 1;
        let e = map_query.set_tile(
            &mut commands,
            new_pos,
            Tile {
                texture_index: 1,
                ..Default::default()
            },
            0u16,
            GameLayer::Pests
        ).unwrap();
        commands.entity(e).insert(rabbit.clone());
        map_query.notify_chunk_for_tile(new_pos, 0u16, GameLayer::Pests);
    }
}

fn spawn_rabbits(
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    let mut rng = thread_rng();
    let spawn_slots:Vec<_> = (2..MAP_SIZE-2).collect();
    let count = rng.gen_range(1..4);
    for y in spawn_slots.choose_multiple(&mut rng, count) {
        let mut position = TilePos(MAP_SIZE-1,*y);
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
        commands.entity(e.unwrap()).insert(Rabbit::default());
        commands.entity(e.unwrap()).insert(IdlePest);
        map_query.notify_chunk_for_tile(position, 0u16, GameLayer::Pests);
    }
}

fn despawn_rabbits(
    mut commands: Commands,
    mut rabbit_query: Query<&TilePos, (With<Rabbit>, Without<IdlePest>)>,
    mut map_query: MapQuery,
) {
    for pos in rabbit_query.iter() {
        map_query.despawn_tile(
            &mut commands,
            *pos,
            0u16,
            GameLayer::Pests
        );
        map_query.notify_chunk_for_tile(*pos, 0u16, GameLayer::Pests);
    }
}
