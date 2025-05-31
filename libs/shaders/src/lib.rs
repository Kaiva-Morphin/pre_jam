use bevy::{prelude::*, sprite::Material2dPlugin};
use components::*;
use compute::*;

mod compute;
mod components;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ComputePlugin,
            Material2dPlugin::<VelocityBufferMaterial>::default(),
            Material2dPlugin::<GrassMaterial>::default(),
        ))
        .add_event::<SpritePreloadEvent>()
        .add_systems(Startup, ((spawn_player, setup).chain(), preload_sprites))
        .add_systems(Update, (player_controller, extract_player_pos, draw, spawn_sprites))
        .insert_resource(VelocityBufferHandles::default())
        .insert_resource(Extractor::default())
        ;
    }
}
