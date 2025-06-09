use std::{collections::HashMap, f32::consts::PI};

use bevy::{prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, ShaderType, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::{NoFrustumCulling, RenderLayers, VisibilitySystems}}};

use bevy_ecs_tiled::prelude::{TiledMapLayer, TiledMapTile};
use bevy_ecs_tilemap::map::TilemapRenderSettings;
use bevy_tailwind::tw;
use pixel_utils::camera::{PixelCamera, PixelCamera3d, PixelTarget, PIXEL_PERFECT_LAYERS, TARGET_HEIGHT, TARGET_WIDTH};

use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use tiled::PropertyValue;

use crate::utils::{background::ParalaxLayer, noise::get_noise_3d};

pub struct LightPlugin;


impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                Material2dPlugin::<LightMaterial>::default(),
                Material2dPlugin::<CompositorMaterial>::default(),
            ))
            .register_type::<LightOccluderLayer>()
            .add_systems(PostUpdate, sync_cameras)
            .add_systems(PostUpdate, (update, watcher).after(VisibilitySystems::CheckVisibility)) //modify_texture_descriptor
            .add_systems(Startup, setup)
            ;
    }
}

#[derive(Component)]
pub struct SyncCamera;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct LightOccluderLayer;

#[derive(Component)]
pub struct Unshaded;

