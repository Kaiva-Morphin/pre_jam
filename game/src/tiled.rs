use bevy::prelude::*;
use bevy::scene::SceneLoader;
use bevy_inspector_egui::bevy_egui::{EguiContextPass, EguiContexts};
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use bevy_ecs_tiled::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use pixel_utils::camera::HIGH_RES_LAYERS;
use core::CorePlugin;
use std::collections::{HashMap, HashSet};
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

    let mut clips = HashMap::new();
    for a in PlayerAnimationNode::iter() {
        let clip = asset_server.load(GltfAssetLabel::Animation(a as usize).from_asset("raw/astro.glb"));
        clips.insert(a, clip);
    }

    let mut animation_graph = AnimationGraph::new();
    let mut blends = HashMap::new();
    for n in BlendNode::iter() {
        let p = n.parent();

        let p = match p {
            Some(p) => blends.get(&p).cloned().expect("Reorder iter of BlendNode! :D"),
            None => animation_graph.root,
        };

        let node = animation_graph.add_blend(0.0, p);
        blends.insert(n, node);
    }


    let mut anims = HashMap::new();
    for b in BlendNode::iter() {
        for a in b.clips() {
            let idx = animation_graph.add_clip(
                clips.get(&a).unwrap().clone(),
                1.0,
                blends.get(&b).unwrap().clone(),
            );
            anims.insert(a, idx);
        }
    }

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


    let mut nodes = HashMap::new();
    let mut weights = HashMap::new();
    for (k, v) in anims {
        nodes.insert(AnimationNode::Clip(k), v);
        weights.insert(AnimationNode::Clip(k), 0.0);
    }
    for (k, v) in blends {
        nodes.insert(AnimationNode::Blend(k), v);
        weights.insert(AnimationNode::Blend(k), 0.0);
    }
    
    cmd.insert_resource(PlayerAnimations {
        nodes: nodes,
        graph: animation_graphs.add(animation_graph),
        weights: weights,
        target: PlayerAnimationNode::Idle,
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






pub struct PlayerAnimationGraph{
    states: Vec<(f32, PlayerAnimationNode)>,
    controls: Vec<(f32, String)>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Hash)]
pub enum PlayerAnimationNode {
    Breakdance,
    Climb,
    Dance,
    Dance2,
    Dance3,
    Dance4,
    Float,
    HeadSpin,
    HeadSpin2,
    Idle,
    Run,
    RunJump,
    Walk,
}

impl PlayerAnimationNode {
    pub fn iter() -> impl Iterator<Item = PlayerAnimationNode> {
        [
            PlayerAnimationNode::Breakdance,
            PlayerAnimationNode::Climb,
            PlayerAnimationNode::Dance,
            PlayerAnimationNode::Dance2,
            PlayerAnimationNode::Dance3,
            PlayerAnimationNode::Dance4,
            PlayerAnimationNode::Float,
            PlayerAnimationNode::HeadSpin,
            PlayerAnimationNode::HeadSpin2,
            PlayerAnimationNode::Idle,
            PlayerAnimationNode::Run,
            PlayerAnimationNode::RunJump,
            PlayerAnimationNode::Walk,
        ]
        .iter()
        .copied()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive, Hash)]
pub enum BlendNode {
    FloatClimb,
    ClimbIdle,
    IdleWalk,
    WalkRun,
    DancesMovement,
}

impl BlendNode {
    pub fn iter() -> impl Iterator<Item = BlendNode> {
        // !SEQUENCE IS IMPORTANT!
        [
            BlendNode::DancesMovement,
            BlendNode::FloatClimb,
            BlendNode::ClimbIdle,
            BlendNode::IdleWalk,
            BlendNode::WalkRun,
        ]
        .iter()
        .copied()
    }
    pub fn parent(&self) -> Option<Self> {
        match self {
            BlendNode::DancesMovement => None,
            BlendNode::FloatClimb => Some(BlendNode::DancesMovement),
            BlendNode::ClimbIdle => Some(BlendNode::FloatClimb),
            BlendNode::IdleWalk => Some(BlendNode::ClimbIdle),
            BlendNode::WalkRun => Some(BlendNode::IdleWalk),
        }
    }
    pub fn clips(&self) -> Vec<PlayerAnimationNode> {
        match self {
            BlendNode::FloatClimb => vec![PlayerAnimationNode::Float, PlayerAnimationNode::Climb],
            BlendNode::ClimbIdle => vec![PlayerAnimationNode::Climb, PlayerAnimationNode::Idle],
            BlendNode::IdleWalk => vec![PlayerAnimationNode::Idle, PlayerAnimationNode::Walk],
            BlendNode::WalkRun => vec![PlayerAnimationNode::Walk, PlayerAnimationNode::Run],
            BlendNode::DancesMovement => vec![
                PlayerAnimationNode::Breakdance,
                PlayerAnimationNode::Dance,
                PlayerAnimationNode::Dance2,
                PlayerAnimationNode::Dance3,
                PlayerAnimationNode::Dance4,
                PlayerAnimationNode::HeadSpin,
                PlayerAnimationNode::HeadSpin2,
                PlayerAnimationNode::RunJump,
            ],
        }
    }
}



#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnimationNode {
    Blend(BlendNode),
    Clip(PlayerAnimationNode),
}



