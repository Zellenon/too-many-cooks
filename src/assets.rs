use bevy::prelude::{Handle, Image, Plugin, Resource};
use bevy::sprite::{Anchor, Sprite};
use bevy_asset_loader::prelude::*;
use bevy_composable::tree::ComponentTree;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::CT;
use bevy_twin_stick::transform2d_mods::Sprite2dBundle;

use crate::appstate::AppState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(AppState::LoadingAssets);
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "img/level-sheet.png")]
    pub level_sheet: Handle<Image>,
    #[asset(path = "img/temporarilous.png")]
    pub the_transient: Handle<Image>,
}

pub fn add_texture(tex: &Handle<Image>) -> ComponentTree {
    let tex = tex.clone();
    CT!(Sprite2dBundle {
        sprite: Sprite {
            anchor: Anchor::Center,
            ..Default::default()
        },
        texture: tex.clone(),
        ..Default::default()
    })
}
