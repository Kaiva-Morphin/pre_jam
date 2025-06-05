use std::{collections::HashMap, f32::consts::PI, sync::{Arc, RwLock}};

use bevy::{asset::{self, LoadState, UntypedAssetId}, prelude::*};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::config::LoadingStateConfig};
use bevy_inspector_egui::{bevy_egui::{EguiContextPass, EguiContexts}, egui::{self, debug_text}};
use bevy_rapier2d::prelude::*;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use utils::WrappedDelta;

use crate::{camera::plugin::CameraFocus, core::states::{GlobalAppState, OnGame, PreGameTasks}, physics::{animator::{PlayerAnimationNode, PlayerAnimations, PlayerAnimatorPlugin}, constants::*}, tilemap::plugin::LadderCollider};
use utils::MoveTowards;







pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnGame, spawn_player)
            .add_systems(Startup, load_player_assets)
            .add_systems(EguiContextPass, debug)
            .insert_resource(PlayerConstants::default())
            .insert_resource(NearestLadders{ladders: HashMap::new()})
            .add_systems(Update, (
                check_player_assets
            ).run_if(in_state(GlobalAppState::AssetLoading)))
            .add_systems(Update, (
                tick_controllers,
                update_controllers,
                listen_events
            ).run_if(in_state(GlobalAppState::InGame)))
            .add_plugins(
                // ControllersPlugin,
                PlayerAnimatorPlugin
            )
        ;
    }
}



#[derive(Component)]
pub struct PlayerMesh;


pub const REG_FRICTION : Friction = Friction{coefficient: 1.0, combine_rule: CoefficientCombineRule::Max};

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


#[derive(Resource, Debug, Clone)]
pub struct PlayerConstants {
    pub gravity: Vec2,

    pub speed_gain: f32,
    pub speed_loss: f32,

    pub walk_speed: f32,
    pub run_speed: f32,
    pub climb_speed: f32,
    pub climb_out_speed: f32,

    pub max_horisontal_velocity: f32,
    pub max_vertical_velocity: f32,
    
    pub spacewalk_max_linvel: f32,
    pub spacewalk_max_angvel: f32,
    pub spacewalk_speed: f32,
    pub spacewalk_ang_speed: f32,

    pub mesh_turn_speed: f32,
    pub mesh_rot_speed: f32,

    pub mesh_rot_attn: f32,
    pub mesh_rot_weight: f32,
    pub mesh_vel_attn: f32,

    pub jump_vel: f32,
    pub air_jump_vel: f32,

    pub total_air_jumps: usize,
}

impl Default for PlayerConstants {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -981.0 * 0.5),
            speed_gain: 1400.0,
            speed_loss: 350.0,
            walk_speed: 55.0,
            run_speed: 120.0,
            
            climb_speed: 20.0,
            climb_out_speed: 200.0,

            spacewalk_max_linvel: 100.0,
            spacewalk_max_angvel: 2.0,
            spacewalk_speed: 1.0,
            spacewalk_ang_speed: 0.5,
            jump_vel: 200.0,
            air_jump_vel: 150.0,
            max_horisontal_velocity: 500.0,
            max_vertical_velocity: 500.0,
            total_air_jumps: 0,

            mesh_rot_attn: 0.2,
            mesh_rot_weight: 1.0,
            mesh_vel_attn: 0.1,

            mesh_turn_speed: 16.0,
            mesh_rot_speed: 16.0,
        }
    }
}

