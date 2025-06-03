use std::f32::consts::{PI, TAU};

use bevy::{color::palettes::css::{BLUE, RED}, prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}, sprite::{AlphaMode2d, Material2d}, ui::RelativeCursorPosition};
use pixel_utils::camera::{PixelCamera, RenderCamera, TARGET_HEIGHT, TARGET_WIDTH};

use crate::{ui::target::LowresUiContainer, utils::{custom_material_loader::{SpinnyAtlasHandles, SpriteAssets}, mouse::CursorPosition}};

use super::components::{InInteractionArray, InteractionTypes};


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

impl UiMaterial for WaveGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVEGRAPH_MATERIAL_PATH.into()
    }
}

pub fn open_wave_modulator_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    spinny_atlas_handles: Res<SpinnyAtlasHandles>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    mut wave_graph_material: ResMut<Assets<WaveGraphMaterial>>,
    mut modulator_consts: ResMut<WaveModulatorConsts>,
    images: Res<Assets<Image>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WaveModulator && in_interaction_array.in_any_interaction {
            let consts = generate_wave_modulator_consts();
            modulator_consts.consts = consts.1;
            let t = images.get(&sprite_assets.wave_graph_sprite).unwrap();
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
            let sprite_handle = asset_server.add(canvas);
            // println!("{:?} {:?}", interactables_material_handle.rendered_image_handle, interactables_material_handle.base_image_handle);
            let entity = commands.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        // wave graph
                        MaterialNode(wave_graph_material.add(
                            WaveGraphMaterial {
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
                                sprite_handle,
                                base_sprite_handle: sprite_assets.wave_graph_sprite.clone(),
                            })),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            ..default()
                        },
                    ),
                    (
                        // spinny
                        BackgroundColor::from(Color::Srgba(Srgba::new(1., 0., 0., 0.5))),
                        SpinnyIds {id: 0},
                        ImageNode::from_atlas_image(
                            spinny_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(spinny_atlas_handles.layout_handle.clone())
                        ),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        RelativeCursorPosition::default(),
                    ),
                    (
                        // spinny
                        BackgroundColor::from(Color::Srgba(Srgba::new(1., 1., 0., 0.5))),
                        SpinnyIds {id: 1},
                        ImageNode::from_atlas_image(
                            spinny_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(spinny_atlas_handles.layout_handle.clone())
                        ),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        RelativeCursorPosition::default(),
                    ),
                    (
                        // spinny
                        BackgroundColor::from(Color::Srgba(Srgba::new(1., 1., 1., 0.5))),
                        SpinnyIds {id: 2},
                        ImageNode::from_atlas_image(
                            spinny_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(spinny_atlas_handles.layout_handle.clone())
                        ),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        RelativeCursorPosition::default(),
                    ),
                    (
                        // spinny
                        BackgroundColor::from(Color::Srgba(Srgba::new(0., 1., 0., 0.5))),
                        SpinnyIds {id: 3},
                        ImageNode::from_atlas_image(
                            spinny_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(spinny_atlas_handles.layout_handle.clone())
                        ),
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(200.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        RelativeCursorPosition::default(),
                    ),
                ]
            )).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub const NUM_SPINNY_STATES: f32 = 8.;
pub const SPINNY_SIZE: UVec2 = UVec2::new(20,20);
const ANGLE_PER_SPINNY_STATE: f32 = PI / NUM_SPINNY_STATES;

#[derive(Resource, Default)]
pub struct Spinny {
    pub is_locked: bool,
    pub locked_id: usize,
    pub angle: f32,
}

#[derive(Component)]
pub struct SpinnyIds {
    pub id: usize,
}

#[derive(Component)]
pub struct WaveGraph;

