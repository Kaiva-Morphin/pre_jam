use std::{borrow::Cow, num::NonZero};

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::RED,
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin, render_asset::RenderAssets, render_graph::{self, RenderGraph}, render_resource::{
            binding_types::{storage_buffer_sized, texture_storage_2d}, BindGroupEntries, BindGroupLayoutEntries, BufferDescriptor, BufferUsages, CachedPipelineState, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, PipelineCacheError, ShaderStages, StorageTextureAccess, TextureFormat, TextureUsages
        }, renderer::{RenderContext, RenderDevice, RenderQueue}, texture::GpuImage, Render, RenderApp, RenderSet
    }};
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};
use crate::{components::*, VelocityEmmiter};

const SHADER_ASSET_PATH: &str = "shaders/velocity_buffer.wgsl";

pub struct ComputePlugin;

impl Plugin for ComputePlugin {
    fn build(&self, app: &mut App) {
        println!("START COMP PLUG");
        app.add_plugins(ExtractResourcePlugin::<VelocityBufferHandles>::default());
        app.add_plugins(ExtractResourcePlugin::<Extractor>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            (
                (prepare_bind_group, update_buffer).chain().in_set(RenderSet::PrepareBindGroups),
            ),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(ComputeLabel, ComputeNode::default());
        render_graph.add_node_edge(ComputeLabel, bevy::render::graph::CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ComputePipeline>();
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VelocityBufferMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut buffer_handles: ResMut<VelocityBufferHandles>,
    player_pos: Single<&Transform, With<VelocityEmmiter>>,
) {
    println!("{} {}", TARGET_WIDTH, TARGET_HEIGHT);
    let buffer_handle = generate_empty_buffer(TARGET_WIDTH as usize, TARGET_HEIGHT as usize, &mut images);
    let debug_handle = generate_empty_buffer(TARGET_WIDTH as usize, TARGET_HEIGHT as usize, &mut images);
    buffer_handles.buffer_handle = buffer_handle.clone();
    buffer_handles.debug_handle = debug_handle.clone();
    let handle = materials.add(VelocityBufferMaterial {
       screen_size: Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32),
       player_pos: player_pos.translation.xy(),
       buffer_handle,
       debug_handle,
    });
    buffer_handles.material_handle = handle.clone();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new((TARGET_WIDTH / 2) as f32, (TARGET_HEIGHT / 2) as f32))),
        MeshMaterial2d(handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Depth Buffer")
    ));
}

pub fn generate_empty_buffer(width: usize, height: usize, asset_server: &mut ResMut<Assets<Image>>,) -> Handle<Image> {
    let mut buffer = Vec::with_capacity(width * height * 4);
    for _ in 0..(width * height) {
        buffer.extend_from_slice(&[0.5f32, 0.5f32, 0.5f32, 0.5f32]);
    }
    let buffer: &[u8] = bytemuck::cast_slice(&buffer);
    let mut image = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &buffer,
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::STORAGE_BINDING | TextureUsages::COPY_DST;
    asset_server.add(image)
}

impl render_graph::Node for ComputeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ComputePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            ComputeState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = ComputeState::Init;
                    }
                    // If the shader hasn't loaded yet, just wait.
                    CachedPipelineState::Err(PipelineCacheError::ShaderNotLoaded(_)) => {}
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            ComputeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = ComputeState::Update(1);
                }
            }
            ComputeState::Update(0) => {
                self.state = ComputeState::Update(1);
            }
            ComputeState::Update(1) => {
                self.state = ComputeState::Update(0);
            }
            ComputeState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<ComputeImageBindGroups>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            ComputeState::Loading => {}
            ComputeState::Init => {}
            ComputeState::Update(_index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups.bind_groups[0], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups((TARGET_WIDTH + 7) / 8, (TARGET_HEIGHT + 7) / 8, 1);
            }
        }

        Ok(())
    }
}

