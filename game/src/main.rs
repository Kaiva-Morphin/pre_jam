use bevy::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

mod core;
use core::CorePlugin;
mod camera;
mod utils;
mod physics;

fn main() {
    App::new()
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::enabled()
        ))
        .run();
}




