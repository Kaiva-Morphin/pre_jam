use bevy::{prelude::*, sprite::Material2dPlugin};
use components::*;
use compute::*;

pub mod compute;
pub mod components;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ComputePlugin,
        ))
        // .add_systems(Startup, (setup, preload_sprites))
        // .add_systems(Update, (extract, spawn_sprites))
        .insert_resource(Extractor::default())
        ;
    }
}

#[derive(Component)]
pub struct VelocityEmmiter;