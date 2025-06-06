use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}, ui::RelativeCursorPosition};
use bevy_tailwind::tw;

use crate::{interactions::{components::{InInteractionArray, InteractionTypes}, wave_modulator::{Spinny, SpinnyIds}}, ui::{components::{containers::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, spinny::ui_spinny}, target::LowresUiContainer}, utils::{custom_material_loader::{SpinnyAtlasHandles, SpriteAssets}, debree::{Malfunction, MalfunctionType}}};


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
    malfunction: Res<Malfunction>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::CollisionMinigame && in_interaction_array.in_any_interaction {
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
            
            let mut is_active = false;
            if malfunction.malfunction_types.contains(&MalfunctionType::Collision) {
                is_active = true;
            }
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);

            let mut children = vec![];
            for i in 0..2 {
                children.push(commands.spawn(
                ui_main_container(&main, children![(
                    ui_spinny(&(spinny_atlas_handles.image_handle.clone(), spinny_atlas_handles.layout_handle.clone()), SpinnyIds { id: i }, ()),
                )])).id());
            }
            children.push(
                commands.spawn(
                MaterialNode(collision_graph_material.add(
                CollisionGraphMaterial {
                    a: collision_consts.consts1[0],
                    b: collision_consts.consts1[1],
                    ra: collision_consts.consts1[2],
                    rb: collision_consts.consts1[3],
                    time: 0.,
                    _webgl2_padding_8b: 0,
                    _webgl2_padding_12b: 0,
                    _webgl2_padding_16b: 0,
                    sprite_handle,
                    base_sprite_handle: sprite_assets.wave_graph_sprite.clone(),
                })),).id()
            );
            
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ())).with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ())).with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_children(&children);
                    });
                });
            }).id();
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
                            material.a = consts.consts2[0][snapped_state];
                        },
                        1 => {
                            material.b = consts.consts2[1][snapped_state];
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
    pub consts1: [f32; 4],
    pub consts2: [Vec<f32>; 2],
    pub is_loaded: bool,
}

pub fn generate_collision_minigame_consts(
    mut collision_consts: ResMut<CollisionMinigameConsts>,
    malfunction: Res<Malfunction>,
) {
    if malfunction.malfunction_types.contains(&MalfunctionType::Collision) && !collision_consts.is_loaded {
        collision_consts.is_loaded = true;
        let mi_offset = 0.;
        let ma_offset = 2.;
    
        let mi_amplitude = 0.1;
        let ma_amplitude = 10.; // visible, but not clipped
    
        let a = gen_collision_rng(mi_offset, ma_offset);
        let b = gen_collision_rng(mi_amplitude, ma_amplitude);
        
        let ra = a.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
        let rb = b.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
        collision_consts.consts1 = [a.0, b.0, ra, rb];
        collision_consts.consts2 = [a.1, b.1];
    }
}

fn gen_collision_rng(mi: f32, ma: f32) -> (f32, Vec<f32>) {
    let a = mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (ma - mi) as f32);
    let mut t = (0..NUM_COLLISION_STATES as i32 / 2)
    .map(|i| mi + ((a - mi) / (NUM_COLLISION_STATES / 2.) * i as f32)).collect::<Vec<f32>>();
    t.extend((0..NUM_COLLISION_STATES as i32 / 2).map(|i| a + ((ma - a) / (NUM_COLLISION_STATES / 2.) * i as f32)));
    (a, t)
}

pub fn update_collision_minigame(
    
) {

}