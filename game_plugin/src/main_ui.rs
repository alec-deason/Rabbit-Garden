use bevy::{
    prelude::*,
    asset::HandleId,
    input::{
        ElementState,
        mouse::MouseButtonInput
    }
};
use rand::prelude::*;

use bevy_ecs_tilemap::prelude::*;
use crate::{
    GameState,
    map::{MAP_SIZE, TILE_SIZE, GameLayer, Fence},
    plants::{Plant, RoundsTillMature},
    loading::TextureAssets,
    turn_structure::TurnState,
};

pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CursorPosition>();
        app.init_resource::<TileQueue>();
        app.init_resource::<PendingPlacement>();
        app.add_system(track_cursor.system());
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_queue.system())
                .with_system(spawn_overlay.system())
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(cleanup_queue.system())
                .with_system(despawn_overlay.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(quit_to_menu.system())
                .with_system(track_click_events.system())
        );
        app.add_system_set(
            SystemSet::on_update(TurnState::PlayerTurn)
                .with_system(place_tile.system())
        );
        app.add_system_set(
            SystemSet::on_enter(TurnState::PlayerTurn)
                .with_system(arrange_placables.system())
        );
        app.add_system_set(
            SystemSet::on_exit(TurnState::PlayerTurn)
                .with_system(arrange_placables.system())
        );
    }
}

#[derive(Copy, Clone, Debug)]
enum PlacableTile {
    Radish,
    Carrot,
    Pumpkin,
    Fence,
}

impl PlacableTile {
    fn spawn_random<R: Rng>(commands: &mut Commands, textures: &TextureAssets, materials: &mut Assets<ColorMaterial>, rng: &mut R) -> Entity {
        let pt = *[
            PlacableTile::Radish,
            PlacableTile::Carrot,
            PlacableTile::Pumpkin,
            PlacableTile::Fence,
        ].choose(rng).unwrap();
        let e = commands.spawn_bundle(SpriteBundle {
            material: materials.add(pt.texture_handle(&textures).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        }).insert(pt).id();
        e
    }

    fn texture_handle(&self, assets: &TextureAssets) -> Handle<Texture> {
        match self {
            PlacableTile::Radish=> assets.radish.clone(),
            PlacableTile::Carrot => assets.carrot.clone(),
            PlacableTile::Pumpkin => assets.pumpkin.clone(),
            PlacableTile::Fence => assets.fence.clone(),
        }
    }

    fn can_place(&self, pos: bevy_ecs_tilemap::TilePos, map_query: &MapQuery) -> bool {
        match self {
            PlacableTile::Radish |
            PlacableTile::Carrot |
            PlacableTile::Pumpkin => {
                map_query.get_tile_entity(
                    pos,
                    0u16,
                    GameLayer::Fences
                ).is_err() &&
                map_query.get_tile_entity(
                    pos,
                    0u16,
                    GameLayer::Plants
                ).is_err()
            }
            PlacableTile::Fence => {
                map_query.get_tile_entity(
                    pos,
                    0u16,
                    GameLayer::Plants
                ).is_err()
            }
        }
    }

    fn place_on_map(&self, commands: &mut Commands, pos: bevy_ecs_tilemap::TilePos, map_query: &mut MapQuery) {
            if let PlacableTile::Fence = self {
                if map_query.get_tile_entity(
                    pos,
                    0u16,
                    GameLayer::Fences
                ).is_ok() {
                    map_query.despawn_tile(
                        commands,
                        pos,
                        0u16,
                        GameLayer::Fences
                    );
                    map_query.notify_chunk_for_tile(pos, 0u16, GameLayer::Fences);
                    return
                }
            }

            let e = map_query.set_tile(
                commands,
                pos,
                Tile {
                    texture_index: match self {
                        PlacableTile::Radish => 4,
                        PlacableTile::Carrot => 3,
                        PlacableTile::Pumpkin => 2,
                        PlacableTile::Fence => 0,
                    },
                    ..Default::default()
                },
                0u16,
                match self {
                    PlacableTile::Radish |
                    PlacableTile::Carrot |
                    PlacableTile::Pumpkin => GameLayer::Plants,
                    PlacableTile::Fence => GameLayer::Fences,
                }
            ).unwrap();
            match self {
                PlacableTile::Radish=> {
                    commands.entity(e).insert(Plant(1));
                    commands.entity(e).insert(RoundsTillMature(1));
                    map_query.notify_chunk_for_tile(pos, 0u16, GameLayer::Plants);
                }
                PlacableTile::Carrot => {
                    commands.entity(e).insert(Plant(3));
                    commands.entity(e).insert(RoundsTillMature(2));
                    map_query.notify_chunk_for_tile(pos, 0u16, GameLayer::Plants);
                }
                PlacableTile::Pumpkin => {
                    commands.entity(e).insert(Plant(9));
                    commands.entity(e).insert(RoundsTillMature(4));
                    map_query.notify_chunk_for_tile(pos, 0u16, GameLayer::Plants);
                }
                PlacableTile::Fence => {
                    commands.entity(e).insert(Fence);
                    map_query.notify_chunk_for_tile(pos, 0u16, GameLayer::Fences);
                }
            }
    }
}

#[derive(Default)]
struct TileQueue(Vec<Entity>);

fn setup_queue(
    mut commands: Commands,
    mut queue: ResMut<TileQueue>,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = thread_rng();
    queue.0.clear();
    for _ in 0..5 {
        let e = PlacableTile::spawn_random(&mut commands, &textures, &mut materials, &mut rng);
        queue.0.push(e);
    }
}

fn cleanup_queue(
    mut commands: Commands,
    mut queue: ResMut<TileQueue>,
    query: Query<Entity, With<PlacableTile>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
    queue.0.clear();
}

fn arrange_placables(
    queue: Res<TileQueue>,
    mut query: Query<&mut Transform, With<PlacableTile>>,
) {
    for (i, e) in queue.0.iter().enumerate() {
        if let Ok(mut t) = query.get_mut(*e) {
            t.translation.x = i as f32 * TILE_SIZE as f32 * 2.0 - (TILE_SIZE as f32 * MAP_SIZE as f32)/2.0 + TILE_SIZE as f32;
            t.translation.y = (TILE_SIZE as f32 * MAP_SIZE as f32)/2.0;
        }
    }
}

struct GameOverlay;
fn spawn_overlay(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.overlay.clone().into()),
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..Default::default()
    }).insert(GameOverlay);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.underlay.clone().into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(GameOverlay);
}

