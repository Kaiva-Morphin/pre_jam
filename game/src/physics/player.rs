use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{camera::plugin::CameraFocus, physics::{animator::{PlayerAnimationNode, PlayerAnimations, PlayerAnimatorPlugin}, constants::*, controller::{Controller, ControllerConstrants, ControllersPlugin}}};








pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, spawn_player)
            .add_plugins((
                ControllersPlugin,
                PlayerAnimatorPlugin
            ))
        ;
    }
}





#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerMesh;


pub const REG_FRICTION : Friction = Friction{coefficient: 1.0, combine_rule: CoefficientCombineRule::Average};

pub fn spawn_player(
    mut cmd: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
){
    let astro = asset_server.load(GltfAssetLabel::Scene(0).from_asset("raw/astro.glb"));
    cmd.spawn((
        (
            RigidBody::Dynamic,
            Transform::from_xyz(0.0, 100.0, 0.0),
            Velocity::zero(),
            Player,
            Dominance::group(0),
            GravityScale(0.0),
            Name::new("Player"),
            Collider::capsule(vec2(0.0, 18.0), vec2(0.0, -6.0), 6.0),
            LockedAxes::ROTATION_LOCKED,
            Sleeping::disabled(),
            REG_FRICTION,
            Ccd::enabled(),
        ),
        CollisionGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::from_bits(STRUCTURES_CG | LADDERS_CG).unwrap(),
        ),
        CameraFocus{priority: 0},
        Controller::default(),
        ControllerConstrants::default(),
        children![
            (
                SceneRoot(astro.clone()),
                Transform::from_xyz(0.0, -12.7, 0.0).with_scale(Vec3::splat(5.0)),
                Visibility::Visible,
                PlayerMesh
            )
        ]
    ));
    let mut clips = HashMap::new();
    for a in PlayerAnimationNode::iter() {
        let clip = asset_server.load(GltfAssetLabel::Animation(a as usize).from_asset("raw/astro.glb"));
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
    cmd.insert_resource(PlayerAnimations::new(
        anims,
        animation_graphs.add(animation_graph),
        PlayerAnimationNode::Float,
    ));

}


