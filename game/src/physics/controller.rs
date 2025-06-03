use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

use bevy::app::FixedMain;
use bevy::color::palettes::css::{GREEN, RED};
use bevy::ecs::query::{QueryData, WorldQuery};
use bevy::input::keyboard;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::picking::pointer::PointerLocation;
use bevy::{gizmos, prelude::*};


use bevy_inspector_egui::bevy_egui::{EguiContext, EguiContextPass, EguiContexts};
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayEvent;
use debug_utils::overlay_text;
use pixel_utils::camera::PixelCamera;
use utils::{wrap, MoveTowards, WrappedDelta};

use crate::physics::animator::{PlayerAnimationNode, PlayerAnimations};
use crate::physics::player::{Player, PlayerMesh, REG_FRICTION};

use super::platforms::{MovingPlatform, MovingPlatformMode};

pub struct ControllersPlugin;

impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_controllers)
            .add_systems(FixedPreUpdate, tick_controllers)
            .add_systems(EguiContextPass, debug)
            .insert_resource(SpaceWalk(false))
            .insert_resource(GlobalGravity(Vec2::new(0.0, -981. / 2.0)))
            ;
    }
}





#[derive(Resource)]
pub struct GlobalGravity(pub Vec2);
#[derive(Component)]
pub struct GravityOverride(pub Vec2);


#[derive(Component, Debug)]
pub struct Controller {
    pub horisontal_velocity: f32,
    pub jumping: bool,
    pub air_jumps: usize,
    pub time_in_air: f32,
    pub platform_velocity: Option<Vec2>,
}

#[derive(Component, Debug)]
pub struct ControllerConstrants {
    pub speed_gain: f32,
    pub speed_loss: f32,

    pub walk_speed: f32,
    pub run_speed: f32,
    pub climb_speed: f32,

    pub max_slope_angle: f32,
    pub snap_to_ground_depth: f32,
    pub snap_to_ground_height: f32,

    pub max_autostep_angle: f32,
    pub autostep_depth: f32,
    pub autostep_height: f32,

    pub max_horisontal_velocity: f32,
    pub max_vertical_velocity: f32,
    
    pub spacewalk_max_linvel: f32,
    pub spacewalk_max_angvel: f32,
    pub spacewalk_speed: f32,
    pub spacewalk_ang_speed: f32,

    pub mesh_turn_speed: f32,
    pub mesh_rot_speed: f32,

    pub jump_vel: f32,
    pub air_jump_vel: f32,

    pub total_air_jumps: usize,
}

