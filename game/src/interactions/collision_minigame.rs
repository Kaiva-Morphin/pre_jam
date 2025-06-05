use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}, ui::RelativeCursorPosition};

use crate::{interactions::{components::{InInteractionArray, InteractionTypes}, wave_modulator::{Spinny, SpinnyIds}}, ui::target::LowresUiContainer, utils::custom_material_loader::{SpinnyAtlasHandles, SpriteAssets}};


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct CollisionGraphMaterial {
    #[uniform(0)]
    pub a: f32,
    #[uniform(0)]
    pub b: f32,
    #[uniform(0)]
    pub ra: f32,
    #[uniform(0)]
    pub rb: f32,
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

const WAVEGRAPH_MATERIAL_PATH: &str = "shaders/collision_graph.wgsl";

impl UiMaterial for CollisionGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVEGRAPH_MATERIAL_PATH.into()
    }
}

pub fn open_collision_minigame_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    spinny_atlas_handles: Res<SpinnyAtlasHandles>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    mut collision_graph_material: ResMut<Assets<CollisionGraphMaterial>>,
    mut collision_consts: ResMut<CollisionMinigameConsts>,
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
        if in_interaction_array.in_interaction == InteractionTypes::CollisionMinigame && in_interaction_array.in_any_interaction {
            let consts = generate_collision_minigame_consts();
            collision_consts.consts = consts.1;
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
                        // collision graph
                        MaterialNode(collision_graph_material.add(
                            CollisionGraphMaterial {
                                a: consts.0[0],
                                b: consts.0[1],
                                ra: consts.0[2],
                                rb: consts.0[3],
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
                        // TODO: change it to the acceleration lever
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
                ]
            )).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub fn interact_with_spinny_collision(
    spinny: Res<Spinny>,
    spinny_q: Query<(&SpinnyIds, &mut ImageNode)>,
    material_handle: Single<&MaterialNode<CollisionGraphMaterial>>,
    mut material_assets: ResMut<Assets<CollisionGraphMaterial>>,
    consts: Res<CollisionMinigameConsts>,
) {
    if spinny.is_locked {
        if spinny.angle < 0. {
            return;
        }
        let snapped_state = (spinny.angle / ANGLE_PER_COLLISION_SPINNY_STATE).floor() as usize;
        for (spinny_id, mut spinny_image_node) in spinny_q {
            if spinny_id.id == spinny.locked_id {
                if let Some(material) = material_assets.get_mut(*material_handle) {
                    match spinny_id.id {
                        0 => {
                            material.a = consts.consts[0][snapped_state];
                        },
                        1 => {
                            material.b = consts.consts[1][snapped_state];
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

const NUM_COLLISION_STATES: f32 = 8.;
const ANGLE_PER_COLLISION_SPINNY_STATE: f32 = PI / NUM_COLLISION_STATES;

#[derive(Resource, Default)]
pub struct CollisionMinigameConsts {
    pub consts: [Vec<f32>; 2],
}

pub fn generate_collision_minigame_consts() -> ([f32; 4], [Vec<f32>; 2]) {
    // For horizontal offset (a, ra)
    let mi_offset = 0.;
    let ma_offset = 2.;

    // For amplitude (b, rb)
    let mi_amplitude = 0.1;
    let ma_amplitude = 10.; // visible, but not clipped

    let a = gen_collision_rng(mi_offset, ma_offset);
    let b = gen_collision_rng(mi_amplitude, ma_amplitude);

    let ra = a.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
    let rb = b.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
    ([a.0, b.0, ra, rb], [a.1, b.1])
}

fn gen_collision_rng(mi: f32, ma: f32) -> (f32, Vec<f32>) {
    let a = mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (ma - mi) as f32);
    let mut t = (0..NUM_COLLISION_STATES as i32 / 2)
    .map(|i| mi + ((a - mi) / (NUM_COLLISION_STATES / 2.) * i as f32)).collect::<Vec<f32>>();
    t.extend((0..NUM_COLLISION_STATES as i32 / 2).map(|i| a + ((ma - a) / (NUM_COLLISION_STATES / 2.) * i as f32)));
    (a, t)
}