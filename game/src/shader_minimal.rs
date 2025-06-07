use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_tailwind::tw;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::{NoFrustumCulling, RenderLayers, VisibilitySystems}}};


use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use pixel_utils::camera::{PixelCamera, PixelCameraPlugin, PixelTarget};







fn spawn_camera(
    cmd: &mut Commands,
    target: Option<RenderTarget>,
    layer: RenderLayers,
    order: isize
){
    let t = target.unwrap_or_default();
    cmd.spawn((
        Name::new(format!("Camera {}", order)),
        Msaa::Off,
        Camera2d,
        Camera {
            target: t,
            order,
            msaa_writeback: false,
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        layer
    ));
}


pub fn generate_texture(
    assets: &mut ResMut<Assets<Image>>,
) -> Handle<Image> {
    let size = Extent3d {
        width: (32) as u32,
        height: (32) as u32,
        depth_or_array_layers: 1,
    };
    let mut t = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    t.resize(size);
    assets.add(t.clone())
}
pub fn spawn_mesh(
    cmd: &mut Commands,
    material: impl Bundle,
    meshes: &mut ResMut<Assets<Mesh>>,
){
    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        NoFrustumCulling,
        material
    ));
}

fn main() {
    let mut app : App = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            EguiPlugin { enable_multipass_for_primary_context: true },
            SwitchableEguiInspectorPlugin::default(),
            PixelCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
    ;
}



fn update(

){

}

fn setup(
    mut cmd: Commands,
    assets: Res<AssetServer>,
){
    cmd.spawn((
        Sprite::from_image(
            assets.load("pixel/arrow.png")
        ),
    ));
}