pub fn touch_spinny(
    spinny_q: Query<(&RelativeCursorPosition, &SpinnyIds)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut spinny: ResMut<Spinny>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        spinny.is_locked = false;
    }
    for (cursor_rel_pos, spiddy_id) in spinny_q {
        if let Some(rel_pos) = cursor_rel_pos.normalized {
            let changed_pos = (rel_pos - Vec2::ONE / 2.) * -2.;
            if (changed_pos.x * changed_pos.x + changed_pos.y * changed_pos.y) < 1. &&
            mouse_button.just_pressed(MouseButton::Left) {
                spinny.is_locked = true;
                spinny.locked_id = spiddy_id.id;
            }
            if spinny.is_locked && spinny.locked_id == spiddy_id.id {
                let angle = changed_pos.to_angle();
                spinny.angle = angle;
            }
        }
    }
}

pub fn interact_with_spinny(
    spinny: Res<Spinny>,
    spinny_q: Query<(&SpinnyIds, &mut ImageNode)>,
    material_handle: Single<&MaterialNode<WaveGraphMaterial>>,
    mut material_assets: ResMut<Assets<WaveGraphMaterial>>,
    modulator_consts: Res<WaveModulatorConsts>,
) {
    if spinny.is_locked {
        if spinny.angle < 0. {
            return;
        }
        let snapped_state = (spinny.angle / ANGLE_PER_SPINNY_STATE).floor() as usize;
        for (spinny_id, mut spinny_image_node) in spinny_q {
            if spinny_id.id == spinny.locked_id {
                if let Some(material) = material_assets.get_mut(*material_handle) {
                    match spinny_id.id {
                        0 => {
                            material.a = modulator_consts.consts[0][snapped_state];
                        },
                        1 => {
                            material.b = modulator_consts.consts[1][snapped_state];
                        },
                        2 => {
                            material.c = modulator_consts.consts[2][snapped_state];
                        },
                        3 => {
                            material.d = modulator_consts.consts[3][snapped_state];
                        },
                        _ => {}
                    }
                }
                if let Some(texture_atlas) = &mut spinny_image_node.texture_atlas {
                    texture_atlas.index = snapped_state;
                }
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct WaveModulatorConsts {
    pub consts: [Vec<f32>; 4],
}

pub fn generate_wave_modulator_consts() -> ([f32; 8], [Vec<f32>; 4]) {
    // For vertical offset (a, ra)
    let mi_offset = 0.4;
    let ma_offset = 0.6;

    // For amplitude (b, rb)
    let mi_amplitude = 0.2;
    let ma_amplitude = 0.4; // visible, but not clipped

    // For phase (c, rc)
    let mi_phase = 0.0;
    let ma_phase = std::f32::consts::TAU; // 0 to 2π

    // For frequency (d, rd)
    let mi_freq = 1.0;
    let ma_freq = 3.0; // 1 to 3 waves across the texture

    let a = gen_wave_rng(mi_offset, ma_offset);
    let b = gen_wave_rng(mi_amplitude, ma_amplitude);
    let c = gen_wave_rng(mi_phase, ma_phase);
    let d = gen_wave_rng(mi_freq, ma_freq);

    let ra = a.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (NUM_SPINNY_STATES - 1.)) as usize];
    let rb = b.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (NUM_SPINNY_STATES - 1.)) as usize];
    let rc = c.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (NUM_SPINNY_STATES - 1.)) as usize];
    let rd = d.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (NUM_SPINNY_STATES - 1.)) as usize];
    ([a.0, b.0, c.0, d.0, ra, rb, rc, rd], [a.1, b.1, c.1, d.1])
}

fn gen_wave_rng(mi: f32, ma: f32) -> (f32, Vec<f32>) {
    let a = mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (ma - mi) as f32);
    let mut t = (0..NUM_SPINNY_STATES as i32 / 2)
    .map(|i| mi + ((a - mi) / (NUM_SPINNY_STATES / 2.) * i as f32)).collect::<Vec<f32>>();
    t.extend((0..NUM_SPINNY_STATES as i32 / 2).map(|i| a + ((ma - a) / (NUM_SPINNY_STATES / 2.) * i as f32)));
    (a, t)
}