pub fn spawn_player(
    mut cmd: Commands,
    assets: Res<PlayerAssetCollection>,
){
    cmd.spawn((
        (
        Transform::from_xyz(0.0, 200.0, 0.0),
        Player::default(),
        // ActiveHooks::MODIFY_SOLVER_CONTACTS,
        Name::new("Player"),
        Collider::capsule(vec2(0.0, 22.0), vec2(0.0, -6.0), 8.0),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        GravityScale(0.0),
        REG_FRICTION,
        CameraFocus{priority: 0},
        Ccd::enabled(),
        Visibility::default(),
        InheritedVisibility::default(),
        ),
        CollisionGroups::new(
        Group::from_bits(PLAYER_CG).unwrap(),
        Group::from_bits(STRUCTURES_CG | LADDERS_CG).unwrap(),
        ),
        (
            RigidBody::Dynamic,
            Controller::default(),
            Velocity::default(),
        ),
        // (
        //     RigidBody::KinematicVelocityBased,
        //     KinematicCharacterControllerOutput::default(),
        //     ActiveCollisionTypes::KINEMATIC_STATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        //     KinematicCharacterController {
        //         snap_to_ground: Some(CharacterLength::Absolute(10.0)),
        //         up: Vec2::Y,
        //         autostep: Some(CharacterAutostep{
        //             max_height: CharacterLength::Absolute(10.0),
        //             min_width: CharacterLength::Absolute(10.0),
        //             include_dynamic_bodies: true,
        //         }),
        //         max_slope_climb_angle: 1.0,
        //         min_slope_slide_angle: 1.0,
        //         ..default()
        //     },
        // ),
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


#[derive(Component, Clone, Debug)]
pub struct Player {
    pub state: PlayerState
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    Regular{
        accumulated_vel: f32,
    },
    Dance,
    Climbing{
        ladder: Ladder
    },
    Spacewalk
}

#[derive(Resource)]
pub struct NearestLadders{
    pub ladders: HashMap<Entity, Ladder>
}

pub fn listen_events(
    mut collision_events: EventReader<CollisionEvent>,
    q: Query<&GlobalTransform, With<LadderCollider>>,
    mut l: ResMut<NearestLadders>
){
    for collision_event in collision_events.read() {
        let (s, m, i) = match collision_event {
            CollisionEvent::Started(s, m, _) => (s, m, true),
            CollisionEvent::Stopped(s, m, _) => (s, m, false),
        };
        let Ok((t, e)) = q.get(*s).map(|v|(v, *s)).or(q.get(*m).map(|v|(v, *m))) else {continue;};
        if i {
            l.ladders.insert(e, Ladder{x_pos: t.translation().x, entity: e});
        } else {
            l.ladders.remove(&e);
        }
    }
}


#[derive(Clone, Debug)]
pub struct Ladder {
    pub x_pos: f32,
    pub entity: Entity,
}

impl Player {
    pub fn is_climbing(&self) -> bool {
        matches!(self.state, PlayerState::Climbing{ladder: _})
    }
    pub fn is_spacewalking(&self) -> bool {
        matches!(self.state, PlayerState::Spacewalk)
    }
    pub fn is_dancing(&self) -> bool {
        matches!(self.state, PlayerState::Dance)
    }
    pub fn is_regular(&self) -> bool {
        matches!(self.state, PlayerState::Regular{accumulated_vel: _})
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Regular{accumulated_vel: 0.0},
        }
    }
}

#[derive(Component, Debug)]
pub struct Controller {
    pub horisontal_velocity: f32,
    pub jumping: bool,
    pub air_jumps: usize,
    pub time_in_air: f32,
    pub platform_velocity: Option<Vec2>,
}


impl Controller {
    pub fn is_on_floor(&self) -> bool {
        self.time_in_air < 0.1
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            horisontal_velocity: 0.0,
            jumping: false,
            air_jumps: 0,
            time_in_air: 0.0,
            platform_velocity: None,
        }
    }
}



fn debug(
    mut contexts: EguiContexts,
    // mut player: Single<&mut KinematicCharacterController>,
    mut consts: ResMut<PlayerConstants>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Vars").show(ctx, |ui| {
        ui.heading("Constants");
        // ui.label("Gravity");
        // ui.add(egui::Slider::new(&mut consts.gravity.y, -10.0..=500.0));
        // ui.label("Speed gain");
        // ui.add(egui::Slider::new(&mut consts.speed_gain, 0.0..=500.0));
        // ui.label("Speed loss");
        // ui.add(egui::Slider::new(&mut consts.speed_loss, 0.0..=500.0));
        ui.label("Walk speed");
        ui.add(egui::Slider::new(&mut consts.walk_speed, 0.0..=1000.0));
        ui.label("Run speed");
        ui.add(egui::Slider::new(&mut consts.run_speed, 0.0..=1000.0));
        ui.label("Climb speed");
        ui.add(egui::Slider::new(&mut consts.climb_speed, 0.0..=1000.0));
        ui.label("Climb out speed");
        ui.add(egui::Slider::new(&mut consts.climb_out_speed, 0.0..=1000.0));

        ui.label("mesh_rot_attn");
        ui.add(egui::Slider::new(&mut consts.mesh_rot_attn, 0.0..=1000.0));

        ui.label("mesh_rot_weight");
        ui.add(egui::Slider::new(&mut consts.mesh_rot_weight, 0.0..=1000.0));

        ui.label("mesh_vel_attn");
        ui.add(egui::Slider::new(&mut consts.mesh_vel_attn, 0.0..=1000.0));
        // ui.separator();
        // ui.heading("Slide");
        // ui.checkbox(&mut player.slide, "Slide");
        // if let Some(a) = &mut player.autostep {
        //     ui.heading("Autostep");
        //     if let bevy_rapier2d::prelude::CharacterLength::Absolute(mut v) = a.max_height {
        //         ui.label("Height");
        //         if ui.add(egui::Slider::new(&mut v, 0.0..=100.0)).changed() {
        //             a.max_height = bevy_rapier2d::prelude::CharacterLength::Absolute(v);
        //         }
        //     }
        //     if let bevy_rapier2d::prelude::CharacterLength::Absolute(mut v) = a.min_width {
        //         ui.label("Width");
        //         if ui.add(egui::Slider::new(&mut v, 0.0..=100.0)).changed() {
        //             a.min_width = bevy_rapier2d::prelude::CharacterLength::Absolute(v);
        //         }
        //     }
        // }
        // ui.heading("Max Slope Climb Angle");
        // ui.add(egui::Slider::new(&mut player.max_slope_climb_angle, 0.0..=1.0)).changed();
        // ui.heading("Min Slope Slide Angle");
        // ui.add(egui::Slider::new(&mut player.min_slope_slide_angle, 0.0..=1.0)).changed();
        // if let Some(s) = player.snap_to_ground {
        //     ui.heading("Snap To Ground");
        //     if let bevy_rapier2d::prelude::CharacterLength::Absolute(mut v) = s {
        //         if ui.add(egui::Slider::new(&mut v, 0.0..=100.0)).changed() {
        //             player.snap_to_ground = Some(bevy_rapier2d::prelude::CharacterLength::Absolute(v));
        //         }
        //     }
        // }
        // ui.heading("Normal Nudge Factor");
        // ui.add(egui::Slider::new(&mut player.normal_nudge_factor, 0.0..=1.0)).changed();
    });
}






