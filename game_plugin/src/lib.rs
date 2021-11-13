mod loading;
mod menu;
mod map;
mod pests;
mod plants;
mod turn_structure;
mod main_ui;
mod scoring;

use crate::{
    loading::LoadingPlugin,
    menu::MenuPlugin,
    map::MapPlugin,
    pests::PestPlugin,
    plants::PlantPlugin,
    turn_structure::TurnPlugin,
    main_ui::MainUiPlugin,
    scoring::ScoringPlugin,
};

use game_music::MusicPlugin;

use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(MainUiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PestPlugin)
            .add_plugin(PlantPlugin)
            .add_plugin(TurnPlugin)
            .add_plugin(MusicPlugin)
            .add_plugin(ScoringPlugin)
            .add_plugin(MapPlugin);

    }
}
