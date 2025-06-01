use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::*;
use camera::plugin::CameraControllerPlugin;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use interactions::components::*;
use interactions::InteractionsPlugin;
use physics::controller::ControllersPlugin;
use pixel_utils::camera::PixelCameraPlugin;
use shaders::{ShaderPlugin, VelocityEmmiter};
use utils::custom_material_loader::SpritePreloadPlugin;
use utils::mouse::CursorPositionPlugin;

mod core;
mod camera;
mod utils;
mod physics;
mod interactions;

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
        // ShaderPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(12.0),
        EguiPlugin { enable_multipass_for_primary_context: true },
        PixelCameraPlugin,
        CameraControllerPlugin,
        SwitchableEguiInspectorPlugin,
        SwitchableRapierDebugPlugin,
        ControllersPlugin,
        DebugOverlayPlugin::default(),
        InteractionsPlugin,
        CursorPositionPlugin,
        SpritePreloadPlugin,
    ))
    .add_systems(Startup, spawn.before(shaders::compute::setup))
    .run();
}

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    const GAP: f32 = 30.;
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(200., 2.),
        Name::new("Floor"),
    ));
    commands.spawn((
        (VelocityEmmiter,
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
        Ccd::enabled(),),
        camera::plugin::CameraFocus{priority: 0},
        physics::controller::Controller{
            horisontal_velocity: 0.0,
            max_horisontal_velocity: 100.0,
            total_air_jumps: 2,
            ..default()
        },
        CollisionGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::from_bits(STRUCTURES_CG).unwrap(),
        ),
    ))
    .with_child((
        Name::new("Player sensor"),
        Collider::ball(30.),
        CollisionGroups::new(
            Group::from_bits(PLAYER_SENSOR_CG).unwrap(),
            Group::from_bits(INTERACTABLE_CG).unwrap(),
        ),
        Sensor,
    ))
    // .with_child((
    //     SpatialListener::new(GAP);
    //     children![
    //         (
                
    //         )
    //     ]
    // ))
    ;
}
