use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, Sensor};
use debug_utils::debug_overlay::{DebugOverlayEvent, DebugOverlayPlugin};
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};

use core::plugin::CorePlugin;
use std::collections::HashMap;

use crate::core::states::OnGame;
use crate::interactions::components::PlayerSensor;
use crate::physics::constants::{INTERACTABLE_CG, PLAYER_SENSOR_CG};
use crate::physics::player::{spawn_player, Player, PlayerPlugin};
use crate::tilemap::light::{LightEmitter, LightPlugin, LIT_OVERLAY_LAYER};
use crate::tilemap::plugin::MapPlugin;
use crate::utils::background::StarBackgroundPlugin;
use crate::utils::energy::EnergyPlugin;

mod core;
mod ui;
mod camera;
mod utils;
mod physics;
mod interactions;
mod tilemap;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            CorePlugin,
            StarBackgroundPlugin,
            PlayerPlugin,
            LightPlugin,
            MapPlugin,
            EnergyPlugin,
            // SwitchableEguiInspectorPlugin::default(),
            // SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::default(),
        ))
        .add_systems(OnGame, spawn.after(spawn_player))
        .add_systems(Startup, setup)
        .add_systems(OnGame, lights_setup)
        .add_systems(Update, (light_flickering, set_volume))
        .insert_resource(LightFlicker::default())
        // .add_systems(Update, update)
        .run();
}

pub struct DefaultVolume(Volume);

impl Default for DefaultVolume {
    fn default() -> Self {
        DefaultVolume(Volume::default() - Volume::Linear(0.5))
    }
}

fn set_volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut music_controller: Query<&mut AudioSink>,
    mut v: Local<DefaultVolume>
) {
    let Ok(mut sink) = music_controller.single_mut() else {
        return;
    };
    if keyboard_input.just_pressed(KeyCode::Equal) {
        v.0 = v.0 + Volume::Linear(0.1);
        sink.set_volume(v.0);
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        v.0 = v.0 - Volume::Linear(0.1);
        sink.set_volume(v.0);
    }
}


#[derive(Default, Resource)]
pub struct LightFlicker {
    emitters: HashMap<Entity, f32>
}

pub fn lights_setup(
    mut emitters: Query<(Entity, &LightEmitter)>,
    mut fl : ResMut<LightFlicker>
){
    for (e, em) in emitters.iter_mut() {
        // store defaults
        fl.emitters.insert(e, em.intensity);
    }
}

pub fn light_flickering(
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut LightEmitter, &Transform)>,
    fl : ResMut<LightFlicker>
) {
    let t = time.elapsed_secs();
    for (e, mut emitter, transform) in emitters.iter_mut() {
        if let Some(base_intensity) = fl.emitters.get(&e) {
            let pos = transform.translation;
            let noise = (pos.x.sin() * 13.37 + pos.y.cos() * 42.0 + t * 5.0).sin();
            let flicker = 0.8 + 0.2 * noise;
            emitter.intensity = base_intensity * flicker;
        }
    }
}

// #[derive(Component)]
// pub struct ToWorld;

// pub fn update(
//     mut cmd: Commands,
//     q: Query<&mut GlobalTransform, (Without<PixelCamera>, Without<Player>)>,
//     e: Option<Single<Entity, With<ToWorld>>>,
//     windows: Single<&Window>,
//     v: Res<pixel_utils::camera::PixelCameraVars>,
//     cq: Single<(&Camera, &GlobalTransform), With<PixelCamera>>,
//     asset_server: Res<AssetServer>,
//     mut dbg: EventWriter<DebugOverlayEvent>,
//     p: Single<&GlobalTransform, With<Player>>
// ){
//     let i = asset_server.load("pixel/arrow.png");

//     let Some(e) = e else {
//         cmd.spawn((
//             Sprite::from_image(i),
//             ToWorld,
//             GlobalTransform::default()
//         ));
//         return;
//     };
//     let t = q.get(*e).unwrap();
//     let (camera, camera_transform) = *cq;
//     let window = *windows;
//     let window_size = window.size();
//     let target_size = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
//     if let Some(screen_position) = window.cursor_position() {
//         let world_position = camera.viewport_to_world_2d(camera_transform, screen_position).unwrap();
//         let s = screen_position;
        
//         let us = s / window_size - 0.5 ;
//         let up = (s - window_size * 0.5) / (target_size * v.scale());

//         let h_scale = window_size.x / TARGET_WIDTH as f32;
//         let v_scale = window_size.y / TARGET_HEIGHT as f32;

//         let mut pos = camera_transform.translation().truncate();
//         pos += up * target_size * vec2(1.0, -1.0);
//         // let w =   * target_size * v.scale() * 0.5 + target_size * v.scale() * 0.5 + camera_transform.translation().truncate();
//         // let sc = (screen_position - window_size * 0.5);
//         // let su = sc / (window_size * 0.5);
//         // let PU: Vec2 = su * ((window_size - target_size) / target_size);
//         // let pu = su * (target_size * 0.5) * vec2(1.0, -1.0) + camera_transform.translation().truncate();
        
//         let rp = (p.translation().truncate() - pos).to_angle();
//         cmd.entity(*e).insert((
//             LIT_OVERLAY_LAYER,
//             Transform::from_translation(pos.extend(0.)).with_rotation(Quat::from_rotation_z(rp)),
//         ));
//         // info!("S: {}", s);
//     }
// }





pub fn setup(
    mut cmd: Commands,
    cam: Single<Entity, With<PixelCamera>>,
){
    cmd.entity(*cam).insert(Transform::from_translation(vec3(774.582, 80.0, 0.0)));
}

pub fn spawn(
    mut commands: Commands,
    p: Single<Entity, With<Player>>,
) {
    const GAP: f32 = 50.;
    commands.entity(*p).insert((
        SpatialListener::new(GAP),
            // Transform::from_translation(vec3(0.0, 0.0, 0.0))
        Transform::from_translation(vec3(774.582, 80.0, 0.0))
    ));
    commands.entity(*p).with_children(|cmd|{
        cmd.spawn((
            Name::new("Player sensor"),
            Collider::ball(30.),
            CollisionGroups::new(
                Group::from_bits(PLAYER_SENSOR_CG).unwrap(),
                Group::from_bits(INTERACTABLE_CG).unwrap(),
            ),
            Sensor,
            PlayerSensor,
        ));
    });
}
