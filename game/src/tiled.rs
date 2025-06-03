use bevy::prelude::*;
use bevy::scene::SceneLoader;
use bevy_inspector_egui::bevy_egui::{EguiContextPass, EguiContexts};
use bevy_inspector_egui::egui;
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use bevy_ecs_tiled::prelude::*;
use pixel_utils::camera::HIGH_RES_LAYERS;
use core::CorePlugin;
use std::f32::consts::PI;

use crate::camera::plugin::CameraFocus;
use crate::physics::controller::{Controller, ControllersPlugin};
use crate::physics::scene::{spawn_player, Player};
use crate::utils::background::StarBackgroundPlugin;

use bevy_ecs_tilemap::TilemapPlugin;
mod core;
mod camera;
mod utils;
mod physics;
mod interactions;
mod ui;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            DebugOverlayPlugin::default(),
            TilemapPlugin,
            TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: None }),
            SwitchableRapierDebugPlugin::disabled(),
            TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default(),
            StarBackgroundPlugin,
        ))
        .add_systems(Startup, start)
        .add_systems(Update, setup_scene_once_loaded)
        .add_systems(EguiContextPass, update)
        .run();
}


pub const RENDER_3D_WORLD: &str = "render_3d_world";



pub fn start(
    mut cmd: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            ..default()
        }
    ));

    let clip = asset_server.load(GltfAssetLabel::Animation(1).from_asset("raw/astro.glb"));
    let mut animation_graph = AnimationGraph::new();
    let node_indices = animation_graph
        .add_clips(vec![clip], 1.0, animation_graph.root)
        .collect();
    cmd.insert_resource(Animations {
        node_indices,
        graph: animation_graphs.add(animation_graph),
    });
    let astro = asset_server.load(GltfAssetLabel::Scene(0).from_asset("raw/astro.glb"));
    
    cmd.spawn((
        SceneRoot(astro.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(10.0)),
        Visibility::Visible,
    ));
    cmd.spawn((
        TiledMapHandle(asset_server.load("tilemaps/v1.0/pad_test.tmx")),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsRapierBackend>::default(),
    ));
}



pub struct PlayerAnimationGraph(pub Handle<AnimationGraph>);





fn update(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("world");
    });
}









fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut cmd: Commands,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    mut done: Local<bool>,
) {
    if *done {return}
    for (e, mut p) in player.iter_mut() {
        cmd.entity(e)
                .insert(AnimationGraphHandle(animations.graph.clone()))
                .insert(AnimationTransitions::new());
        p.play(animations.node_indices[0]).repeat();
        *done = true;
    }
}


#[derive(Resource)]
struct Animations {
    node_indices: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
