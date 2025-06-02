use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureUsages}, view::RenderLayers}};

use crate::utils::{custom_material_loader::SpriteAssets, debree::DebreeLevel};

use super::{chain_reaction_display::ChainGraphMaterial, components::InteractablesImageHandle};

pub const UI_RENDER_LAYERS: RenderLayers = RenderLayers::layer(2);

pub fn spawn_ui_camera(
    mut commands: Commands,
    mut interactables_material_handle: ResMut<InteractablesImageHandle>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
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
    commands.spawn((Camera {
        target: RenderTarget::Image(render_texure_handle.clone().into()),
        ..default()
    },
    UI_RENDER_LAYERS,
    Camera2d,
    ));
    let material = ChainGraphMaterial {
        chain: 0.,
        sprite_handle: render_texure_handle.clone(),
        base_sprite_handle: interactables_material_handle.base_image_handle.clone(),
        _webgl2_padding_8b: 0,
        _webgl2_padding_12b: 0,
        _webgl2_padding_16b: 0,
    };
    let handle = chain_graph_material.add(material);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200., 200.))),
        MeshMaterial2d(handle.clone()),
        Name::new("UiRenderedMaterial"),
        Transform::from_translation(Vec3::ZERO),
        UI_RENDER_LAYERS,
    ));
}