impl Default for ControllerConstrants {
    fn default() -> Self {
        Self {
            speed_gain: 1400.0,
            speed_loss: 350.0,
            walk_speed: 80.0,
            run_speed: 120.0,
            
            climb_speed: 120.0,

            max_slope_angle: PI / 3.0,
            snap_to_ground_depth: 10.0,
            snap_to_ground_height: 10.0,

            max_autostep_angle: PI / 4.0,
            autostep_depth: 20.0,
            autostep_height: 20.0,

            spacewalk_max_linvel: 100.0,
            spacewalk_max_angvel: 2.0,
            spacewalk_speed: 1.0,
            spacewalk_ang_speed: 0.5,
            jump_vel: 200.0,
            air_jump_vel: 150.0,
            max_horisontal_velocity: 500.0,
            max_vertical_velocity: 500.0,
            total_air_jumps: 0,

            mesh_turn_speed: 16.0,
            mesh_rot_speed: 16.0,
        }
    }
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

pub fn tick_controllers(
    time: Res<Time>,
    ctx: ReadRapierContext,
    mut controllers: Query<(Entity, &mut Velocity, &mut Controller, &ControllerConstrants, Option<&GravityOverride>, &Collider, &Transform)>,
    platforms: Query<&MovingPlatform>,
    global_gravity: Res<GlobalGravity>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
    mut gizmos: Gizmos
){
    overlay_text!(overlay_events;TopLeft;FIXED_DT:format!("Fixed dt: {:.1} ({:.1} fps)", time.delta_secs(), 1.0 / time.delta_secs()),(255, 255, 255););
    let dt = time.dt();
    let Ok(ctx) = ctx.single() else {return};
    let gg = global_gravity.0;
    let ctx = Arc::new(ctx);
    let ew = Arc::new(RwLock::new(overlay_events));
    let gz = Arc::new(RwLock::new(gizmos));
    controllers.par_iter_mut().for_each(move |(e, mut v, mut c, co, go, collider, t)| {
        c.time_in_air += dt;
        c.platform_velocity = None;
        let g = if let Some(o) = go {
            o.0
        } else {
            gg
        };
        let g = if c.jumping {g * 1.0} else {g};
        let ctx = ctx.clone();
        let filter = QueryFilter::default().exclude_collider(e).exclude_sensors();
        let options = ShapeCastOptions {max_time_of_impact: 2.0 / 64.0,
            target_distance: 0.0,
            stop_at_penetration: true,
            compute_impact_geometry_on_penetration: true,
        };
        let p = t.translation.truncate();
        if v.linvel.y > -co.max_vertical_velocity {
            v.linvel += g * dt;
        }
        // let max_vel = vec2(c.max_horisontal_velocity, c.max_vertical_velocity);
        // v.linvel = v.linvel.clamp(-max_vel, max_vel);
        // let g = g.normalize();

        
        let mut col = collider.clone();
        let mut ca = col.as_capsule_mut().expect("Player must be capsule... please?...");
        ca.set_radius(ca.radius() * 0.95);
        // col.set_scale(Vec2::splat(0.95), 1);

        // floor checks
        if let Some((entity, hit)) =
            ctx.cast_shape(p, 0.0, g * dt * 2.0, 
            &col, options, filter) {
            if let Ok(p) = platforms.get(entity) {
                c.platform_velocity = Some(p.velocity);
            }
            let Some(d) = hit.details else {return;};
            if d.normal1.dot(g) < -0.7 {
                c.time_in_air = 0.0;
                c.air_jumps = co.total_air_jumps;
                c.jumping = false;
            }
        };

        // Snap-to-ground
        if let Some((entity, hit)) =
            ctx.cast_shape(p, 0.0, g * dt * 2.0, 
            &col, options, filter) {
            if let Ok(p) = platforms.get(entity) {
                c.platform_velocity = Some(p.velocity);
            }

            // overlay_text!(overlay_events;BottomLeft;FIXED_DT:format!("Fixed dt: {:.1} ({:.1} fps)", time.delta_secs(), 1.0 / time.delta_secs()),(255, 255, 255););
            
        };


        
        
    });
}


#[derive(Resource)]
pub struct SpaceWalk(pub bool);

pub fn debug(
    mut contexts: EguiContexts,
    mut controller: Single<&mut Controller, (With<Player>, Without<PlayerMesh>)>,
){
    let ctx = contexts.ctx_mut();
    egui::Window::new("A").show(ctx, |ui| {
        ui.heading("Controller");
        ui.label("");
        ui.add(egui::Slider::new(&mut controller.time_in_air, 0.0..=1.0));
    });
}

pub fn update_controllers(
    mut player: Single<(Entity, &mut Velocity, &mut Controller, &ControllerConstrants, &mut Transform), (With<Player>, Without<PlayerMesh>)>,
    mut player_mesh: Single<&mut Transform, (With<PlayerMesh>, Without<Player>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
    time: Res<Time>,
    mut cmd: Commands,
    mut animations: ResMut<PlayerAnimations>,
    
    mut spacewalk: ResMut<SpaceWalk>,
    mut mesh_turn: Local<f32>,
    mut mesh_rotation: Local<f32>,
){
    let dt = time.dt();
    let mut raw_direction = Vec2::ZERO;
    keyboard.pressed(KeyCode::KeyA).then(|| raw_direction.x -= 1.0);
    keyboard.pressed(KeyCode::KeyD).then(|| raw_direction.x += 1.0);
    keyboard.pressed(KeyCode::KeyS).then(|| raw_direction.y -= 1.0);
    keyboard.pressed(KeyCode::KeyW).then(|| raw_direction.y += 1.0);

    // let direction = raw_direction.normalize_or_zero();

    // overlay_text!(overlay_events;TopCenter;PLAYER_INPUTS:
    //     "Plyer inputs ->".to_string(),(100);
    //     format!("{:.1} {:.1}", direction.x, direction.y),(255, 100, 100);
    // );




    // animations.target = PlayerAnimationNode::Float;


    let (player_e, player_vel, controller, constrants, transform) = &mut *player;
    if keyboard.just_pressed(KeyCode::KeyC) {
        spacewalk.0 = !spacewalk.0;
        if spacewalk.0 {
            cmd.entity(*player_e).insert((
                GravityOverride(Vec2::ZERO),
                Friction::coefficient(1.0),
            ));
            cmd.entity(*player_e).insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_X);
        } else {
            cmd.entity(*player_e).remove::<GravityOverride>();
            cmd.entity(*player_e).insert((
                LockedAxes::ROTATION_LOCKED,
                REG_FRICTION,
            ));
            transform.rotation = Quat::IDENTITY;
        }
    }


    
    if spacewalk.0 {
        animations.target = PlayerAnimationNode::Float;
        let ang_dir = keyboard.pressed(KeyCode::KeyQ) as usize as f32 - keyboard.pressed(KeyCode::KeyE) as usize as f32;
        let target = player_vel.angvel + ang_dir * dt * constrants.spacewalk_ang_speed;
        if target.abs() > player_vel.angvel.abs() {
            player_vel.angvel = target.clamp(-constrants.spacewalk_max_angvel, constrants.spacewalk_max_angvel);
        } else {
            player_vel.angvel = target;
        }

        let impulse = raw_direction.rotate(transform.right().xy());
        let target = player_vel.linvel + impulse;

        if target.length_squared() > player_vel.linvel.length_squared() {
            if target.length_squared() > constrants.spacewalk_max_linvel * constrants.spacewalk_max_linvel {
                player_vel.linvel = target.normalize_or_zero() * constrants.spacewalk_max_linvel;
            } else {
                player_vel.linvel = target;
            }
        } else {
            player_vel.linvel = target;
        }
        *mesh_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        controller.horisontal_velocity = player_vel.linvel.x;
    } else {
        let target = raw_direction.x * if keyboard.pressed(KeyCode::ShiftLeft) {constrants.run_speed} else {constrants.walk_speed};

        if controller.is_on_floor() {
            controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, constrants.speed_gain * dt);
        } else {
            controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, constrants.speed_loss * dt);
        }

