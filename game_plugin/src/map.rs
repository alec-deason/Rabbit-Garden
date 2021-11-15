use std::collections::HashSet;
use crate::{
    loading::TextureAssets,
    plants::{Plant, RoundsTillMature, Health, PrizePlant},
    turn_structure::TurnState,
    scoring::PrizePlantScore,
    main_ui::spawn_tile_sprites,
    GameState,
};
use bevy::prelude::*;
use anyhow::Result;

use rand::prelude::*;

pub struct MapPlugin;

pub const MAP_SIZE: i32 = 12;
pub const TILE_SIZE: i32 = 62;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TilePos(pub IVec2);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system());
        app.add_system_to_stage(CoreStage::PostUpdate, update_fence_autotile.system())
           .add_system_to_stage(CoreStage::PostUpdate, update_tile_position.system());

        app.add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(spawn_initial_map.system().chain(spawn_tile_sprites.system()))
        );

    }
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum GameLayer {
    Fences,
    Plants,
    Pests,
}

pub struct Blocking;

pub struct Fence;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


fn update_fence_autotile(
    mut tile_query: Query<(&mut TextureAtlasSprite, &TilePos), With<Fence>>,
) {
    //TODO: Use and event instead of naive check on each frame

    let mut current_positions = HashSet::with_capacity(10);
    for (_, pos) in tile_query.iter_mut() {
        current_positions.insert(pos.0);
    }
    for (mut sprite, pos) in tile_query.iter_mut() {
        sprite.index = 0;
        for ((dx, dy), constant) in &[((0, 1), 1), ((0, -1), 8), ((-1, 0), 2), ((1, 0), 4)] {
            let other = IVec2::new(pos.0.x+dx, pos.0.y+dy);
            if current_positions.contains(&other) {
                sprite.index += constant;
            }
        }
    }
}

fn update_tile_position(
    mut query: Query<(&mut Transform, &TilePos)>,
) {
    for (mut t, p) in query.iter_mut() {
        t.translation.x = p.0.x as f32 * TILE_SIZE as f32 - (MAP_SIZE as f32 * TILE_SIZE as f32) / 2.0 + TILE_SIZE as f32 /2.0;
        t.translation.y = p.0.y as f32 * TILE_SIZE as f32 - (MAP_SIZE as f32 * TILE_SIZE as f32) / 2.0 - TILE_SIZE as f32 /2.0;
    }
}

fn spawn_initial_map(
    mut commands: Commands,
    mut prize_plant_score: ResMut<PrizePlantScore>,
) -> Result<Vec<(Entity, String)>> {
    let mut fences = vec![];
    for i in 4..8 {
        fences.push(IVec2::new(i, 4));
        fences.push(IVec2::new(i, 7));
        fences.push(IVec2::new(4, i));
        fences.push(IVec2::new(7, i));
    }
    let mut to_spawn = Vec::with_capacity(fences.len());
    for pos in fences {
        let e = commands.spawn().insert(GameLayer::Fences)
         .insert(Fence)
         .insert(TilePos(pos))
         .insert(Health(1))
         .id();
        to_spawn.push((e, "fence".to_string()))
    }
    prize_plant_score.0 = 0;
    let e = commands.spawn().insert(GameLayer::Plants)
     .insert(Plant(60))
     .insert(RoundsTillMature(10))
     .insert(Health(10))
     .insert(PrizePlant)
     .insert(TilePos(IVec2::new(5,5)))
     .id();
    to_spawn.push((e, "big_pumpkin1".to_string()));
    let e = commands.spawn().insert(GameLayer::Plants)
     .insert(Plant(60))
     .insert(RoundsTillMature(10))
     .insert(Health(10))
     .insert(PrizePlant)
     .insert(TilePos(IVec2::new(6,5)))
     .id();
    to_spawn.push((e, "big_pumpkin2".to_string()));
    let e = commands.spawn().insert(GameLayer::Plants)
     .insert(Plant(60))
     .insert(RoundsTillMature(10))
     .insert(Health(10))
     .insert(PrizePlant)
     .insert(TilePos(IVec2::new(6,6)))
     .id();
    to_spawn.push((e, "big_pumpkin3".to_string()));
    let e = commands.spawn().insert(GameLayer::Plants)
     .insert(Plant(60))
     .insert(RoundsTillMature(10))
     .insert(Health(10))
     .insert(PrizePlant)
     .insert(TilePos(IVec2::new(5,6)))
     .id();
    to_spawn.push((e, "big_pumpkin4".to_string()));

    Ok(to_spawn)
}