impl LightEmitter {
    pub fn from_properties(properties: &HashMap<String, PropertyValue>) -> Option<Self> {
        let Some(PropertyValue::FloatValue(radius_px)) = properties.get("radius") else {return None};
        let Some(PropertyValue::FloatValue(spot)) = properties.get("angle") else {return None};
        let Some(PropertyValue::FloatValue(rotation)) = properties.get("rotation") else {return None};
        let Some(PropertyValue::FloatValue(intensity)) = properties.get("intensity") else {return None};
        let Some(PropertyValue::ColorValue(color)) = properties.get("color") else {return None};

        Some(Self{
            radius_px: *radius_px,
            spot: *spot,
            color_and_rotation: vec4(color.red as f32 / 255., color.green as f32 / 255., color.blue as f32 / 255., *rotation),
            intensity: *intensity,
        })
    }
    fn to_emitter(&self, relative_to_cam: Vec2) -> RelativeLightEmitter {
        RelativeLightEmitter {
            camera_relative_position: relative_to_cam,
            radius: self.radius_px,
            spot: self.spot,
            color_and_rotation: self.color_and_rotation,
            intensity: self.intensity,
            _padding: 0.0,
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[derive(ShaderType, Clone, Copy)]
#[reflect(Component, Default)]
pub struct LightEmitter {
    pub radius_px: f32,
    pub spot: f32,
    pub color_and_rotation: Vec4,
    pub intensity: f32,
}



#[derive(ShaderType, Debug, Clone, Copy)]
#[repr(C)]
pub struct RelativeLightEmitter {
    pub camera_relative_position: Vec2,
    pub radius: f32,
    pub spot: f32,
    pub color_and_rotation: Vec4,
    pub intensity: f32,
    pub _padding: f32,
}

impl Default for RelativeLightEmitter {
    fn default() -> Self {
        Self {
            camera_relative_position: Vec2::ZERO,
            radius: 0.0,
            spot: 0.0,
            color_and_rotation: Vec4::ZERO,
            intensity: 0.0,
            _padding: 0.0,
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
    lights: [RelativeLightEmitter; MAX_EMITTERS],
    #[sampler(2)]
    #[texture(3)]
    occluders_texture: Handle<Image>,
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
    rotation_z: f32,
    #[uniform(0)]
    position_x: f32,
    #[uniform(0)]
    position_y: f32,
    #[uniform(0)]
    debree_inf: f32,
    #[uniform(0)]
    _pad: u32,
    
    #[sampler(1)]
    #[texture(2)]
    light_texture: Handle<Image>,

    #[sampler(3)]
    #[texture(4)]
    occluders_texture: Handle<Image>,

    #[sampler(5)]
    #[texture(6)]
    scene_texture: Handle<Image>,

    #[sampler(7)]
    #[texture(8, dimension = "3d")]
    noise_texture: Handle<Image>,

    #[sampler(9)]
    #[texture(10)]
    bg_texture: Handle<Image>,

    #[sampler(11)]
    #[texture(12)]
    lit_texture: Handle<Image>,
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
const BG_LAYER : RenderLayers = RenderLayers::layer(24);
pub const LIT_OVERLAY_LAYER : RenderLayers = RenderLayers::layer(26);
const LIGHT_RESOLUTION : f32 = 1.0;


// ASLO PASS IN SHADER!
const MAX_EMITTERS: usize = 64;

#[derive(Component)]
struct CompositorCamera;


fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<LightMaterial>>,
    mut compositor_material: ResMut<Assets<CompositorMaterial>>,
    mut pixel_camera3d: Single<(Entity, &mut Camera), (With<PixelCamera>, Without<PixelCamera3d>)>,
    mut pixel_camera: Single<(Entity, &mut Camera), (With<PixelCamera3d>, Without<PixelCamera>)>,
    pixel_target: Res<PixelTarget>,
    paralax_layers: Query<Entity, With<ParalaxLayer>>
) {
    
    let size = Extent3d {
        width: (TARGET_WIDTH as f32 * LIGHT_RESOLUTION) as u32,
        height: (TARGET_HEIGHT as f32 * LIGHT_RESOLUTION) as u32,
        ..default()
        // depth_or_array_layers: 1,
    };

    let mut lit_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("lit_scene_texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    lit_texture.resize(size);
    let lit_texture_handle: Handle<Image> = images.add(lit_texture);
    cmd.spawn((
        Name::new("Lit Camera"),
        SyncCamera,
        Camera2d,
        Camera {
            target: RenderTarget::Image(lit_texture_handle.clone().into()),
            order: -17,
            msaa_writeback: false,
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        LIT_OVERLAY_LAYER
    ));


    let mut scene_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("occluder_scene_texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    scene_texture.resize(size);
    let occluders_texture_handle: Handle<Image> = images.add(scene_texture);

    let retarget_size = Extent3d {
        width: TARGET_WIDTH,
        height: TARGET_HEIGHT,
        ..default()
    };
    let mut retarget_canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: retarget_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    retarget_canvas.resize(retarget_size);
    let retarget_handle: Handle<Image> = images.add(retarget_canvas);


    let bg_size = Extent3d {
        width: TARGET_WIDTH,
        height: TARGET_HEIGHT,
        ..default()
    };
    let mut bg_canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: bg_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    bg_canvas.resize(bg_size);
    let bg_handle: Handle<Image> = images.add(bg_canvas);

    for e in paralax_layers {
        cmd.entity(e).insert(BG_LAYER);
    }


    cmd.spawn((
        Name::new("Scene Occluder Camera"),
        SyncCamera,
        Camera2d,
        Camera {
            target: RenderTarget::Image(occluders_texture_handle.clone().into()),
            order: -19,
            msaa_writeback: false,
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        SCENE_OCCLUDER_LAYER
    ));

    let (_e, pc3d) = &mut *pixel_camera3d;
    pc3d.target = RenderTarget::Image(retarget_handle.clone().into());
    // pc3d.target = RenderTarget::Image(occluders_texture_handle.clone().into());
    // cmd.entity(*_e).insert(Msaa::Off);

    let (pixel_camera_e, pixel_camera) = &mut *pixel_camera;
    pixel_camera.target = RenderTarget::Image(retarget_handle.clone().into());

    

    cmd.entity(*pixel_camera_e).with_children(|cmd|{
        cmd.spawn((
            Camera2d,
            Camera {
                order: -21,
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                target: RenderTarget::Image(pixel_target.image.clone().into()),
                msaa_writeback: false,
                ..default()
            },
            PIXEL_PERFECT_LAYERS,
            Msaa::Off,
        ));
    });
    cmd.spawn((
        Name::new("Bg Camera"),
        Camera2d,
        SyncCamera,
        Camera {
            target: RenderTarget::Image(bg_handle.clone().into()),
            order: -18,
            msaa_writeback: false,
            ..default()
        },
        BG_LAYER
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
    let light_handle: Handle<Image> = images.add(light_texture);
    cmd.spawn((
        Name::new("Light Camera"),
        Camera2d,
        Camera {
            target: RenderTarget::Image(light_handle.clone().into()),
            order: -17,
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
            occluders_texture: occluders_texture_handle.clone(),
            emitters: 0,
            noise_texture: noise_handle.clone(),
            width: size.width,
            height: size.height,
            lights: [RelativeLightEmitter::default(); MAX_EMITTERS],
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
            rotation_z: 0.,
            position_x: 0.,
            position_y: 0.,
            debree_inf: 0.,
            _pad: 0,
            light_texture: light_handle.clone(),
            occluders_texture: occluders_texture_handle.clone(),
            scene_texture: retarget_handle.clone(),
            noise_texture: noise_handle.clone(),
            bg_texture: bg_handle.clone(),
            lit_texture: lit_texture_handle,
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

    emitters: Query<(&LightEmitter, &GlobalTransform)>,
    camera: Single<&GlobalTransform, With<PixelCamera>>,

    mut light: Query<&mut MeshMaterial2d<LightMaterial>>,
    mut compositor: Query<&mut MeshMaterial2d<CompositorMaterial>>,
) {
    for m in light.iter_mut() {
        let material = light_mat.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
        material.emitters = 0;
        for (emitter, relative_to_cam) in emitters.iter() {
            let camera_inverse = camera.compute_matrix().inverse();
            let emitter_pos = camera_inverse.transform_point3(relative_to_cam.translation());
            let relative = emitter_pos.xy();

            let emitter_global_rot = relative_to_cam.rotation().to_euler(EulerRot::XYZ).2;
            let camera_global_rot = camera.rotation().to_euler(EulerRot::XYZ).2;
            let relative_rotation = -(emitter_global_rot - camera_global_rot) / PI * 180.;

            let mut emitter = emitter.to_emitter(relative);
            emitter.color_and_rotation.w += relative_rotation as f32;
            material.lights[material.emitters as usize] = emitter;
            material.emitters += 1;
            if material.emitters >= MAX_EMITTERS as u32 {
                break;
            }
        }

        // material.lights[0] = RelativeLightEmitter {
        //     camera_relative_position: Vec2::new(100.0, 200.0),
        //     radius: 3.0,
        //     spot: 0.5,
        //     color_and_rotation: Vec4::new(1.0, 0.0, 0.0, 1.0),
        // };

        // material.lights[1] = RelativeLightEmitter {
        //     camera_relative_position: Vec2::new(777.0, 123.0),
        //     radius: 6.0,
        //     spot: 0.8,
        //     color_and_rotation: Vec4::new(0.0, 0.0, 1.0, 2.0),
        // };
    }
    for m in compositor.iter_mut() {
        let material = comp_mat.get_mut(&m.0).unwrap();
        material.time = time.elapsed_secs() as f32;
    }
}

pub fn sync_cameras(
    mut cams: Query<(&mut Transform, &mut Projection), (With<SyncCamera>, Without<PixelCamera>)>,
    c2d: Single<(&Transform, &Projection), (With<PixelCamera>, Without<SyncCamera>)>,
) {
    for mut co in cams.iter_mut() {
        *co.0 = *c2d.0;
        *co.1 = c2d.1.clone();
    }
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
