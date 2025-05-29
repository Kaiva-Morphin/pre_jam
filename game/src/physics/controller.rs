use std::collections::HashMap;
use std::ops::DerefMut;

use bevy::ecs::query::{QueryData, WorldQuery};
use bevy::input::keyboard;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::picking::pointer::PointerLocation;
use bevy::prelude::*;


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pixel_utils::camera::PixelCamera;
use utils::wrap;

use super::platforms::{MovingPlatform, MovingPlatformMode};




pub struct ControllersPlugin;

impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_player, 
                init_scene
            ))
            .add_systems(PreUpdate, (
                tick_controllers, 
                update_controllers
            ).chain())
            .insert_resource(GlobalGravity(Vec2::new(0.0, -9.81)))
            ;
    }
}






#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut cmd: Commands,
    assets: Res<AssetServer>,

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
        Sprite::from_image(assets.load("pixel/test.png")),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Ccd::enabled(),
        Controller{
            horisontal_velocity: 0.0,
            max_horisontal_velocity: 100.0,
            total_jumps: 2,
            ..default()
        }
    ));
}

pub fn init_scene(
    mut cmd: Commands,
    assets: Res<AssetServer>,
){
    cmd.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(100.0, 5.0),
    ));
    cmd.spawn((
        RigidBody::Dynamic,
        Dominance::group(0),
        Name::new("Box"),
        // GravityScale(1.0),
        // Velocity::zero(),
        Transform::from_xyz(125.0, 25.0, 0.0),
        Collider::cuboid(10.0, 10.0),
        // Sleeping::disabled(),
        // Ccd::enabled(),

    ));
    cmd.spawn((
        Collider::cuboid(25.0, 5.0),
        MovingPlatform::bundle(
            vec![
                vec2(125.0, 0.0),
                vec2(125.0, 100.0),
                vec2(225.0, 100.0),
                vec2(225.0, 0.0),
            ],
            25.0, 
            MovingPlatformMode::Loop
        ),
    ));
}


#[derive(Resource)]
pub struct GlobalGravity(pub Vec2);
#[derive(Component)]
pub struct GravityOverride(pub Vec2);


#[derive(Component)]
pub struct Controller {
    pub horisontal_velocity: f32,
    pub jumping: bool,
    pub jumps: usize,
    pub is_on_ceiling: bool,
    pub is_on_floor: bool,


    pub max_horisontal_velocity: f32,
    pub total_jumps: usize,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            horisontal_velocity: 0.0,
            jumping: false,
            jumps: 0,
            is_on_ceiling: false,
            is_on_floor: false,
            max_horisontal_velocity: 100.0,
            total_jumps: 2
        }
    }
}

pub fn tick_controllers(
    
){

}


pub fn update_controllers(
    mut player: Single<(&mut Velocity, &mut Controller, Option<&GravityOverride>), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    global_gravity: Res<GlobalGravity>,
){
    let mut raw_direction = Vec2::ZERO;
    keyboard.pressed(KeyCode::KeyA).then(|| raw_direction.x -= 1.0);
    keyboard.pressed(KeyCode::KeyD).then(|| raw_direction.x += 1.0);
    keyboard.pressed(KeyCode::KeyS).then(|| raw_direction.y -= 1.0);
    keyboard.pressed(KeyCode::KeyW).then(|| raw_direction.y += 1.0);

    let direction = raw_direction.normalize_or_zero();
    let (player_vel, controller, grav_override) = &mut *player;
    if direction != Vec2::ZERO {
        player_vel.linvel = direction * 100.0;
    }
    let grav = if let Some(overr) = grav_override {
        overr.0
    } else {
        global_gravity.0
    };
    player_vel.linvel += grav;
}






pub trait Setting {
    fn get_name(&self) -> String {
        String::new()
    }
    fn get_description(&self) -> String {
        String::new()
    }
    fn get_value(&self) -> String {
        String::new()
    }
}

pub trait IntoSettingValue {

}

pub trait SettingBundle : Setting + Resource {}
// TODO: USE STORE INSTEAD





