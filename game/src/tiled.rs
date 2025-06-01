use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use bevy_ecs_tiled::prelude::*;
use core::CorePlugin;

use crate::camera::plugin::CameraFocus;
use crate::physics::controller::{Controller, ControllersPlugin};
use crate::physics::scene::{spawn_player, Player};
use bevy_ecs_tilemap::TilemapPlugin;
mod core;
mod camera;
mod utils;
mod physics;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::enabled(),
            TilemapPlugin,
            TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: None }),
            TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default(),
        ))
        .add_systems(Startup, (start))
        .run();
}

pub fn start(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
){
    cmd.spawn((
        RigidBody::Dynamic,
        Transform::from_xyz(0.0, 100.0, 0.0),
        Velocity::zero(),
        Player,
        Dominance::group(0),
        GravityScale(0.0),
        Name::new("Player"),
        Collider::capsule(vec2(0.0, 6.0), vec2(0.0, -6.0), 6.0),
        Sprite::from_image(asset_server.load("pixel/test.png")),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Friction{coefficient: 0.0, combine_rule: CoefficientCombineRule::Min},
        Ccd::enabled(),
        CameraFocus{priority: 0},
        Controller{
            horisontal_velocity: 0.0,
            max_horisontal_velocity: 100.0,
            total_air_jumps: 2,
            ..default()
        }
    ));

    cmd.spawn((
        TiledMapHandle(asset_server.load("tilemaps/test/main.tmx")),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsRapierBackend>::default(),
    ));

    cmd.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(100.0, 5.0),
    ));
}


