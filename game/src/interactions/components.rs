use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};

pub const PLAYER_CG: u32 =        0b0000_0000_0000_0001;
pub const PLAYER_SENSOR_CG: u32 = 0b0000_0000_0000_0010;
pub const INTERACTABLE_CG: u32 =  0b0000_0000_0000_0100;
pub const STRUCTURES_CG: u32 =    0b0000_0000_0000_1000;

#[derive(Component)]
pub struct Interactable;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct InteractableMaterial {
    #[uniform(1)]
    pub time: f32,
    #[texture(2)]
    #[sampler(3)]
    pub sprite_handle: Handle<Image>,
}

const INTERACTABLE_MATERIAL_PATH: &str = "shaders/interact.wgsl";

impl Material2d for InteractableMaterial {
    fn fragment_shader() -> ShaderRef {
        INTERACTABLE_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Event)]
pub struct InteractGlowEvent {
    pub entity: Entity,
    pub active: bool,
}

#[derive(Component)]
pub struct InInteraction {
    pub data: bool,
}

#[derive(Component)]
pub struct InteractableKeyLink {
    pub entity: Entity,
}

#[derive(Component)]
pub struct EKey;

#[derive(Resource)]
pub struct KeyTimer {
    pub timer: Timer
}