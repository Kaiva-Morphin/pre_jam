use std::collections::HashMap;

use bevy::{asset::{self, LoadState, UntypedAssetId}, prelude::*};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::config::LoadingStateConfig};
use bevy_rapier2d::prelude::*;
use utils::WrappedDelta;

use crate::{camera::plugin::CameraFocus, core::states::{GlobalAppState, OnGame, PreGameTasks}, physics::{animator::{PlayerAnimationNode, PlayerAnimations, PlayerAnimatorPlugin}, constants::*, controller::{Controller, ControllerConstrants, ControllersPlugin}}};








pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnGame, spawn_player)
            .add_systems(Startup, load_player_assets)
            .add_systems(Update, (
                check_player_assets
            ).run_if(in_state(GlobalAppState::AssetLoading)))
            .add_systems(Update, controller.run_if(in_state(GlobalAppState::InGame)))
            .add_plugins(
                // ControllersPlugin,
                PlayerAnimatorPlugin
            )
        ;
    }
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerMesh;


pub const REG_FRICTION : Friction = Friction{coefficient: 1.0, combine_rule: CoefficientCombineRule::Average};

#[derive(Resource)]
pub struct PlayerAssetCollection {
    clips: HashMap<PlayerAnimationNode, AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
    player_scene: Handle<Scene>,
}

pub fn load_player_assets(
    asset_server: Res<AssetServer>,
    mut cmd: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut tasks: ResMut<PreGameTasks>,
) {
    tasks.add("player_assets".to_string());
    let mut clips = HashMap::new();
    for a in PlayerAnimationNode::iter() {
        let clip = asset_server.load(GltfAssetLabel::Animation(a as usize).from_asset("models/astro.glb"));
        clips.insert(a, clip);
    }
    let mut animation_graph = AnimationGraph::new();
    let mut anims = HashMap::new();
    for anim in PlayerAnimationNode::iter() {
        if !anims.contains_key(&anim) {
            let idx = animation_graph.add_clip(
                clips.get(&anim).unwrap().clone(),
                1.0,
                animation_graph.root
            );
            anims.insert(anim, idx);
        }
    }
    let player_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/astro.glb"));
    cmd.insert_resource(
        PlayerAssetCollection {
            clips: anims,
            graph: animation_graphs.add(animation_graph),
            player_scene,
        }
    )
}

fn check_player_assets(
    asset_server: Res<AssetServer>,
    mut tasks: ResMut<PreGameTasks>,
    assets: Res<PlayerAssetCollection>,
){
    let p = asset_server.get_load_state(&assets.player_scene);
    if let Some(s) = p {
        match s {
            asset::LoadState::Loaded => {}
            LoadState::Failed(e) => {error!("Error loading asset: {:?}, ignoring", e);}
            _ => {return;}
        }
    }
    tasks.done("player_assets".to_string());
}



pub fn spawn_player(
    mut cmd: Commands,
    assets: Res<PlayerAssetCollection>,
){
    cmd.spawn((
        (
            RigidBody::KinematicPositionBased,
            Transform::from_xyz(0.0, 100.0, 0.0),
            Player,
            Name::new("Player"),
            Collider::capsule(vec2(0.0, 22.0), vec2(0.0, -6.0), 8.0),
            LockedAxes::ROTATION_LOCKED,
            Sleeping::disabled(),
            REG_FRICTION,
            Ccd::enabled(),
        ),
        CollisionGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::from_bits(STRUCTURES_CG | LADDERS_CG).unwrap(),
        ),
        ActiveCollisionTypes::KINEMATIC_STATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        CameraFocus{priority: 0},
        Controller::default(),
        ControllerConstrants::default(),
        KinematicCharacterController::default(),
        PlayerState::default(),
        children![
            (
                SceneRoot(assets.player_scene.clone()),
                Transform::from_xyz(0.0, -12.7, 0.0).with_scale(Vec3::splat(5.0)),
                Visibility::Visible,
                PlayerMesh
            )
        ]
    ));
    
    cmd.insert_resource(PlayerAnimations::new(
        assets.clips.clone(),
        assets.graph.clone(),
        PlayerAnimationNode::Float,
    ));
    cmd.remove_resource::<PlayerAssetCollection>();

}


#[derive(Component)]
pub struct PlayerState{
    pub spacewalk: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            spacewalk: false,
        }
    }
}

pub fn controller(
    mut p: Single<(&mut KinematicCharacterController, &mut PlayerState, &mut ControllerConstrants, &mut Transform), With<Player>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
){
    let dt = time.dt();
    let mut raw_direction = Vec2::ZERO;
    keyboard.pressed(KeyCode::KeyA).then(|| raw_direction.x -= 1.0);
    keyboard.pressed(KeyCode::KeyD).then(|| raw_direction.x += 1.0);
    keyboard.pressed(KeyCode::KeyS).then(|| raw_direction.y -= 1.0);
    keyboard.pressed(KeyCode::KeyW).then(|| raw_direction.y += 1.0);
    let (c, s, cc, t) = &mut *p;
    c.translation = Some(raw_direction);
}