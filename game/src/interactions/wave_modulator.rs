use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};
use pixel_utils::camera::PixelCamera;

use super::components::{InInteractionArray, InteractablesImageHandle, InteractionTypes};


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct WaveGraphMaterial {
    #[uniform(0)]
    pub a: f32,
    #[uniform(0)]
    pub b: f32,
    #[uniform(0)]
    pub c: f32,
    #[uniform(0)]
    pub d: f32,
    #[uniform(0)]
    pub ra: f32,
    #[uniform(0)]
    pub rb: f32,
    #[uniform(0)]
    pub rc: f32,
    #[uniform(0)]
    pub rd: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub _webgl2_padding_8b: u32,
    #[uniform(0)]
    pub _webgl2_padding_12b: u32,
    #[uniform(0)]
    pub _webgl2_padding_16b: u32,
    #[texture(1)]
    #[sampler(2)]
    pub sprite_handle: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub base_sprite_handle: Handle<Image>,
}

const WAVEGRAPH_MATERIAL_PATH: &str = "shaders/wave_graph.wgsl";

impl Material2d for WaveGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVEGRAPH_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn open_wave_modulator_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    interactables_material_handle: Res<InteractablesImageHandle>,
    pc: Single<Entity, With<PixelCamera>>,
) {
    // println!("{:?} {:?}", in_interaction_array, already_spawned);
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WaveModulator && in_interaction_array.in_any_interaction {
            let entity = commands.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                UiTargetCamera(*pc)
            )).with_child((
                ImageNode {
                    image: interactables_material_handle.rendered_image_handle.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    ..default()
                }
            )).id();
            *already_spawned = Some(entity);
        }
    }
}