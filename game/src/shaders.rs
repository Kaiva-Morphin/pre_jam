use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use camera::plugin::CameraControllerPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use pixel_utils::camera::PixelCameraPlugin;
use shaders::ShaderPlugin;

mod core;
mod camera;
mod utils;
mod physics;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800., 800.),
                    title: "Game".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        ShaderPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(12.0),
        EguiPlugin { enable_multipass_for_primary_context: true },
        PixelCameraPlugin,
        CameraControllerPlugin,
        SwitchableEguiInspectorPlugin,
    ))
    .run();
}

