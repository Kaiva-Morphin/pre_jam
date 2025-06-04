use bevy::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

use core::plugin::CorePlugin;

use crate::physics::player::PlayerPlugin;
use crate::tilemap::plugin::MapPlugin;
use crate::utils::background::StarBackgroundPlugin;

mod core;
mod ui;
mod camera;
mod utils;
mod physics;
mod interactions;
mod tilemap;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            DebugOverlayPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            StarBackgroundPlugin,
            MapPlugin,
            PlayerPlugin
        ))
        .run();
}




