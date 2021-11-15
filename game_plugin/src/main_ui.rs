use bevy::{
    prelude::*,
    asset::HandleId,
    input::{
        ElementState,
        mouse::MouseButtonInput
    }
};
use rand::prelude::*;
use anyhow::Result;

use crate::{
    GameState,
    map::{MAP_SIZE, TILE_SIZE, GameLayer, Fence, TilePos},
    plants::{Plant, RoundsTillMature, Health},
    pests::Pest,
    loading::TextureAssets,
    turn_structure::TurnState,
};

pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CursorPosition>();
        app.init_resource::<TileQueue>();
        app.init_resource::<FirstClickSupressor>();
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
                .with_system(despawn_tiles.system())
                .with_system(reset_first_click_supressor.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(quit_to_menu.system())
                .with_system(track_click_events.system())
        );
        app.add_system_set(
            SystemSet::on_update(TurnState::PlayerTurn)
                .with_system(place_tile.system().chain(spawn_tile_sprites.system()))
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

    fn can_place(&self, pos: TilePos) -> bool {
        /*
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
        */
        true
    }

    fn place_on_map(&self, commands: &mut Commands, pos: TilePos) -> (Entity, String) {
        let mut e = commands.spawn();
        let (spawner, sprite) = match self {
            PlacableTile::Radish => {
                (
                e.insert(GameLayer::Plants)
                 .insert(Plant(1))
                 .insert(Health(1))
                 .insert(RoundsTillMature(1)),
                 "radish".to_string()
                )
            }
            PlacableTile::Carrot => {
                (
                e.insert(GameLayer::Plants)
                 .insert(Plant(2))
                 .insert(Health(1))
                 .insert(RoundsTillMature(2)),
                "carrot".to_string()
                )
            }
            PlacableTile::Pumpkin => {
                (
                e.insert(GameLayer::Plants)
                 .insert(Plant(6))
                 .insert(Health(1))
                 .insert(RoundsTillMature(4)),
                "pumpkin".to_string()
                )
            }
            PlacableTile::Fence => {
                (
                e.insert(GameLayer::Fences)
                 .insert(Health(1))
                 .insert(Fence),
                 "fence".to_string()
                )
            }
        };
        (spawner.insert(pos).id(), sprite)
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
            t.translation.x = i as f32 * TILE_SIZE as f32 * 2.61 - (TILE_SIZE as f32 * MAP_SIZE as f32)/2.0 + TILE_SIZE as f32;
            t.translation.y = (TILE_SIZE as f32 * MAP_SIZE as f32)/2.0;
        }
    }
}

pub struct GameOverlay;
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

pub fn despawn_overlay(
    mut commands: Commands,
    query: Query<Entity, With<GameOverlay>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn reset_first_click_supressor(
    mut first_click_supressor: ResMut<FirstClickSupressor>,
) {
    first_click_supressor.0 = false;
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
#[derive(Default)]
struct FirstClickSupressor(bool);
fn track_click_events(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    cursor_position: Res<CursorPosition>,
    mut pending_placement: ResMut<PendingPlacement>,
    mut first_click_supressor: ResMut<FirstClickSupressor>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Released {
            if !first_click_supressor.0 {
                first_click_supressor.0 = true;
                continue
            }
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
    mut queue: ResMut<TileQueue>,
    placables_query: Query<&PlacableTile>,
    mut state: ResMut<State<TurnState>>,
    mut pending_placement: ResMut<PendingPlacement>,
    collision_query: Query<&TilePos>,
) -> Result<Vec<(Entity, String)>> {
    let mut to_spawn = vec![];
    if let Some(click_pos) = pending_placement.0.take() {
        let mut tile = (click_pos / TILE_SIZE as f32 + Vec2::splat(MAP_SIZE as f32 / 2.0)).floor();
        tile.y += 1.0;
        if tile.x > 1.0 && tile.x < MAP_SIZE as f32-2.0 && tile.y > 1.0 && tile.y < MAP_SIZE as f32-2.0 {
            if let Ok(placable_tile) = placables_query.get(queue.0[0]) {
                let pos = TilePos(IVec2::new(tile.x as i32, tile.y as i32));
                if !collision_query.iter().any(|other| other == &pos) {
                    let placable_entity = queue.0.remove(0);
                    let e = PlacableTile::spawn_random(&mut commands, &textures, &mut materials, &mut thread_rng());
                    queue.0.push(e);
                    to_spawn.push(placable_tile.place_on_map(&mut commands, pos));
                    commands.entity(placable_entity).despawn_recursive();
                    state.set(TurnState::PestTurnA);
                }
            }
        }
    }
    Ok(to_spawn)
}

struct DesiredSprite(String);
pub fn spawn_tile_sprites(
    In(to_spawn): In<Result<Vec<(Entity, String)>>>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(to_spawn) = to_spawn {
        for (e, desired_sprite) in &to_spawn {
            let handle = match desired_sprite.as_str() {
                "radish" => textures.radish.clone(),
                "carrot" => textures.carrot.clone(),
                "pumpkin" => textures.pumpkin.clone(),
                "big_pumpkin1" => textures.big_pumpkin1.clone(),
                "big_pumpkin2" => textures.big_pumpkin2.clone(),
                "big_pumpkin3" => textures.big_pumpkin3.clone(),
                "big_pumpkin4" => textures.big_pumpkin4.clone(),
                "fence" => textures.fence_tiles.clone(),
                _ => unimplemented!()
            };
            if desired_sprite == "fence" {
                let texture_atlas = TextureAtlas::from_grid(handle, Vec2::new(86.0, 86.0), 4, 4);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                commands.entity(*e).insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..Default::default()
                })
                .remove::<DesiredSprite>();
            } else {
                commands.entity(*e).insert_bundle(SpriteBundle {
                    material: materials.add(handle.into()),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..Default::default()
                })
                .remove::<DesiredSprite>();
            }
        }
    }
}


fn despawn_tiles(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Plant>, With<Fence>, With<Pest>)>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
