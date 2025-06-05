use bevy::prelude::*;

use crate::ui::components::hack_button::ui_hack_button_hover;




pub struct UiSystemPlugin;

impl Plugin for UiSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ui_hack_button_hover)
        ;
    }
}