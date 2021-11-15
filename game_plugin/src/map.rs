use std::collections::HashSet;
use crate::{
    loading::TextureAssets,
    turn_structure::TurnState,
    GameState,
};
use bevy::prelude::*;

use rand::prelude::*;

pub struct MapPlugin;

pub const MAP_SIZE: i32 = 9;
pub const TILE_SIZE: i32 = 86;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TilePos(pub IVec2);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_camera.system());
        app.add_system_to_stage(CoreStage::PostUpdate, update_fence_autotile.system())
           .add_system_to_stage(CoreStage::PostUpdate, update_tile_position.system());
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