#[derive(Resource)]
struct PlayerAnimations {
    nodes: HashMap<AnimationNode, AnimationNodeIndex>,
    weights: HashMap<AnimationNode, f32>,
    graph: Handle<AnimationGraph>,
    target: PlayerAnimationNode
}

impl PlayerAnimations {
    pub fn get_idx(&self, key: AnimationNode) -> AnimationNodeIndex {
        *self.nodes.get(&key).unwrap()
    }
    pub fn get_clip(&self, key: PlayerAnimationNode) -> AnimationNodeIndex {
        self.get_idx(AnimationNode::Clip(key))
    }
    pub fn get_blend(&self, key: BlendNode) -> AnimationNodeIndex {
        self.get_idx(AnimationNode::Blend(key))
    }
}


fn update(
    mut contexts: EguiContexts,
    mut player: Single<(Entity, &mut AnimationPlayer)>,
    mut animations: ResMut<PlayerAnimations>,
) {
    let ctx = contexts.ctx_mut();
    let (_e, p) = &mut *player;
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.heading("Weights");
        let nodes = animations.nodes.clone().into_iter().collect::<Vec<_>>();
        // nodes.sort_by(
        //     |a, b| match (a.0, b.0) {
        //     (AnimationNode::Blend(_), AnimationNode::Blend(_)) => std::cmp::Ordering::Equal,
        //     (AnimationNode::Blend(_), AnimationNode::Clip(_)) => std::cmp::Ordering::Less,
        //     (AnimationNode::Clip(_), AnimationNode::Blend(_)) => std::cmp::Ordering::Greater,
        //     (AnimationNode::Clip(_), AnimationNode::Clip(_)) => std::cmp::Ordering::Equal,
        // });
        
        for (a, i) in nodes.iter() {
            let AnimationNode::Clip(a) = a else {continue};
            ui.horizontal(|ui|{
                let t= ui.button(format!("{:?}: ", a));
                if t.clicked() {
                    animations.target = *a;
                }
                let a = p.animation_mut(*i);
                let Some(a) = a else {return;};
                let mut w = a.weight();
                let r = ui.add(Slider::new(&mut w, 0.0..=1.0));
                if r.changed() {
                    a.set_weight(w);
                }
            });
        }
        ui.separator();
        ui.heading("Graph");
        for (a, i) in nodes.iter() {
            let AnimationNode::Blend(a) = a else {continue};
            ui.horizontal(|ui|{
                ui.label(format!("{:?}: ", a));
                let a = p.animation_mut(*i);
                let Some(a) = a else {return;};
                let mut w = a.weight();
                let r = ui.add(Slider::new(&mut w, 0.0..=1.0));
                if r.changed() {
                    a.set_weight(w);
                }
            });
        }
        ui.separator();
        ui.heading(format!("Targeting: {:?}", animations.target));
    });
}












fn setup_scene_once_loaded(
    animations: Res<PlayerAnimations>,
    mut cmd: Commands,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    mut done: Local<bool>,
) {
    if *done {return}
    for (e, mut p) in player.iter_mut() {
        cmd.entity(e)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(AnimationTransitions::new());

        for k in animations.nodes.values() {
            p.play(*k).repeat();
            let a = p.animation_mut(*k).unwrap();
            a.set_weight(1.0);
        }
        *done = true;
    }
}


