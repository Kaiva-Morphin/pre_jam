use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::*;
use camera::plugin::CameraControllerPlugin;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use physics::controller::ControllersPlugin;
use pixel_utils::camera::PixelCameraPlugin;
use shaders::{ShaderPlugin, VelocityEmmiter};

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
        ControllersPlugin,
        DebugOverlayPlugin::default(),
    ))
    .add_systems(Startup, spawn.before(shaders::compute::setup))
    .run();
}

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(200., 2.),
        Name::new("Floor"),
    ));
    commands.spawn((
        VelocityEmmiter,
        RigidBody::Dynamic,
        Transform::from_xyz(0.0, 100.0, 0.0),
        Velocity::zero(),
        physics::scene::Player,
        Dominance::group(0),
        GravityScale(0.0),
        Name::new("Player"),
        Collider::capsule(vec2(0.0, 6.0), vec2(0.0, -6.0), 6.0),
        Sprite::from_image(assets.load("pixel/test.png")),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Ccd::enabled(),
        camera::plugin::CameraFocus{priority: 0},
        physics::controller::Controller{
            horisontal_velocity: 0.0,
            max_horisontal_velocity: 100.0,
            total_air_jumps: 2,
            ..default()
        }
    ));
}
