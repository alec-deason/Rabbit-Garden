use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(ui_example.system())
        );
    }
}

fn ui_example(
    egui_context: Res<EguiContext>,
    mut state: ResMut<State<GameState>>,
) {
        egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
            ui.centered_and_justified(|ui| {
                if ui.button("Play").clicked() {
                    state.set(GameState::Playing);
                }
            })
        });
}
