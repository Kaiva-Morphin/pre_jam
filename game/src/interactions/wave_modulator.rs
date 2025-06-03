use std::f32::consts::TAU;

use bevy::{color::palettes::css::{BLUE, RED}, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}, ui::RelativeCursorPosition};
use pixel_utils::camera::{PixelCamera, RenderCamera, TARGET_HEIGHT, TARGET_WIDTH};

use crate::utils::{custom_material_loader::SpinnyAtlasHandles, mouse::CursorPosition};

use super::components::{InInteractionArray, InteractablesImageHandle, InteractionTypes};


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct WaveGraphMaterial {
    #[uniform(0)]
    pub a: f32,
    #[uniform(0)]
    pub b: f32,
    #[uniform(0)]
    pub c: f32,
    #[uniform(0)]
    pub d: f32,
    #[uniform(0)]
    pub ra: f32,
    #[uniform(0)]
    pub rb: f32,
    #[uniform(0)]
    pub rc: f32,
    #[uniform(0)]
    pub rd: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub _webgl2_padding_8b: u32,
    #[uniform(0)]
    pub _webgl2_padding_12b: u32,
    #[uniform(0)]
    pub _webgl2_padding_16b: u32,
    #[texture(1)]
    #[sampler(2)]
    pub sprite_handle: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub base_sprite_handle: Handle<Image>,
}

const WAVEGRAPH_MATERIAL_PATH: &str = "shaders/wave_graph.wgsl";

impl Material2d for WaveGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVEGRAPH_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn open_wave_modulator_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    interactables_material_handle: Res<InteractablesImageHandle>,
    pc: Single<Entity, With<PixelCamera>>,
    rc: Single<Entity, With<RenderCamera>>,
    spinny_atlas_handles: Res<SpinnyAtlasHandles>,
) {
    // println!("{:?} {:?}", in_interaction_array, already_spawned);
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WaveModulator && in_interaction_array.in_any_interaction {
            let entity = commands.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                UiTargetCamera(*pc),
                children![
                    (
                        // wave graph
                        ImageNode {
                            image: interactables_material_handle.rendered_image_handle.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            ..default()
                        },
                    ),
                    (
                        // spinny
                        SpinnyIds {id: 0},
                        ImageNode::from_atlas_image(
                            spinny_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(spinny_atlas_handles.layout_handle.clone())
                        ),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            ..default()
                        },
                    ),
                    (
                        BackgroundColor::from(Color::Srgba(RED)),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(100.),
                            width: Val::Px(20.),
                            height: Val::Px(200.),
                            ..default()
                        },
                        RelativeCursorPosition::default(),
                    )
                ]
            )).id();
            *already_spawned = Some(entity);
        }
    }
}

pub const NUM_SPINNY_STATES: f32 = 8.;
pub const SPINNY_SIZE: UVec2 = UVec2::new(20,20);
const ANGLE_PER_SPINNY_STATE: f32 = TAU / NUM_SPINNY_STATES;

#[derive(Resource)]
pub struct Spinny {
    pub is_locked: bool,
    pub locked_id: usize,
}

#[derive(Component)]
pub struct SpinnyIds {
    pub id: usize,
}

#[derive(Component)]
pub struct WaveGraph;

pub fn custom_relative_cursor_system(
    q: Single<&RelativeCursorPosition>,
) {
    println!("{:?}", q.normalized);
}

pub fn interact_with_spinny(
    spinny: Res<Spinny>,
    spinny_q: Query<(&SpinnyIds, &mut ImageNode)>,
    material_handle: Single<&MeshMaterial2d<WaveGraphMaterial>>,
    mut material_assets: ResMut<Assets<WaveGraphMaterial>>,
) {
    if spinny.is_locked {
        let angle = 0f32;
        let snapped_state = (angle / ANGLE_PER_SPINNY_STATE).floor();
        for (spinny_id, mut spinny_image_node) in spinny_q {
            if spinny_id.id == spinny.locked_id {
                if let Some(material) = material_assets.get_mut(*material_handle) {
                    material.a += 1.;
                }
                if let Some(texture_atlas) = &mut spinny_image_node.texture_atlas {
                    texture_atlas.index = snapped_state as usize;
                }
            }
        }
    }
}