fn despawn_overlay(
    mut commands: Commands,
    query: Query<Entity, With<GameOverlay>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn quit_to_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Menu);
    }
}

fn track_cursor(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    for event in cursor_moved_events.iter() {
        cursor_position.0 = event.position - window_size/2.0;
    }
}

#[derive(Default)]
struct PendingPlacement(Option<Vec2>);
fn track_click_events(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    cursor_position: Res<CursorPosition>,
    mut pending_placement: ResMut<PendingPlacement>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            pending_placement.0.replace(cursor_position.0);
        }
    }
}

#[derive(Default)]
struct CursorPosition(Vec2);
fn place_tile(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_transform_query: Query<&Transform, With<Map>>,
    mut queue: ResMut<TileQueue>,
    mut map_query: MapQuery,
    placables_query: Query<&PlacableTile>,
    mut state: ResMut<State<TurnState>>,
    mut pending_placement: ResMut<PendingPlacement>,
) {
    if let Some(click_pos) = pending_placement.0.take() {
        if let Ok(t) = map_transform_query.single() {
            let tile = ((click_pos - t.translation.truncate()) / TILE_SIZE as f32).floor();
            if tile.x > 1.0 && tile.x < MAP_SIZE as f32-2.0 && tile.y > 1.0 && tile.y < MAP_SIZE as f32-2.0 {
                if let Ok(placable_tile) = placables_query.get(queue.0[0]) {
                    let pos = bevy_ecs_tilemap::TilePos(tile.x as u32, tile.y as u32);
                    if placable_tile.can_place(pos, &map_query) {
                        let placable_entity = queue.0.remove(0);
                        let e = PlacableTile::spawn_random(&mut commands, &textures, &mut materials, &mut thread_rng());
                        queue.0.push(e);
                        placable_tile.place_on_map(&mut commands, pos, &mut map_query);
                        commands.entity(placable_entity).despawn_recursive();
                        state.set(TurnState::PestTurnA);
                    }
                }
            }
        }
    }
}
