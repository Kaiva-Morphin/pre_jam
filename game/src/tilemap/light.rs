use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::{NoFrustumCulling, RenderLayers, VisibilitySystems}}};

use bevy_ecs_tiled::prelude::{TiledMapLayer, TiledMapTile};
use bevy_ecs_tilemap::map::TilemapRenderSettings;
use bevy_tailwind::tw;
use pixel_utils::camera::{PixelCamera, PixelTarget, TARGET_HEIGHT, TARGET_WIDTH};

use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::utils::noise::get_noise_3d;

pub struct LightPlugin;


impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                Material2dPlugin::<LightMaterial>::default(),
                Material2dPlugin::<CompositorMaterial>::default(),
            ))
            .register_type::<LightOccluderLayer>()
            .add_systems(PostUpdate, sync_camera)
            .add_systems(PostUpdate, (update, watcher).after(VisibilitySystems::CheckVisibility)) //modify_texture_descriptor
            .add_systems(Startup, setup)
            ;
    }
}

#[derive(Component)]
pub struct SceneOccluderCamera;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct LightOccluderLayer;



#[derive(Component, Debug, Default, Reflect)]
#[derive(ShaderType, Clone, Copy)]
pub struct LightEmitter {
    pub radius_px: f32,
    pub spot: f32,
    pub rotation: f32,
    pub color: Vec3,
}

#[derive(ShaderType, Debug, Clone, Copy)]
pub struct RelativeLightEmitter {
    pub camera_relative_position: Vec2,
    pub emitter: LightEmitter,
}

impl Default for RelativeLightEmitter {
    fn default() -> Self {
        Self {
            camera_relative_position: Vec2::ZERO,
            emitter: LightEmitter::default(),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(C)]
struct LightMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    width: u32,
    #[uniform(0)]
    height: u32,
    #[uniform(0)]
    emitters: u32,
    #[uniform(1)]
    lights: [RelativeLightEmitter; 64],
    #[sampler(2)]
    #[texture(3)]
    scene_texture: Handle<Image>,
    #[sampler(4)]
    #[texture(5, dimension = "3d")]
    noise_texture: Handle<Image>,
}



impl Material2d for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/light.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }


}




#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(C)]
struct CompositorMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    width: u32,
    #[uniform(0)]
    height: u32,
    #[uniform(0)]
    _b: u32,
    #[sampler(1)]
    #[texture(2)]
    light_texture: Handle<Image>,

    #[sampler(3)]
    #[texture(4)]
    scene_texture: Handle<Image>,

    #[sampler(5)]
    #[texture(6, dimension = "3d")]
    noise_texture: Handle<Image>,
}

impl Material2d for CompositorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap/compositor.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}


const COMPOSITOR_LAYER: RenderLayers = RenderLayers::layer(10);
const SCENE_OCCLUDER_LAYER: RenderLayers = RenderLayers::layer(20);
const LIGHT_LAYER: RenderLayers = RenderLayers::layer(30);
const LIGHT_RESOLUTION : f32 = 1.0;

// todo:
const LIGHT_OFFSET : f32 = 0.0;


#[derive(Component)]
struct CompositorCamera;


fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<LightMaterial>>,
    mut compositor_material: ResMut<Assets<CompositorMaterial>>,
    pixel_target: Res<PixelTarget>,
) {
    let size = Extent3d {
        width: (TARGET_WIDTH as f32 * LIGHT_RESOLUTION) as u32,
        height: (TARGET_HEIGHT as f32 * LIGHT_RESOLUTION) as u32,
        depth_or_array_layers: 1,
    };

    let mut scene_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("occluder_scene_texture"),
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
    scene_texture.resize(size);
    let scene_texture_handle: Handle<Image> = images.add(scene_texture.clone());
    cmd.spawn((
        Name::new("Scene Occluder Camera"),
        SceneOccluderCamera,
        Camera2d,
        Camera {
            target: RenderTarget::Image(scene_texture_handle.clone().into()),
            order: -19,
            msaa_writeback: false,
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        SCENE_OCCLUDER_LAYER
    ));
    
    let mut light_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("light_texture"),
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
    light_texture.resize(size);
    let light_handle: Handle<Image> = images.add(light_texture.clone());
    cmd.spawn((
        Name::new("Light Camera"),
        Camera2d,
        Camera {
            target: RenderTarget::Image(light_handle.clone().into()),
            order: -18,
            msaa_writeback: false,
            ..default()
        },
        LIGHT_LAYER
    ));

    let noise = get_noise_3d();
    let noise_handle = images.add(noise);

    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        NoFrustumCulling,
        MeshMaterial2d(materials.add(LightMaterial {
            time: 0.0,
            scene_texture: scene_texture_handle.clone(),
            emitters: 0,
            noise_texture: noise_handle.clone(),
            width: size.width,
            height: size.height,
            lights: [RelativeLightEmitter::default(); 64],
        })),
        LIGHT_LAYER,
        Transform::default().with_scale(vec3(
            size.width as f32,
            size.height as f32,
            1.0,
        )),
    ));

    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        NoFrustumCulling,
        MeshMaterial2d(compositor_material.add(CompositorMaterial {
            time: 0.0,
            width: TARGET_WIDTH,
            height: TARGET_HEIGHT,
            _b: 0,
            light_texture: light_handle.clone(),
            scene_texture: scene_texture_handle.clone(),
            noise_texture: noise_handle.clone(),
        })),
        COMPOSITOR_LAYER,
        Transform::default().with_scale(vec3(TARGET_WIDTH as f32, TARGET_HEIGHT as f32, 1.0)).with_translation(Vec3::Z * 128.0),
    ));

    cmd.spawn((
        CompositorCamera,
        Camera2d,
        Msaa::Off,
        Camera {
            target: RenderTarget::Image(pixel_target.image.clone().into()),
            clear_color: ClearColorConfig::None,
            order: -4,
            ..default()
        },
        COMPOSITOR_LAYER
    ));
}




fn update(
    mut light_mat: ResMut<Assets<LightMaterial>>,
    mut comp_mat: ResMut<Assets<CompositorMaterial>>,
    time: Res<Time>,

    emitters: Query<&LightEmitter>,


    mut light: Query<&mut MeshMaterial2d<LightMaterial>>,
    mut compositor: Query<&mut MeshMaterial2d<CompositorMaterial>>,
) {
    for m in light.iter_mut() {
        let material = light_mat.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
        material.emitters = 0;
        // material.lights[0] = LightEmitter {
        //     camera_relative_position: Vec2::new(-100.0, 10.0),
        //     radius_px: 250.0,
        //     spot: 0.0,
        //     color: vec3(0.8, 0.4, 0.6),
        //     ..default()
        // };
        // material.lights[1] = LightEmitter {
        //     camera_relative_position: Vec2::new(100.0, 10.0),
        //     radius_px: 250.0,
        //     spot: 0.0,
        //     color: vec3(0.2, 0.2, 0.2),
        //     ..default()
        // };
    }
    for m in compositor.iter_mut() {
        let material = comp_mat.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
    }
}

pub fn sync_camera(
    mut co: Single<(&mut Transform, &mut Projection), (With<SceneOccluderCamera>, Without<PixelCamera>)>,
    c2d: Single<(&Transform, &Projection), (With<PixelCamera>, Without<SceneOccluderCamera>)>,
) {
    *co.0 = *c2d.0;
    *co.1 = c2d.1.clone();
}



pub fn watcher(
    q: Query<(Entity, &Children), (Without<RenderLayers>, With<LightOccluderLayer>)>,
    // c: Query<&Children, Or<(With<TiledMapTile>, With<TiledMapLayer>)>>,
    mut cmd: Commands,
){
    for (e, c) in q {
        cmd.entity(e).insert(SCENE_OCCLUDER_LAYER);
        for e in c {
            cmd.entity(*e).insert(SCENE_OCCLUDER_LAYER);
        }
        info!("Occluder layer injected!");
    }
}
