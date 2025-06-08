use std::collections::VecDeque;

use bevy::{color::palettes::css::RED, prelude::*, render::{camera::RenderTarget, render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}}, sprite::{AlphaMode2d, Material2d}};
use bevy_tailwind::tw;
use pixel_utils::camera::PixelCamera;

use crate::{ui::{components::containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::{text_display_green_handle, ui_text_display_green_with_text}, viewport_container::{ui_viewport_container, viewport_handle}}, target::LowresUiContainer}, utils::{custom_material_loader::SpriteAssets, debree::DebreeLevel, spacial_audio::PlaySoundEvent}};

use super::{components::{InInteractionArray, InteractionTypes}, wave_modulator::WaveGraphMaterial};

pub const CHAIN_GRAPH_LENGTH: usize = 10;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct ChainGraphMaterial {
    #[uniform(0)]
    pub chain: [Vec4; CHAIN_GRAPH_LENGTH],
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

#[derive(Component)]
pub struct ChainDisplayText;

pub fn open_chain_graph_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    images: Res<Assets<Image>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    // println!("{:?} {:?}", in_interaction_array, already_spawned);
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::ChainReactionDisplay && in_interaction_array.in_any_interaction {
            event_writer.write(PlaySoundEvent::OpenUi);
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
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            let view = viewport_handle(&asset_server);
            let text_bundle = text_display_green_handle(&asset_server);
            let material = MaterialNode(chain_graph_material.add(
                ChainGraphMaterial {
                    chain: [Vec4::ZERO; CHAIN_GRAPH_LENGTH],
                    sprite_handle,
                    base_sprite_handle: sprite_assets.chain_graph_sprite.clone(),
                })
            );
            let ui_entity = commands.spawn(
            ui_main_container(&main, children![(
                ui_viewport_container(&view, 
                    children![(
                        material,
                        tw!("z-10 w-[128px] h-[128px]")
                )]),)])
            ).id();
            let display_text = "Chain Reaction Progress 000 %";
            let text_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_text_display_green_with_text(&text_bundle, (ChainDisplayText, ChainDisplayText), display_text, &asset_server)
                ])
            ).id();

            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ()))
                .with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(ui_entity);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(text_entity);
                    });
                });
            }).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub fn update_chain_graph_display(
    text: Query<&mut Text, With<ChainDisplayText>>,
    debree_level: Res<DebreeLevel>,
    chain_material_handle: Query<&MaterialNode<ChainGraphMaterial>>,
    mut chain_material_assets: ResMut<Assets<ChainGraphMaterial>>,

) {
    for mut text in text {
        let len = (debree_level.chain_reaction as i32).to_string().len();
        text.0 = format!("Chain Reaction Progress {}{} %", "0".repeat(3 - len), debree_level.chain_reaction as i32);
    }
    if let Ok(chain_material_handle) = chain_material_handle.single() {
        if let Some(material) = chain_material_assets.get_mut(chain_material_handle) {
            let mut new = [Vec4::ZERO; 10];
            for (idx, val) in debree_level.chain_reaction_graph.iter().enumerate() {
                let vec_idx = idx / 4;
                let elem_idx = idx % 4;
                new[vec_idx][elem_idx] = *val;
            }
            material.chain = new;
        }
    }
}