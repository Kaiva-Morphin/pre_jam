use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;

use bevy::app::FixedMain;
use bevy::ecs::query::{QueryData, WorldQuery};
use bevy::input::keyboard;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::picking::pointer::PointerLocation;
use bevy::prelude::*;


use bevy_inspector_egui::bevy_egui::{EguiContext, EguiContexts};
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayEvent;
use debug_utils::overlay_text;
use pixel_utils::camera::PixelCamera;
use utils::{wrap, MoveTowards, WrappedDelta};

use crate::physics::scene::Player;

use super::platforms::{MovingPlatform, MovingPlatformMode};

pub struct ControllersPlugin;

impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_controllers)
            .add_systems(FixedPreUpdate, tick_controllers)
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
    pub is_on_ceiling: bool,
    pub is_on_floor: bool,
    
    pub platform_velocity: Option<Vec2>,

    pub max_horisontal_velocity: f32,
    pub max_vertical_velocity: f32,

    pub total_air_jumps: usize,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            horisontal_velocity: 0.0,
            jumping: false,
            air_jumps: 0,
            is_on_ceiling: false,
            is_on_floor: false,

            

            max_horisontal_velocity: 500.0,
            max_vertical_velocity: 500.0,
            total_air_jumps: 2,
            platform_velocity: None
        }
    }
}

pub fn tick_controllers(
    time: Res<Time>,
    ctx: ReadRapierContext,
    mut controllers: Query<(Entity, &mut Velocity, &mut Controller, Option<&GravityOverride>, &Collider, &Transform)>,
    platforms: Query<&MovingPlatform>,
    global_gravity: Res<GlobalGravity>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
){
    overlay_text!(overlay_events;TopLeft;FIXED_DT:format!("Fixed dt: {:.1} ({:.1} fps)", time.delta_secs(), 1.0 / time.delta_secs()),(255, 255, 255););
    let dt = time.dt();
    let Ok(ctx) = ctx.single() else {return};
    let gg = global_gravity.0;
    let ctx = Arc::new(ctx);
    controllers.par_iter_mut().for_each(move |(e, mut v, mut c, go, collider, t)| {
        c.is_on_ceiling = false;
        c.is_on_floor = false;
        c.platform_velocity = None;
        let g = if let Some(o) = go {
            o.0
        } else {
            gg
        };
        let g = if c.jumping {g * 0.75} else {g};
        let ctx = ctx.clone();
        let filter = QueryFilter::default().exclude_collider(e);
        let options = ShapeCastOptions {max_time_of_impact: 1.0 / 60.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: true,
        };
        let p = t.translation.truncate();
        if v.linvel.y > -c.max_vertical_velocity {
            v.linvel += g * dt;
        }
        // let max_vel = vec2(c.max_horisontal_velocity, c.max_vertical_velocity);
        // v.linvel = v.linvel.clamp(-max_vel, max_vel);
        let Some((entity, hit)) =
        ctx.cast_shape(p, 0.0, v.linvel, 
            collider, options, filter) else {return;};
        if let Ok(p) = platforms.get(entity) {
            c.platform_velocity = Some(p.velocity);
        }
        let Some(d) = hit.details else {return;};
        if d.normal1.dot(g) < 0.5 {
            c.is_on_floor = true;
            c.air_jumps = c.total_air_jumps;
            c.jumping = false;
        }
        if d.normal2.dot(g) < -0.5 {
            c.is_on_ceiling = true;
        }
    });
}

pub fn update_controllers(
    mut player: Single<(Entity, &mut Velocity, &mut Controller), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
    time: Res<Time>,
    mut acc_vel: Local<f32>,
    mut max_speed: Local<f32>,
    mut speed_gain: Local<f32>,
){
    if *max_speed <= 0. {*max_speed = 80.;}
    if *speed_gain <= 0. {*speed_gain = 700.;}
    let mut p = 1.0;

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




    let (player_e, player_vel, controller) = &mut *player;
    
    let target = raw_direction.x * *max_speed;
    if controller.is_on_floor {
        controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, *speed_gain * dt * 2.0);
    } else {
        controller.horisontal_velocity = controller.horisontal_velocity.move_towards(target, *speed_gain * dt * 0.5);
    }
    


    // controller.horisontal_velocity += diff * *speed_gain * dt;

    player_vel.linvel.x = controller.horisontal_velocity;

    let sp = keyboard.pressed(KeyCode::Space);
    let sjp = keyboard.just_pressed(KeyCode::Space);
    if keyboard.just_pressed(KeyCode::KeyZ) {controller.jumping =false};
    
    if controller.is_on_floor {
        if sjp {
            controller.jumping = true;
            player_vel.linvel.y = 200.0;
        }
    } else {
        if sjp && controller.air_jumps > 0 {
            controller.air_jumps -= 1;
            controller.jumping = true;
            player_vel.linvel.y = 150.0;
        }
    }
    
    if !sp || player_vel.linvel.y < 0.0 {
        controller.jumping = false;
    }
    overlay_text!(overlay_events;TopLeft;CONTROLLER:
        format!("{:#?}", controller),(255, 255, 255);
    );
}

