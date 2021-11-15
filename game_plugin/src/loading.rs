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
    #[asset(path = "textures/big_pumpkin1.png")]
    pub big_pumpkin1: Handle<Texture>,
    #[asset(path = "textures/big_pumpkin2.png")]
    pub big_pumpkin2: Handle<Texture>,
    #[asset(path = "textures/big_pumpkin3.png")]
    pub big_pumpkin3: Handle<Texture>,
    #[asset(path = "textures/big_pumpkin4.png")]
    pub big_pumpkin4: Handle<Texture>,
    #[asset(path = "textures/wind.png")]
    pub wind: Handle<Texture>,
    #[asset(path = "textures/radish.png")]
    pub radish: Handle<Texture>,
    #[asset(path = "textures/carrot.png")]
    pub carrot: Handle<Texture>,
    #[asset(path = "textures/fence.png")]
    pub fence: Handle<Texture>,
    #[asset(path = "textures/rabbit.png")]
    pub rabbit: Handle<Texture>,
    #[asset(path = "textures/fence_tiles.png")]
    pub fence_tiles: Handle<Texture>,
    #[asset(path = "textures/overlay.png")]
    pub overlay: Handle<Texture>,
    #[asset(path = "textures/underlay.png")]
    pub underlay: Handle<Texture>,

    #[asset(path = "textures/no_prize.png")]
    pub no_prize: Handle<Texture>,
    #[asset(path = "textures/3rd_prize.png")]
    pub third_prize: Handle<Texture>,
    #[asset(path = "textures/2nd_prize.png")]
    pub second_prize: Handle<Texture>,
    #[asset(path = "textures/1st_prize.png")]
    pub first_prize: Handle<Texture>,
}
