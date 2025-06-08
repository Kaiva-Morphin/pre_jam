use bevy::audio::{PlaybackMode, SpatialScale, Volume};
use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, Sensor};
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

use core::plugin::CorePlugin;

use crate::core::states::OnGame;
use crate::interactions::components::PlayerSensor;
use crate::physics::constants::{INTERACTABLE_CG, PLAYER_SENSOR_CG};
use crate::physics::player::{spawn_player, Player, PlayerPlugin};
use crate::tilemap::light::LightPlugin;
use crate::tilemap::plugin::MapPlugin;
use crate::utils::background::StarBackgroundPlugin;
use crate::utils::energy::EnergyPlugin;
use crate::utils::spacial_audio::SpacialAudioPlugin;

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
            DebugOverlayPlugin::enabled(),
            EnergyPlugin,
            LightPlugin,
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
    commands.entity(*p).insert(SpatialListener::new(GAP));
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
