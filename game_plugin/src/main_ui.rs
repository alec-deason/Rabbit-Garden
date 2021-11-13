use bevy::{
    prelude::*,
    asset::HandleId,
};

use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_ecs_tilemap::prelude::*;
use crate::{
    GameState,
    map::{MAP_SIZE, TILE_SIZE, GameLayer, Plant, Fence},
    loading::TextureAssets,
};

pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_exit(GameState::Loading)
                .with_system(setup_assets.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(main_ui.system())
        );
    }
}

fn setup_assets(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    textures: Res<TextureAssets>,
) {
    egui_context.set_egui_texture(0, textures.pumpkin.clone());
    commands.spawn().insert(DraggableTile {
        name: "pumpkin".to_string(),
        default_position: egui::Pos2::new(0.0, 0.0),
        needs_reset: true,
        texture_id: 0,
        game_layer: GameLayer::Plants,
        tile_id: 2,
    });
    egui_context.set_egui_texture(1, textures.carrot.clone());
    commands.spawn().insert(DraggableTile {
        name: "carrot".to_string(),
        default_position: egui::Pos2::new(TILE_SIZE as f32, 0.0),
        needs_reset: true,
        texture_id: 1,
        game_layer: GameLayer::Plants,
        tile_id: 3,
    });
    egui_context.set_egui_texture(2, textures.fence.clone());
    commands.spawn().insert(DraggableTile {
        name: "fence".to_string(),
        default_position: egui::Pos2::new(TILE_SIZE as f32*2.0, 0.0),
        needs_reset: true,
        texture_id: 2,
        game_layer: GameLayer::Fences,
        tile_id: 0,
    });
}

struct DraggableTile {
    name: String,
    default_position: egui::Pos2,
    needs_reset: bool,
    texture_id: u64,
    game_layer: GameLayer,
    tile_id: u32,
}

impl DraggableTile {
    fn show(&mut self, ctx: &EguiContext) -> egui::InnerResponse<()> {
        let mut area = egui::Area::new(&self.name);
        if self.needs_reset {
            self.needs_reset = false;
            area = area.current_pos(self.default_position);
        }
        area.show(ctx.ctx(), |ui| {
            ui.image(
                egui::TextureId::User(self.texture_id),
                [23.0, 20.0],
            );
        })
    }
}

fn main_ui(
    mut commands: Commands,
    egui_context: Res<EguiContext>,
    mut map_component_query: Query<&mut Transform, With<Map>>,
    mut map_query: MapQuery,
    wnds: Res<Windows>,
    mut draggables_query: Query<&mut DraggableTile>,
) {

        let draggable_responses:Vec<_> = draggables_query.iter_mut().map(|mut d| d.show(&egui_context).response).collect();

        let rect = egui::SidePanel::left("my_left_panel").resizable(false).show(egui_context.ctx(), |ui| {
            let mut size = ui.available_size();
            size.x = size.x.min(300.0);
            ui.allocate_space(size);
        }).response.rect;
        let wnd = wnds.get_primary().unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let scale = rect.max.y.min(size.x - rect.max.x) / (MAP_SIZE*TILE_SIZE) as f32;
        let map_x = rect.max.x - size.x / 2.0;
        let map_y =((MAP_SIZE*TILE_SIZE) as f32 / -2.0) * scale;

        for mut t in map_component_query.iter_mut() {
            t.translation.x = map_x;
            t.translation.y = map_y;
            t.scale = Vec3::new(scale, scale, 1.0);
        }

        for (mut draggable, response) in draggables_query.iter_mut().zip(draggable_responses) {
            if response.drag_released() {
                draggable.needs_reset = true;
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let pointer_pos = egui::Pos2::new(pointer_pos.x - size.x/2.0, pointer_pos.y - size.y/2.0);
                    let map_pos = pointer_pos - egui::Pos2::new(map_x, map_y);
                    let mut tile_pos = map_pos / egui::Vec2::new(TILE_SIZE as f32 * scale, TILE_SIZE as f32 * scale);
                    tile_pos.y = MAP_SIZE as f32 - tile_pos.y;
                    if tile_pos.x >= 0.0 && tile_pos.y >= 0.0 && tile_pos.x < MAP_SIZE as f32 && tile_pos.y < MAP_SIZE as f32 {
                        let tile_pos = TilePos(tile_pos.x as u32, tile_pos.y as u32);
                        let e = map_query.set_tile(
                            &mut commands,
                            tile_pos,
                            Tile {
                                texture_index: draggable.tile_id as u16,
                                ..Default::default()
                            },
                            0u16,
                            draggable.game_layer,
                        );
                        map_query.notify_chunk_for_tile(tile_pos, 0u16, draggable.game_layer);
                        match draggable.game_layer {
                            GameLayer::Plants => { commands.entity(e.unwrap()).insert(Plant); },
                            GameLayer::Fences => { commands.entity(e.unwrap()).insert(Fence); },
                            _ => ()
                        }
                    }
                }
            }
        }

}
