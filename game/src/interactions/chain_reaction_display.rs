use bevy::{color::palettes::css::RED, prelude::*, render::{camera::RenderTarget, render_resource::{AsBindGroup, ShaderRef}}, sprite::{AlphaMode2d, Material2d}};
use pixel_utils::camera::PixelCamera;

use crate::utils::debree::DebreeLevel;

use super::{components::{InInteractionArray, InteractablesImageHandle, InteractionTypes}, wave_modulator::WaveGraphMaterial};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct ChainGraphMaterial {
    #[uniform(0)]
    pub chain: f32,
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

const CHAINGRAPH_MATERIAL_PATH: &str = "shaders/chain_graph.wgsl";

impl Material2d for ChainGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        CHAINGRAPH_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn open_chain_graph_display(
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
        if in_interaction_array.in_interaction == InteractionTypes::ChainReactionDisplay && in_interaction_array.in_any_interaction {
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