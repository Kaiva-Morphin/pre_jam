use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_tiled::map::TiledMapHandle;
use bevy_ecs_tiled::prelude::{TiledPhysicsPlugin, TiledPhysicsRapierBackend, TiledPhysicsSettings, TilemapAnchor};
use bevy_ecs_tiled::{TiledMapPlugin, TiledMapPluginConfig};
use bevy_ecs_tilemap::TilemapPlugin;
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
use utils::background::StarBackgroundPlugin;
use utils::custom_material_loader::SpritePreloadPlugin;
use utils::debree::DebreePlugin;
use utils::mouse::CursorPositionPlugin;

use crate::core::CorePlugin;

mod core;
mod camera;
mod utils;
mod physics;
mod interactions;
mod ui;

const AUDIO_SCALE: f32 = 1. / 100.0;

fn main() {
    App::new()
    .add_plugins((
        CorePlugin,
        (TilemapPlugin,
        TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: None }),
        TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default(),
        StarBackgroundPlugin,
        SwitchableEguiInspectorPlugin,
        SwitchableRapierDebugPlugin::enabled(),
        DebugOverlayPlugin::enabled(),),
    ))
    .add_systems(Startup, spawn.before(shaders::compute::setup))
    .run();
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const GAP: f32 = 50.;
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
        Sprite::from_image(asset_server.load("pixel/test.png")),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Ccd::enabled(),),
        Friction{coefficient: 0.0, combine_rule: CoefficientCombineRule::Min},
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
        SpatialListener::new(-GAP),
        children![
        (
            Name::new("Player sensor"),
            Collider::ball(30.),
            CollisionGroups::new(
                Group::from_bits(PLAYER_SENSOR_CG).unwrap(),
                Group::from_bits(INTERACTABLE_CG).unwrap(),
            ),
            Sensor,
        ),
        // left ear
        (
            Sprite::from_color(Color::Srgba(RED), Vec2::splat(20.0)),
            Transform::from_xyz(-GAP / 2., 0.0, 0.0),
        ),
        // right ear
        (
            Sprite::from_color(Color::Srgba(BLUE), Vec2::splat(20.0)),
            Transform::from_xyz(GAP / 2., 0.0, 0.0),
        )
        ],
    ))
    ;
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/173273__tomlija__janitors-bedroom-ambience.wav")),
        PlaybackSettings::LOOP.with_spatial(true),
        Transform::from_xyz(50., 0., 0.),
        Sprite::from_color(Color::Srgba(GREEN), Vec2::splat(20.0)),
    ));
    commands.spawn((
        TiledMapHandle(asset_server.load("tilemaps/v1.0/map.tmx")),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsRapierBackend>::default(),
    ));
}
