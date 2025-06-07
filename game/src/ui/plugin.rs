use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::{ConfigureLoadingState, LoadingStateConfig}, LoadingStateAppExt}};

use crate::{core::states::AppLoadingAssetsSubState, ui::components::{hack_button::ui_hack_button_hover, ui_submit_button::ui_submit_button_hover}};




pub struct UiSystemPlugin;

impl Plugin for UiSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_loading_state(
                LoadingStateConfig::new(AppLoadingAssetsSubState::Loading)
                    .load_collection::<UiAssetCollection>(),
            )
            .add_systems(Update, (ui_hack_button_hover, ui_submit_button_hover))
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiAssetCollection {
    #[asset(path = "ui/hack_button.png")]
    pub button: Handle<Image>,
    #[asset(path = "ui/main_container.png")]
    pub main_container: Handle<Image>,
    #[asset(path = "ui/sub_container.png")]
    pub sub_container: Handle<Image>,
}