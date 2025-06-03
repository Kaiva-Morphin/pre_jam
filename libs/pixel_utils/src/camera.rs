use std::ops::DerefMut;

use bevy::{color::palettes::css::GRAY, core_pipeline::{bloom::{Bloom, BloomCompositeMode}, tonemapping::{DebandDither, Tonemapping}}, input::keyboard::{self, KeyboardInput}, prelude::*, render::{camera::RenderTarget, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, window::WindowResized};


pub struct PixelCameraPlugin;


impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, setup_camera)
            .add_systems(PreUpdate, (true_pixel_switch, fit_canvas).chain())
            .insert_resource(ViewportSize::default())
            .insert_resource(PixelCameraSettings {true_pixel: false});
    }
}

pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub const SCALE : u32 = 1;
pub const TARGET_WIDTH: u32 = 480 * SCALE;
pub const TARGET_HEIGHT: u32 = 270 * SCALE;

#[derive(Component)]
pub struct RenderCamera;

#[derive(bevy_inspector_egui::InspectorOptions)]
#[derive(Resource)]
struct PixelCameraSettings {
    pub true_pixel: bool
}

#[derive(Component)]
struct PixelCanvas;

#[derive(Component)]
pub struct PixelCamera;




pub fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
){
    let canvas_size = Extent3d {
        width: TARGET_WIDTH,
        height: TARGET_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
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

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(Srgba::rgb(0.0, 0.0, 0.0).into()),
            // hdr: true,
            msaa_writeback: false,
            ..default()
        },
        Msaa::Off,
        PixelCamera,
        Name::new("PixelCamera"),
        // Tonemapping::TonyMcMapface,
        // DebandDither::Enabled,
        Transform::from_scale(Vec3::splat(1.0 / SCALE as f32)),
        // Bloom {
        //     composite_mode: BloomCompositeMode::Additive,
        //     intensity: 0.1,
        //     ..default()
        // },
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((Sprite::from_image(image_handle), PixelCanvas, HIGH_RES_LAYERS));

    commands.spawn((
        Camera2d,
        Camera{
            order: 0, 
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            hdr: false, 
            msaa_writeback: false, 
            ..default()
        }, 
        Name::new("RenderCamera"),
        Msaa::Off, 
        RenderCamera, 
        HIGH_RES_LAYERS
    ));
}

#[derive(Resource)]
pub struct ViewportSize {
    pub target: UVec2,
}

impl Default for ViewportSize {
    fn default() -> Self {
        ViewportSize {
            target: UVec2::new(TARGET_WIDTH, TARGET_HEIGHT),
            // current: UVec2::new(TARGET_WIDTH, TARGET_HEIGHT)
        }
    }
}
impl ViewportSize {

}

fn true_pixel_switch(
    mut settings : ResMut<PixelCameraSettings>,
    mut projection: Single<&mut Projection, With<RenderCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
){
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    if keyboard.just_pressed(KeyCode::F4){
        settings.true_pixel = !settings.true_pixel;
    }
    resize(projection, window.size(), settings.true_pixel);
}

fn resize(projection : &mut OrthographicProjection, size : Vec2, true_pixel: bool){
    let h_scale = size.x / TARGET_WIDTH as f32;
    let v_scale = size.y / TARGET_HEIGHT as f32;
    let mut scale = h_scale.min(v_scale);
    if true_pixel {
        scale = scale.floor();
    }
    projection.scale = 1. / scale;
}

fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projection: Single<&mut Projection, With<RenderCamera>>,
    settings : Res<PixelCameraSettings>,
    // mut images: ResMut<Assets<Image>>,
    // canvas: Single<&Sprite, With<PixelCanvas>>,
    // mut viewport : ResMut<ViewportSize>,
){
    let true_pixel = settings.true_pixel;
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for event in resize_events.read() {
        resize(projection, vec2(event.width, event.height), true_pixel);
    }
} 