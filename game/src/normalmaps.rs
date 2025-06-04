
use bevy::{prelude::*, render::render_resource::{TextureDescriptor, TextureUsages}};
use debug_utils::{debug_overlay::DebugOverlayPlugin, inspector::plugin::SwitchableEguiInspectorPlugin};

use crate::core::plugin::CorePlugin;


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

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let c = asset_server.load("tilemaps/v1.0/tilemap.png");
    let n = asset_server.load("tilemaps/v1.0/tilemap_normals.png");

    commands.spawn((
        // Sprite::from_image(c)
        // // Sprite
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            time: 0.0,
            color_texture: Some(c.clone()),
            normal_texture: Some(n.clone()),
            // _b: 0,
            // _c: 0
        })),
        // pixel_utils::camera::HIGH_RES_LAYERS,
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
    // commands.insert_resource(Hook{
    //     n_inited: false,
    //     c_inited: false,
    //     color: c,
    //     norm: n
    // });
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(C)]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    // #[uniform(0)]
    // _b: u32,
    // #[uniform(0)]
    // _c: u32,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    normal_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque //Mask(0.5)
    }
}

fn update(
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
    mut query: Query<&mut MeshMaterial2d<CustomMaterial>>,
) {
    for m in query.iter_mut() {
        let material = materials.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
        return;
    }
}