pub fn update_controllers(
    mut player: Single<(Entity, &mut Velocity, &mut Player, &mut Controller, &mut Transform), (With<Player>, Without<PlayerMesh>)>,
    mut player_mesh: Single<&mut Transform, (With<PlayerMesh>, Without<Player>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
    time: Res<Time>,
    mut cmd: Commands,
    mut anim: ResMut<PlayerAnimations>,
    
    mut consts: ResMut<PlayerConstants>,

    mut mesh_turn: Local<f32>,
    mut mesh_rotation: Local<f32>,
    ladders: Res<NearestLadders>,
    mut time_since_climb: Local<f32>,
){
    let dt = time.dt();
    let mut raw_dir = Vec2::ZERO;
    keyboard.pressed(KeyCode::KeyA).then(|| raw_dir.x -= 1.0);
    keyboard.pressed(KeyCode::KeyD).then(|| raw_dir.x += 1.0);
    keyboard.pressed(KeyCode::KeyS).then(|| raw_dir.y -= 1.0);
    keyboard.pressed(KeyCode::KeyW).then(|| raw_dir.y += 1.0);

    // let direction = raw_direction.normalize_or_zero();

    // overlay_text!(overlay_events;TopCenter;PLAYER_INPUTS:
    //     "Plyer inputs ->".to_string(),(100);
    //     format!("{:.1} {:.1}", direction.x, direction.y),(255, 100, 100);
    // );




    // animations.target = PlayerAnimationNode::Float;


    let (player_e, player_vel, player, controller, transform) = &mut *player;





    if keyboard.just_pressed(KeyCode::KeyC) {
        if !player.is_spacewalking()  {
            player.state = PlayerState::Spacewalk;
            consts.gravity = Vec2::ZERO;
            // cmd.entity(*player_e).insert((
            //     Friction::coefficient(1.0),
            // ));
            cmd.entity(*player_e).insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_X);
        } else {
            player.state = PlayerState::Regular { accumulated_vel: 0.0 };
            consts.gravity = PlayerConstants::default().gravity;
            cmd.entity(*player_e).insert((
                LockedAxes::ROTATION_LOCKED,
                // REG_FRICTION,
            ));
            transform.rotation = Quat::IDENTITY;
        }
    }
    if !player.is_climbing() {
        *time_since_climb += dt;
    } else {
        *time_since_climb = 0.0;
    }
    match &mut player.state {
        PlayerState::Dance => {
            if raw_dir.x != 0.0 || !controller.is_on_floor() {
                player.state = PlayerState::Regular{accumulated_vel: 0.0};
                anim.target = PlayerAnimationNode::Idle;
                player_vel.linvel += consts.gravity;
            }
        }
        PlayerState::Climbing{ladder: Ladder{x_pos: l, entity: e}} => {
            controller.horisontal_velocity = 0.0;
            if let None = ladders.ladders.get(e) {
                player.state = PlayerState::Regular{accumulated_vel: 0.0};
                *mesh_turn = if raw_dir.x.abs() < 0.1 { PI } else 
                if raw_dir.x > 0.0 {PI / 2.0} else {- PI / 2.0};
                return;
            }
            anim.target = PlayerAnimationNode::Climb;
            anim.params.climb_speed = raw_dir.y;
            player_vel.linvel.y = raw_dir.y * consts.climb_speed;
            *mesh_turn = 0.0;
            player_mesh.rotation = Quat::from_axis_angle(Vec3::Y, *mesh_turn);
            if raw_dir.x != 0.0 && raw_dir.y == 0.0 {
                player_vel.linvel.x = raw_dir.x * consts.climb_out_speed;
                player.state = PlayerState::Regular{accumulated_vel: 0.0};
                *mesh_turn = if raw_dir.x.abs() < 0.1 { PI } else 
                if raw_dir.x > 0.0 {PI / 2.0} else {- PI / 2.0};
                return;
            } else {
                player_vel.linvel.x = 0.0;
                transform.translation.x = transform.translation.x.move_towards(*l,dt * 20.0);
            }
        }
        PlayerState::Regular { accumulated_vel: _ } => {
            if raw_dir.y != 0.0 && *time_since_climb > 0.1 {
                if let Some((_, l)) = ladders.ladders.iter().next() {
                    //raw_dir.y != 0.0 
                    player.state = PlayerState::Climbing{ladder: l.clone()};
                    anim.target = PlayerAnimationNode::Climb;
                }
            }
            
            let target = raw_dir.x * if keyboard.pressed(KeyCode::ShiftLeft) {consts.run_speed} else {consts.walk_speed};

            if controller.is_on_floor() {
                controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, consts.speed_gain * dt);
            } else {
                controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, consts.speed_loss * dt);
            }

            // controller.horisontal_velocity += diff * *speed_gain * dt;

            player_vel.linvel.x = controller.horisontal_velocity;

            let sp = keyboard.pressed(KeyCode::Space);
            let sjp = keyboard.just_pressed(KeyCode::Space);
            if keyboard.just_pressed(KeyCode::KeyZ) {controller.jumping = false};
            
            if controller.is_on_floor() {
                if controller.horisontal_velocity > 0.0 {
                    *mesh_turn = mesh_turn.move_towards(PI * 0.5, dt * consts.mesh_turn_speed);
                } else if controller.horisontal_velocity < 0.0 {
                    *mesh_turn = mesh_turn.move_towards(-PI * 0.5, dt * consts.mesh_turn_speed);
                }
                if player_vel.linvel.x.abs() < 1.0 {
                    anim.target = PlayerAnimationNode::Idle;
                } else if player_vel.linvel.x.abs() < consts.walk_speed + 2.0{
                    anim.target = PlayerAnimationNode::Walk;
                } else {
                    anim.target = PlayerAnimationNode::Run;
                }
                if sjp {
                    controller.jumping = true;
                    player_vel.linvel.y = consts.jump_vel;
                }
            } else {
                anim.target = PlayerAnimationNode::Float;

                if sjp && controller.air_jumps > 0 {
                    controller.air_jumps -= 1;
                    controller.jumping = true;
                    player_vel.linvel.y = consts.air_jump_vel;
                }
            }
            player_mesh.rotation = Quat::from_axis_angle(Vec3::Y, *mesh_turn);
            
            if !sp || player_vel.linvel.y < 0.0 {
                controller.jumping = false;
            }
        }
        PlayerState::Spacewalk => {
            anim.target = PlayerAnimationNode::Float;
            let ang_dir = keyboard.pressed(KeyCode::KeyA) as usize as f32 - keyboard.pressed(KeyCode::KeyD) as usize as f32;
            raw_dir.x = keyboard.pressed(KeyCode::KeyE) as usize as f32 - keyboard.pressed(KeyCode::KeyQ) as usize as f32;
            let target = player_vel.angvel + ang_dir * dt * consts.spacewalk_ang_speed;
            if target.abs() > player_vel.angvel.abs() {
                player_vel.angvel = target.clamp(-consts.spacewalk_max_angvel, consts.spacewalk_max_angvel);
            } else {
                player_vel.angvel = target;
            }

            let impulse = raw_dir.rotate(transform.right().xy());
            let target = player_vel.linvel + impulse;

            if target.length_squared() > player_vel.linvel.length_squared() {
                if target.length_squared() > consts.spacewalk_max_linvel * consts.spacewalk_max_linvel {
                    player_vel.linvel = target.normalize_or_zero() * consts.spacewalk_max_linvel;
                } else {
                    player_vel.linvel = target;
                }
            } else {
                player_vel.linvel = target;
            }

            let desired_dir = player_vel.linvel.normalize_or_zero();
            let current_dir = transform.right().xy().normalize_or_zero();
            let dot = desired_dir.dot(current_dir);
            
            let v = player_vel.linvel.x * consts.mesh_vel_attn;
            let r = -player_vel.angvel * consts.mesh_rot_attn;
            // TODO: IMPROVE
            let t = if r.abs() * consts.mesh_rot_weight> v.abs() {
                r
            } else {
                if dot.abs() > 0.8 {
                    v
                } else {
                    r
                }             
            };
            *mesh_turn = mesh_turn.move_towards((t).clamp(-PI / 2.0, PI / 2.0), dt * consts.mesh_turn_speed * 0.2);
            
            player_mesh.rotation = Quat::from_axis_angle(Vec3::Y, *mesh_turn);

            *mesh_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
            controller.horisontal_velocity = player_vel.linvel.x;
        }
    }

    overlay_text!(overlay_events;TopLeft;CONTROLLER:
        format!("{:#?}", player),(255, 255, 255);
    );
}








