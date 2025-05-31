use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: WindowResolution::new(1000., 1000.),
                            title: "Game".to_string(),
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
        ))
        .add_systems(Startup, spawn)
        .add_systems(Update, update)
        .run();
}


fn spawn(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn update() {}




