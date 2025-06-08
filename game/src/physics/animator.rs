use std::collections::HashMap;

use bevy::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use utils::{MoveTowards, WrappedDelta};

use crate::core::states::GlobalAppState;




pub(super) struct PlayerAnimatorPlugin;

impl Plugin for PlayerAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, (setup_scene_once_loaded, update).run_if(in_state(GlobalAppState::InGame)));
    }
}




fn update(
    animations: Res<PlayerAnimations>,
    time: Res<Time>,
    mut player: Single<(Entity, &mut AnimationPlayer)>,
){
    let (_e, p) = &mut *player;
    let dt = time.dt();
    for (k, v) in animations.nodes.iter() {
        p.play(*v).repeat();
        let a = p.animation_mut(*v).unwrap();
        
        let target_val = if animations.target == *k {
            1.0
        } else {
            0.0
        };
        if PlayerAnimationNode::Climb == *k {
            a.set_speed(animations.params.climb_speed);
        }

        if k.need_interpolation() && animations.target.need_interpolation() {
            let s = if PlayerAnimationNode::Float == animations.target ||
            PlayerAnimationNode::Float == *k {8.0} else {6.0};
            a.set_weight(a.weight().move_towards(target_val, dt * s));
        } else {
            a.set_weight(target_val);
        }
    }
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
        .copied() //hehehaha
    }
    pub fn random_dance() -> PlayerAnimationNode {
        match getrandom::u32().unwrap() % 7 {
            0 => PlayerAnimationNode::Breakdance,
            1 => PlayerAnimationNode::Dance,
            2 => PlayerAnimationNode::Dance2,
            3 => PlayerAnimationNode::Dance3,
            4 => PlayerAnimationNode::Dance4,
            5 => PlayerAnimationNode::HeadSpin,
            6 => PlayerAnimationNode::HeadSpin2,
            _ => {unreachable!()},
        }
    }
    pub fn need_interpolation(&self) -> bool {
        matches!(self, PlayerAnimationNode::Idle | PlayerAnimationNode::Walk | 
        PlayerAnimationNode::Run | PlayerAnimationNode::Float)
    }
}


#[derive(Resource)]
pub struct PlayerAnimations {
    nodes: HashMap<PlayerAnimationNode, AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
    pub target: PlayerAnimationNode,
    pub params: AnimParams,
}
#[derive(Default)]
pub struct AnimParams {
    pub climb_speed: f32
}

impl PlayerAnimations {
    pub fn new(
        nodes: HashMap<PlayerAnimationNode, AnimationNodeIndex>,
        graph: Handle<AnimationGraph>,
        target: PlayerAnimationNode
    ) -> Self {
        Self {
            nodes,
            graph,
            target,
            params: Default::default(),
        }
    }
    pub fn get_clip(&self, key: &PlayerAnimationNode) -> Option<&AnimationNodeIndex> {
        self.nodes.get(key)
    }

    pub fn nodes(&self) -> &HashMap<PlayerAnimationNode, AnimationNodeIndex> {
        &self.nodes
    }
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

        for (k, v) in animations.nodes.iter() {
            p.play(*v).repeat();
            let a = p.animation_mut(*v).unwrap();
            let target = if animations.target == *k {
                1.0
            } else {
                0.0
            };
            a.set_weight(target);
        }
        *done = true;
    }
}

