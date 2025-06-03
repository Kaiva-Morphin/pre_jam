use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureUsages}, view::RenderLayers}};

use crate::{interactions::{components::InteractionTypes, wave_modulator::{generate_wave_modulator_consts, WaveGraphMaterial, WaveModulatorConsts}}, utils::{custom_material_loader::SpriteAssets, debree::DebreeLevel}};

use super::{chain_reaction_display::ChainGraphMaterial, components::{InInteractionArray, InteractablesImageHandle}};

pub const UI_RENDER_LAYERS: RenderLayers = RenderLayers::layer(2);

#[derive(Resource)]
pub struct UiCameraData {
    pub ui_rendered_material_entity: Entity,
    pub ui_camera_entity: Entity,
}

pub fn spawn_ui_camera(
    mut commands: Commands,
    mut interactables_material_handle: ResMut<InteractablesImageHandle>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut ui_camera_data: ResMut<UiCameraData>,
) {
    println!("spawn_ui_camera");
    interactables_material_handle.base_image_handle = sprite_assets.chain_graph_sprite.clone();
    let t = images.get(&interactables_material_handle.base_image_handle).unwrap();
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
    let render_texure_handle = asset_server.add(canvas);
    interactables_material_handle.rendered_image_handle = render_texure_handle.clone();
    // println!("{:?} {:?} {}", interactables_material_handle.handle, Handle::<Image>::default(), interactables_material_handle.handle == Handle::<Image>::default());
    let ui_camera_entity = commands.spawn((Camera {
        target: RenderTarget::Image(render_texure_handle.clone().into()),
        ..default()
    },
    UI_RENDER_LAYERS,
    Camera2d,
    )).id();
    ui_camera_data.ui_camera_entity = ui_camera_entity;
    let material = ChainGraphMaterial {
        chain: 0.,
        sprite_handle: render_texure_handle.clone(),
        base_sprite_handle: interactables_material_handle.base_image_handle.clone(),
        _webgl2_padding_8b: 0,
        _webgl2_padding_12b: 0,
        _webgl2_padding_16b: 0,
    };
    let handle = chain_graph_material.add(material);
    let ui_rendered_material_entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200., 200.))),
        MeshMaterial2d(handle.clone()),
        Name::new("UiRenderedMaterial"),
        Transform::from_translation(Vec3::ZERO),
        UI_RENDER_LAYERS,
    )).id();
    ui_camera_data.ui_rendered_material_entity = ui_rendered_material_entity;
}

pub fn redact_ui_camera(
    mut commands: Commands,
    mut interactables_material_handle: ResMut<InteractablesImageHandle>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    mut wave_graph_material: ResMut<Assets<WaveGraphMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    in_interaction_array: Res<InInteractionArray>,
    ui_camera_data: Res<UiCameraData>,
    mut prev_interaction_type: Local<InteractionTypes>,
    mut modulator_consts: ResMut<WaveModulatorConsts>,
) {
    if *prev_interaction_type == in_interaction_array.in_interaction {
        return;
    }
    println!("new interaction {:?}", in_interaction_array.in_interaction);
    *prev_interaction_type = in_interaction_array.in_interaction.clone();
    match in_interaction_array.in_interaction {
        InteractionTypes::ChainReactionDisplay => {
            // handle for base image (screen background)
            interactables_material_handle.base_image_handle = sprite_assets.chain_graph_sprite.clone();
        },
        InteractionTypes::WaveModulator => {
            interactables_material_handle.base_image_handle = sprite_assets.chain_graph_sprite.clone();
        }
    }
    let t = images.get(&interactables_material_handle.base_image_handle).unwrap();
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
    let render_texure_handle = asset_server.add(canvas);
    interactables_material_handle.rendered_image_handle = render_texure_handle.clone();
    // println!("{:?} {:?} {}", interactables_material_handle.handle, Handle::<Image>::default(), interactables_material_handle.handle == Handle::<Image>::default());
    commands.entity(ui_camera_data.ui_camera_entity).insert((Camera {
        target: RenderTarget::Image(render_texure_handle.clone().into()),
        ..default()
    },
    UI_RENDER_LAYERS,
    Camera2d,
    ));
    commands.entity(ui_camera_data.ui_rendered_material_entity)
    .remove::<MeshMaterial2d<ChainGraphMaterial>>()
    .remove::<MeshMaterial2d<WaveGraphMaterial>>();
    match in_interaction_array.in_interaction {
        InteractionTypes::ChainReactionDisplay => {
            let material = ChainGraphMaterial {
                chain: 0.,
                sprite_handle: render_texure_handle.clone(),
                base_sprite_handle: interactables_material_handle.base_image_handle.clone(),
                _webgl2_padding_8b: 0,
                _webgl2_padding_12b: 0,
                _webgl2_padding_16b: 0,
            };
            let handle = chain_graph_material.add(material);
            commands.entity(ui_camera_data.ui_rendered_material_entity).insert((
                Mesh2d(meshes.add(Rectangle::new(size.x as f32, size.y as f32))),
                MeshMaterial2d(handle.clone()),
            ));
        },
        InteractionTypes::WaveModulator => {
            let consts = generate_wave_modulator_consts();
            modulator_consts.consts = consts.1;
            let material = WaveGraphMaterial {
                a: consts.0[0],
                b: consts.0[1],
                c: consts.0[2],
                d: consts.0[3],
                ra: consts.0[4],
                rb: consts.0[5],
                rc: consts.0[6],
                rd: consts.0[7],
                time: 0.,
                _webgl2_padding_8b: 0,
                _webgl2_padding_12b: 0,
                _webgl2_padding_16b: 0,
                sprite_handle: render_texure_handle.clone(),
                base_sprite_handle: interactables_material_handle.base_image_handle.clone(),
            };
            let handle = wave_graph_material.add(material);
            commands.entity(ui_camera_data.ui_rendered_material_entity).insert((
                Mesh2d(meshes.add(Rectangle::new(size.x as f32, size.y as f32))),
                MeshMaterial2d(handle.clone()),
            ));
        }
    }
}