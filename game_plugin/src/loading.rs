use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/tiles.png")]
    pub texture_tiles: Handle<Texture>,
    #[asset(path = "textures/pumpkin.png")]
    pub pumpkin: Handle<Texture>,
    #[asset(path = "textures/carrot.png")]
    pub carrot: Handle<Texture>,
    #[asset(path = "textures/fence.png")]
    pub fence: Handle<Texture>,
    #[asset(path = "textures/fence_tiles.png")]
    pub texture_fence_tiles: Handle<Texture>,
}
