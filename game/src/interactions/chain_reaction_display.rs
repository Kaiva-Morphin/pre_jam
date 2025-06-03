use bevy::{color::palettes::css::RED, prelude::*, render::{camera::RenderTarget, render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}}, sprite::{AlphaMode2d, Material2d}};
use pixel_utils::camera::PixelCamera;

use crate::{ui::target::LowresUiContainer, utils::{custom_material_loader::SpriteAssets, debree::DebreeLevel}};

use super::{components::{InInteractionArray, InteractionTypes}, wave_modulator::WaveGraphMaterial};

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

impl UiMaterial for ChainGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        CHAINGRAPH_MATERIAL_PATH.into()
    }
}

pub fn open_chain_graph_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    images: Res<Assets<Image>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
) {
    // println!("{:?} {:?}", in_interaction_array, already_spawned);
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::ChainReactionDisplay && in_interaction_array.in_any_interaction {
            let t = images.get(&sprite_assets.chain_graph_sprite).unwrap();
            let data = t.data.clone();
            let size = t.size();
            let canvas_size = Extent3d {
                width: size.x,
                height: size.y,
                ..default()
            };
            let canvas = Image {
                texture_descriptor: TextureDescriptor {
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                    label: None,
                    size: canvas_size,
                    dimension: bevy::render::render_resource::TextureDimension::D2,
                    format: bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
                    view_formats: &[],
                    mip_level_count: 1,
                    sample_count: 1,
                },
                data,
                ..default()
            };
            let sprite_handle = asset_server.add(canvas);
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
                MaterialNode(chain_graph_material.add(
                    ChainGraphMaterial {
                        chain: 0.,
                        sprite_handle,
                        base_sprite_handle: sprite_assets.chain_graph_sprite.clone(),
                        _webgl2_padding_8b: 0,
                        _webgl2_padding_12b: 0,
                        _webgl2_padding_16b: 0,
                    })
                ),
                Node {
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    ..default()
                }
            )).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}