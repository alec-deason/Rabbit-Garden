use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TilemapPlugin);
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_map.system())
                .with_system(spawn_camera.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let material_handle = materials.add(textures.texture_tiles.clone().into());
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);
    let (mut layer_builder, _) = LayerBuilder::new(
            &mut commands,
            LayerSettings::new(
                MapSize(2, 2),
                ChunkSize(8, 8),
                TileSize(16.0, 16.0),
                TextureSize(96.0, 16.0),
            ),
            0u16,
            0u16,
            None,
        );
    layer_builder.set_all(TileBundle::default());
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}
