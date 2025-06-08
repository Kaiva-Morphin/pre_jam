use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::ecs::query;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, Sensor};
use debug_utils::debug_overlay::{DebugOverlayEvent, DebugOverlayPlugin};
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::overlay_text;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};

use core::plugin::CorePlugin;

use crate::core::states::OnGame;
use crate::interactions::components::PlayerSensor;
use crate::physics::constants::{INTERACTABLE_CG, PLAYER_SENSOR_CG};
use crate::physics::player::{spawn_player, Player, PlayerPlugin};
use crate::tilemap::light::LightPlugin;
use crate::tilemap::plugin::MapPlugin;
use crate::utils::background::StarBackgroundPlugin;

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
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::default(),
        ))
        .add_systems(OnGame, spawn.after(spawn_player))
        .add_systems(Update, update)
        .run();
}


#[derive(Component)]
pub struct ToWorld;

pub fn update(
    mut cmd: Commands,
    q: Query<&mut GlobalTransform, Without<PixelCamera>>,
    e: Option<Single<Entity, With<ToWorld>>>,
    windows: Single<&Window>,
    v: Res<pixel_utils::camera::PixelCameraVars>,
    cq: Single<(&Camera, &GlobalTransform), With<PixelCamera>>,
    asset_server: Res<AssetServer>,
    mut dbg: EventWriter<DebugOverlayEvent>
){
    let i = asset_server.load("pixel/arrow.png");

    let Some(e) = e else {
        cmd.spawn((
            Sprite::from_image(i),
            ToWorld,
            GlobalTransform::default()
        ));
        return;
    };
    let t = q.get(*e).unwrap();
    let (camera, camera_transform) = *cq;
    let window = *windows;
    let window_size = window.size();
    let target_size = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera.viewport_to_world_2d(camera_transform, screen_position).unwrap();
        let s = screen_position;
        
        let us = s / window_size - 0.5 ;
        let up = (s - window_size * 0.5) / (target_size * v.scale());
        overlay_text!(dbg ;BottomLeft;1;PX:format!(
            "UV screen: {us}\nUV pixel: {up}",
        ),(255));

        let h_scale = window_size.x / TARGET_WIDTH as f32;
        let v_scale = window_size.y / TARGET_HEIGHT as f32;

        let mut pos = camera_transform.translation().truncate();
        pos += up * target_size * vec2(1.0, -1.0);
        // let w =   * target_size * v.scale() * 0.5 + target_size * v.scale() * 0.5 + camera_transform.translation().truncate();
        // let sc = (screen_position - window_size * 0.5);
        // let su = sc / (window_size * 0.5);
        // let PU: Vec2 = su * ((window_size - target_size) / target_size);
        // let pu = su * (target_size * 0.5) * vec2(1.0, -1.0) + camera_transform.translation().truncate();
        
        
        cmd.entity(*e).insert((
            Transform::from_translation(pos.extend(0.)),
        ));
        // info!("S: {}", s);
    }
}





pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    p: Single<Entity, With<Player>>,
) {
    const GAP: f32 = 50.;
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
        // left ear
        cmd.spawn((
            Sprite::from_color(Color::Srgba(RED), Vec2::splat(20.0)),
            Transform::from_xyz(-GAP / 2., 0.0, 0.0),
        ));
        // right ear
        cmd.spawn((
            Sprite::from_color(Color::Srgba(BLUE), Vec2::splat(20.0)),
            Transform::from_xyz(GAP / 2., 0.0, 0.0),
        ));
    });
    
    // commands.spawn((
    //     AudioPlayer::new(asset_server.load("sounds/173273__tomlija__janitors-bedroom-ambience.wav")),
    //     PlaybackSettings::LOOP.with_spatial(true),
    //     Transform::from_xyz(50., 0., 0.),
    //     Sprite::from_color(Color::Srgba(GREEN), Vec2::splat(20.0)),
    // ));

}
