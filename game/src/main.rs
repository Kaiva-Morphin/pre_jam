use bevy::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

use core::CorePlugin;

mod core;
mod camera;
mod utils;
mod physics;
mod interactions;

fn main() {
    App::new()
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::enabled(),
            crate::physics::scene::ScenePlugin,
        ))
        .run();
}




