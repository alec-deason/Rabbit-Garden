use crate::{
    loading::TextureAssets,
    turn_structure::TurnState,
    GameState,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use rand::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin);
        app.add_system_set(
            SystemSet::on_enter(TurnState::RoundSetup)
                .with_system(spawn_map.system())
                .with_system(spawn_camera.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::RoundCleanup)
                .with_system(despawn_map.system())
        );
        app.add_system_set(
            SystemSet::on_exit(TurnState::RoundSetup)
                .with_system(spawn_random_fences.system())
                .with_system(spawn_random_plants.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_fence_autotile.system())
        );
    }
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum GameLayer {
    Dirt,
    Fences,
    Plants,
    Pests,
}

impl Into<u16> for GameLayer {
    fn into(self) -> u16 {
        self as u16
    }
}

impl bevy_ecs_tilemap::prelude::LayerId for GameLayer {}

pub struct Fence;
pub struct Plant;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let base_material_handle = materials.add(textures.texture_tiles.clone().into());
    let fence_material_handle = materials.add(textures.texture_fence_tiles.clone().into());

    // Create dirt layer
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    let mut rng = thread_rng();
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(64.0, 64.0),
            TextureSize(192.0, 128.0),
        ),
        0u16,
        GameLayer::Dirt,
        None,
    );
    for x in 0..16 {
        for y in 0..16 {
            let position = TilePos(x,y);
            let _ = layer_builder.set_tile(
                position,
                TileBundle {
                    tile: Tile {
                        texture_index: rng.gen_range(4..6),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
       }
    }
    let base_layer_entity = map_query.build_layer(&mut commands, layer_builder, base_material_handle.clone());
    map.add_layer(&mut commands, GameLayer::Dirt, base_layer_entity);

    // Create Pest layer

    let (layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(64.0, 64.0),
            TextureSize(192.0, 128.0),
        ),
        0u16,
        GameLayer::Pests,
        None,
    );
    let pest_layer_entity = map_query.build_layer(&mut commands, layer_builder, base_material_handle.clone());

    map.add_layer(&mut commands, GameLayer::Pests, pest_layer_entity);


    // Create fence layer

    let (layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(64.0, 64.0),
            TextureSize(256.0, 256.0),
        ),
        0u16,
        GameLayer::Fences,
        None,
    );
    let fence_layer_entity = map_query.build_layer(&mut commands, layer_builder, fence_material_handle);
    map.add_layer(&mut commands, GameLayer::Fences, fence_layer_entity);

    // Create plant layer

    let (layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(64.0, 64.0),
            TextureSize(192.0, 128.0),
        ),
        0u16,
        GameLayer::Plants,
        None,
    );
    let plant_layer_entity = map_query.build_layer(&mut commands, layer_builder, base_material_handle);
    map.add_layer(&mut commands, GameLayer::Plants, plant_layer_entity);
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-8.0*64.0, -8.0*64.0, 0.0))
        .insert(GlobalTransform::default());
}

fn despawn_map(
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    map_query.despawn(&mut commands, 0u16);
}

fn update_fence_autotile(
    mut tile_query: Query<(&mut Tile, &TilePos), With<Fence>>,
    mut map_query: MapQuery,
) {
    //TODO: Change detection instead of naive check on each frame
    for (mut tile, pos) in tile_query.iter_mut() {
        let mut idx = 0;
        for (maybe_tile, constant) in map_query.get_tile_neighbors(*pos, 0u16, GameLayer::Fences).iter().zip(&[1, 8, 2, 4]) {
            if maybe_tile.is_ok() {
                idx += constant;
            }
        }
        tile.texture_index = idx;
        map_query.notify_chunk_for_tile(*pos, 0u16, GameLayer::Fences);
    }
}

fn spawn_random_fences(
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    let mut rng = thread_rng();
    for x in 0..14 {
        for y in 0..14 {
            if rng.gen::<f32>() < 0.33 {
                let position = TilePos(x,y);
                let e = map_query.set_tile(
                    &mut commands,
                    position,
                    Tile::default(),
                    0u16,
                    GameLayer::Fences,
                );
                commands.entity(e.unwrap()).insert(Fence);
            }
       }
    }
}

fn spawn_random_plants(
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    let mut rng = thread_rng();
    for x in 0..14 {
        for y in 0..15 {
            if rng.gen::<f32>() < 0.1 {
                let position = TilePos(x,y);
                if map_query.get_tile_entity(
                    position,
                    0u16,
                    GameLayer::Fences
                ).is_err() {
                    let e = map_query.set_tile(
                        &mut commands,
                        position,
                        Tile {
                            texture_index: rng.gen_range(2..4),
                            ..Default::default()
                        },
                        0u16,
                        GameLayer::Plants,
                    );
                    commands.entity(e.unwrap()).insert(Plant);
                    map_query.notify_chunk_for_tile(position, 0u16, GameLayer::Plants);
                }
            }
       }
    }
}
