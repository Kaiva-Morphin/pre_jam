
use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, RenderPipelineDescriptor, ShaderSize, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}};
use debug_utils::{debug_overlay::DebugOverlayPlugin, inspector::plugin::SwitchableEguiInspectorPlugin};
use pixel_utils::camera::{TARGET_HEIGHT, TARGET_WIDTH};

use crate::{core::plugin::CorePlugin, utils::noise::{create_texture_3d}};


mod core;
mod camera;
mod utils;
mod physics;
mod interactions;
mod ui;
mod tilemap;


pub fn main(){
    let mut app = App::new();
    app
        .add_plugins((
            CorePlugin,
            DebugOverlayPlugin::default(),
            Material2dPlugin::<CustomMaterial>::default(),
            Material2dPlugin::<BlurMaterial>::default(),
            SwitchableEguiInspectorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, )) //modify_texture_descriptor
        .run()
    ;
}

use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

const MAP_LAYER: RenderLayers = RenderLayers::layer(13);
const LIGHT_RESOLUTION : f32 = 1.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
    mut blur_materials: ResMut<Assets<BlurMaterial>>,
) {
    let size = Extent3d {
        width: (TARGET_WIDTH as f32 * LIGHT_RESOLUTION) as u32,
        height: (TARGET_HEIGHT as f32 * LIGHT_RESOLUTION * 2.0) as u32,
        // 360 mlg hack for normals
        depth_or_array_layers: 1,
    };



    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("offscreen_texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                // | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    // TODO: SAVE AND LOAD
    let data = std::fs::read("R://bebra.bin").unwrap();
    let noise = create_texture_3d(&data, 128);
    let noise_handle = images.add(noise);


    let image_handle = images.add(image);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            time: 0.0,
            emitters: 0,
            color_texture: asset_server.load("pixel/scene.png"),
            normal_texture: asset_server.load("pixel/scene_normals.png"),
            mask_texture: asset_server.load("pixel/scene_mask.png"),
            noise_texture: noise_handle.clone(),
            light_texture: asset_server.load("pixel/light.png"),
            width: size.width,
            height: size.height,
            lights: [LightEmitter::default(); 64],
        })),
        MAP_LAYER,
        Transform::default().with_scale(vec3(
            size.width as f32,
            size.height as f32,
            1.0,
        )),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            target: RenderTarget::Image(image_handle.clone().into()),
            msaa_writeback: false,
            ..default()
        },
        MAP_LAYER
    ));

    let blur = blur_materials.add(BlurMaterial {
        color_texture: image_handle,
        normal_texture: asset_server.load("pixel/scene_normals.png"),
        mask_texture: asset_server.load("pixel/scene_mask.png"),
        noise_texture: noise_handle,
        light_texture: asset_server.load("pixel/light.png"),
        time: 0.0,
        width: TARGET_WIDTH,
        height: TARGET_HEIGHT,
        _b: 0,
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        
        MeshMaterial2d(blur),
        Transform::default().with_scale(vec3(TARGET_WIDTH as f32, TARGET_HEIGHT as f32, 1.0)).with_translation(Vec3::Z * 128.0),
    ));
}



#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(C)]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    width: u32,
    #[uniform(0)]
    height: u32,
    #[uniform(0)]
    emitters: u32,
    #[texture(3)]
    #[sampler(4)]
    color_texture: Handle<Image>,
    #[texture(5)]
    mask_texture: Handle<Image>,

    #[texture(6)]
    normal_texture: Handle<Image>,
    #[texture(7, dimension = "3d")]
    #[sampler(8)]
    noise_texture: Handle<Image>,
    #[texture(9)]
    light_texture: Handle<Image>,
    #[uniform(10)]
    lights: [LightEmitter; 64],
}

#[derive(ShaderType, Debug, Clone, Copy)]
pub struct LightEmitter {
    pub camera_relative_position: Vec2,
    pub radius_px: f32,
    pub ang: f32,
    pub color: Vec4,
}

impl Default for LightEmitter {
    fn default() -> Self {
        Self {
            camera_relative_position: Vec2::ZERO,
            radius_px: 0.0,
            ang: 0.0,
            color: Vec4::ZERO,
        }
    }
}




impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/light_sources.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(C)]
struct BlurMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    width: u32,
    #[uniform(0)]
    height: u32,
    #[uniform(0)]
    _b: u32,
    #[texture(3)]
    #[sampler(4)]
    color_texture: Handle<Image>,
    #[texture(5)]
    mask_texture: Handle<Image>,
    #[texture(6)]
    normal_texture: Handle<Image>,
    #[texture(7, dimension = "3d")]
    #[sampler(8)]
    noise_texture: Handle<Image>,
    #[texture(9)]
    light_texture: Handle<Image>,
}

impl Material2d for BlurMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/blur.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}


fn update(
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut mat2: ResMut<Assets<BlurMaterial>>,
    time: Res<Time>,
    mut query: Query<&mut MeshMaterial2d<CustomMaterial>>,
    mut q: Query<&mut MeshMaterial2d<BlurMaterial>>,
) {
    for m in query.iter_mut() {
        let material = materials.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
        material.emitters = 2;
        material.lights[0] = LightEmitter {
            camera_relative_position: Vec2::new(-100.0, 10.0),
            radius_px: 250.0,
            ang: 0.0,
            color: vec4(0.0, 0.0, 1.0, 1.0)
        };
        material.lights[1] = LightEmitter {
            camera_relative_position: Vec2::new(100.0, 10.0),
            radius_px: 250.0,
            ang: 0.0,
            color: vec4(1.0, 0.0, 1.0, 1.0)
        };
    }
    for m in q.iter_mut() {
        let material = mat2.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
    }
}