        // controller.horisontal_velocity += diff * *speed_gain * dt;

        player_vel.linvel.x = controller.horisontal_velocity;

        let sp = keyboard.pressed(KeyCode::Space);
        let sjp = keyboard.just_pressed(KeyCode::Space);
        if keyboard.just_pressed(KeyCode::KeyZ) {controller.jumping = false};
        
        if controller.is_on_floor() {
            if controller.horisontal_velocity > 0.0 {
                *mesh_turn = mesh_turn.move_towards(PI * 0.5, dt * constrants.mesh_turn_speed);
            } else if controller.horisontal_velocity < 0.0 {
                *mesh_turn = mesh_turn.move_towards(-PI * 0.5, dt * constrants.mesh_turn_speed);
            }
            player_mesh.rotation = Quat::from_axis_angle(Vec3::Y, *mesh_turn);
            if controller.horisontal_velocity.abs() < 10.0 {
                animations.target = PlayerAnimationNode::Idle;
            } else {
                animations.target = PlayerAnimationNode::Walk;
            }
            if sjp {
                controller.jumping = true;
                player_vel.linvel.y = constrants.jump_vel;
            }
        } else {
            animations.target = PlayerAnimationNode::Float;

            if sjp && controller.air_jumps > 0 {
                controller.air_jumps -= 1;
                controller.jumping = true;
                player_vel.linvel.y = constrants.air_jump_vel;
            }
        }
        
        if !sp || player_vel.linvel.y < 0.0 {
            controller.jumping = false;
        }
    }




    // overlay_text!(overlay_events;TopLeft;CONTROLLER:
    //     format!("{:#?}", controller),(255, 255, 255);
    // );
}