pub fn tick_controllers(
    time: Res<Time>,
    ctx: ReadRapierContext,
    mut player: Single<(Entity, &mut Player, &mut Velocity, &mut Controller, &Collider, &Transform)>,
    consts: Res<PlayerConstants>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
){
    overlay_text!(overlay_events;TopLeft;FIXED_DT:format!("Fixed dt: {:.1} ({:.1} fps)", time.delta_secs(), 1.0 / time.delta_secs()),(255, 255, 255););
    let dt = time.dt();
    let Ok(ctx) = ctx.single() else {return};
    let (e, p, v, c, co, t) = &mut *player;
    c.time_in_air += dt;
    c.platform_velocity = None;
    let g = consts.gravity;
    let g = if c.jumping {g * 1.0} else {g};
    let filter = QueryFilter::default().exclude_collider(*e).exclude_sensors();
    let options = ShapeCastOptions {max_time_of_impact: 2.0 / 64.0,
        target_distance: 0.0,
        stop_at_penetration: true,
        compute_impact_geometry_on_penetration: true,
    };
    let pos = t.translation.truncate();
    if v.linvel.y > -consts.max_vertical_velocity {
        if !p.is_climbing() {
            v.linvel += g * dt;
        }
    }
    // let max_vel = vec2(c.max_horisontal_velocity, c.max_vertical_velocity);
    // v.linvel = v.linvel.clamp(-max_vel, max_vel);
    // let g = g.normalize();

    
    let mut col = co.clone();
    let mut ca = col.as_capsule_mut().expect("Player must be capsule... please?...");
    ca.set_radius(ca.radius() * 0.95);
    // col.set_scale(Vec2::splat(0.95), 1);
    // floor checks
    if let Some((_, hit)) =
        ctx.cast_shape(pos, 0.0, g * dt * 2.0, 
        &col, options, filter) {
        let Some(d) = hit.details else {return;};
        if d.normal1.dot(g) < -0.7 {
            c.time_in_air = 0.0;
            c.air_jumps = consts.total_air_jumps;
            c.jumping = false;
        }
    };
}