use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, Sensor};
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

use core::plugin::CorePlugin;

use crate::core::states::OnGame;
use crate::physics::constants::{INTERACTABLE_CG, PLAYER_SENSOR_CG};
use crate::physics::player::{spawn_player, Player, PlayerPlugin};
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
            MapPlugin,
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::default(),
        ))
        .add_systems(OnGame, spawn.after(spawn_player))  //.before(shaders::compute::setup)
        .run();
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
    
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/173273__tomlija__janitors-bedroom-ambience.wav")),
        PlaybackSettings::LOOP.with_spatial(true),
        Transform::from_xyz(50., 0., 0.),
        Sprite::from_color(Color::Srgba(GREEN), Vec2::splat(20.0)),
    ));

}