fn prepare_bind_group(
    mut commands: Commands,
    gpu_images: Res<RenderAssets<GpuImage>>,
    velocity_buffer_handles: Res<VelocityBufferHandles>,
    render_device: Res<RenderDevice>,
) {
    let device = render_device.wgpu_device();
    let view = gpu_images.get(&velocity_buffer_handles.buffer_handle).unwrap();
    let debug_view = gpu_images.get(&velocity_buffer_handles.debug_handle).unwrap();
    let param_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("parameters_buffer"),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        size: 8,
        mapped_at_creation: false,
    });
    let pos_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("player_pos_buffer"),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        size: 16,
        mapped_at_creation: false,
    });
    let bind_group_layout = render_device.create_bind_group_layout(
        "velocity_buffers",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
            texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadWrite),
            texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadWrite),
            storage_buffer_sized(false, Some(NonZero::new(8).unwrap())),
            storage_buffer_sized(false, Some(NonZero::new(16).unwrap())),
        )),
    );
    let bind_group_velbuf = render_device.create_bind_group(
        None,
        &bind_group_layout,
        &BindGroupEntries::sequential((
            &view.texture_view,
            &debug_view.texture_view,
            param_buffer.as_entire_buffer_binding(),
            pos_buffer.as_entire_buffer_binding(),
        )),
    );
    
    commands.insert_resource(PlayerPosBuffer {buffer: pos_buffer.into()});
    commands.insert_resource(ParameterBuffer {buffer: param_buffer.into()});
    commands.insert_resource(ComputeImageBindGroups {bind_groups: vec![bind_group_velbuf]});
}

pub fn extract_player_pos(
    player_pos: Single<&Transform, With<VelocityEmmiter>>,
    extractor: Option<ResMut<Extractor>>,
) {
    if let Some(mut extractor) = extractor {
        let new_pos = player_pos.translation.xy();
        extractor.player_vel = new_pos - extractor.player_pos;
        extractor.player_pos = new_pos;
    }
}

impl FromWorld for ComputePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "velocity_buffers",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadWrite),
                    texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadWrite),
                    storage_buffer_sized(false, Some(NonZero::new(8).unwrap())),
                    storage_buffer_sized(false, Some(NonZero::new(16).unwrap())),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            zero_initialize_workgroup_memory: false,
        });

        ComputePipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

pub fn update_buffer(
    render_queue: Res<RenderQueue>,
    player_pos_buffer: Res<PlayerPosBuffer>,
    parameter_buffer: Res<ParameterBuffer>,
    extractor: Option<Res<Extractor>>,
) {
    if let Some(extractor) = extractor {
        let screen = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
        let player_pos = extractor.player_pos * Vec2::new(1.,-1.,) / screen * 2. + Vec2::new(0.5,0.5,);
        let player_vel = extractor.player_vel * Vec2::new(1.,-1.,) / screen * 2.;
        info!("pos: {:?} vel: {:?}", player_pos, player_vel);
        render_queue.write_buffer(
            &player_pos_buffer.buffer,
            0,
            bytemuck::cast_slice(&[player_pos.x, player_pos.y, player_vel.x, player_vel.y]),
        );
    }
    render_queue.write_buffer(
        &parameter_buffer.buffer,
        0,
        bytemuck::cast_slice(&[TARGET_WIDTH as f32, TARGET_HEIGHT as f32]),
    );
}

pub fn preload_sprites(
    asset_server: ResMut<AssetServer>,
    mut writer: EventWriter<SpritePreloadEvent>,
) {
    let sprite_handle = asset_server.load("pixel/test.png");
    writer.write(SpritePreloadEvent::Grass(sprite_handle));
}

pub fn spawn_sprites(
    mut commands: Commands,
    image_assets: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GrassMaterial>>,
    buffer_handles: Res<VelocityBufferHandles>,
    mut reader: EventReader<SpritePreloadEvent>,
) {
    for event in reader.read() {
        match event {
            SpritePreloadEvent::Grass(sprite_handle) => {
                let image = image_assets.get(sprite_handle).unwrap();
                let width = image.width();
                let height = image.height();
                let pos = Vec2::new(0., 100.);
                let material = GrassMaterial {
                    pos,
                    sprite_handle: sprite_handle.clone(),
                    velbuf_handle: buffer_handles.buffer_handle.clone(),
                };
                let handle = materials.add(material);
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new((width / 2) as f32, (height / 2) as f32))),
                    MeshMaterial2d(handle),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                    Name::new("Grass"),
                ));
            }
        }
    }
}