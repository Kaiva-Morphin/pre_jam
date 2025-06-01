use std::collections::HashMap;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_graph::RenderLabel, render_resource::{AsBindGroup, BindGroup, BindGroupLayout, Buffer, CachedComputePipelineId, ShaderRef}}, sprite::{AlphaMode2d, Material2d}};

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ComputeLabel;

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct Extractor {
    pub player_pos: Vec2,
    pub player_vel: Vec2,
}

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct VelocityBufferHandles {
    pub material_handle: Handle<VelocityBufferMaterial>,
    pub buffer_handle: Handle<Image>,
    pub debug_handle: Handle<Image>,
}

#[derive(Resource)]
pub struct ComputeImageBindGroups {
    pub bind_groups: Vec<BindGroup>
}

#[derive(Resource)]
pub struct ComputePipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

#[derive(Resource)]
pub struct PlayerPosBuffer {
    pub buffer: Buffer
}

#[derive(Resource)]
pub struct ParameterBuffer {
    pub buffer: Buffer
}

pub enum ComputeState {
    Loading,
    Init,
    Update(usize),
}

pub struct ComputeNode {
    pub state: ComputeState,
}

impl Default for ComputeNode {
    fn default() -> Self {
        Self {
            state: ComputeState::Loading,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct VelocityBufferMaterial {
    #[uniform(1)]
    pub screen_size: Vec2,
    #[uniform(2)]
    pub player_pos: Vec2,
    #[texture(3)]
    #[sampler(4)]
    pub buffer_handle: Handle<Image>,
    #[texture(5)]
    #[sampler(6)]
    pub debug_handle: Handle<Image>,
}

const SHADER_ASSET_PATH: &str = "shaders/velocity_buffer.wgsl";

impl Material2d for VelocityBufferMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GrassMaterial {
    #[uniform(1)]
    pub pos: Vec2,
    #[texture(2)]
    #[sampler(3)]
    pub sprite_handle: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub velbuf_handle: Handle<Image>,
    #[uniform(6)]
    pub time: f32,
}

const GRASS_MATERIAL_PATH: &str = "shaders/touch_grass.wgsl";

impl Material2d for GrassMaterial {
    fn fragment_shader() -> ShaderRef {
        GRASS_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}