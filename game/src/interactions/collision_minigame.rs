use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}, ui::RelativeCursorPosition};
use bevy_tailwind::tw;

use crate::{interactions::{components::{InInteractionArray, InteractionTypes}, wave_modulator::{Spinny, SpinnyIds}}, ui::{components::{containers::{base::*, text_display::{text_display_green_handle, ui_text_display_green_with_text}, viewport_container::{ui_viewport_container, viewport_handle}}, spinny::ui_spinny, ui_submit_button::{submit_button_bundle, ui_submit_button}}, target::LowresUiContainer}, utils::{custom_material_loader::{SpinnyAtlasHandles, SpriteAssets}, debree::{Malfunction, MalfunctionType, Resolved}, energy::Energy, spacial_audio::PlaySoundEvent}};


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[repr(align(16))]
pub struct CollisionGraphMaterial {
    #[uniform(0)]
    pub a: f32,
    #[uniform(0)]
    pub b: f32,
    #[uniform(0)]
    pub u: f32,
    #[uniform(0)]
    pub r: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub is_active: f32,
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

#[derive(Component)]
pub struct SubmitButton;

#[derive(Component)]
pub struct CollisionText;

#[derive(Component)]
pub struct CollisionCostText;

const TRAJECTORY_SAFE: &str =     "  TRAJECTORY SAFE  ";
const COLLISION_IMPENDING: &str = "IMPENDING COLLISION";
const COLLISION_AVOIDED: &str =   " COLLISION AVOIDED ";

pub fn open_collision_minigame_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    spinny_atlas_handles: Res<SpinnyAtlasHandles>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    mut collision_graph_material: ResMut<Assets<CollisionGraphMaterial>>,
    collision_consts: ResMut<CollisionMinigameConsts>,
    images: Res<Assets<Image>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    malfunction: Res<Malfunction>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::CollisionMinigame && in_interaction_array.in_any_interaction {
            event_writer.write(PlaySoundEvent::OpenUi);
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
            
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            let view = viewport_handle(&asset_server);
            let submit_bundle = submit_button_bundle(&asset_server, &mut texture_atlases);
            let text_bundle = text_display_green_handle(&asset_server);

            let mut children = vec![];
            for i in 0..2 {
                children.push(commands.spawn(
                ui_main_container(&main, children![(
                    ui_spinny(&(spinny_atlas_handles.image_handle.clone(), spinny_atlas_handles.layout_handle.clone()), SpinnyIds { id: i, angle: 0. }, ()),
                )])).id());
            }
            let mut is_active = 0.;
            let mut collision_text = TRAJECTORY_SAFE;
            if malfunction.malfunction_types.contains(&MalfunctionType::Collision) {
                is_active = 1.;
                collision_text = COLLISION_IMPENDING;
            }
            let material = MaterialNode(collision_graph_material.add(
                CollisionGraphMaterial {
                    a: collision_consts.consts1[0],
                    b: collision_consts.consts1[1],
                    u: collision_consts.consts1[2],
                    r: collision_consts.consts1[3],
                    time: 0.,
                    is_active,
                    _webgl2_padding_12b: 0,
                    _webgl2_padding_16b: 0,
                    sprite_handle,
                    base_sprite_handle: sprite_assets.wave_graph_sprite.clone(),
                })
            );

            let ui_entity = commands.spawn(
            ui_main_container(&main, children![(
                ui_viewport_container(&view, 
                    children![(
                        material,
                        tw!("z-10 w-[128px] h-[128px]")
                )]),)])
            ).id();

            let submit_button_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_submit_button(&submit_bundle, ())
                ])
            ).id();
            
            let text_entity = commands.spawn(
            ui_main_container(&main, ())
            )
            .with_children(|cmd|{
                cmd.spawn(
                    ui_text_display_green_with_text(&text_bundle, (CollisionText, CollisionText), collision_text, &asset_server)
                ).insert(tw!("w-[350px] items-center justify-center p-[5px]"));
            }).id();

            let collision_text = "Maneuver cost: 0 GJ";
            let text_entity1 = commands.spawn(
            ui_main_container(&main, ())
            )
            .with_children(|cmd|{
                cmd.spawn(
                    ui_text_display_green_with_text(&text_bundle, (CollisionCostText, CollisionCostText), collision_text, &asset_server)
                ).insert(tw!("w-[350px] items-center justify-center p-[5px]"));
            }).id();

            // let text_entity1 = commands.spawn(
            // ui_main_container(&main, children![
            //     ui_text_display_green_with_text(&text_bundle, (CollisionCostText, CollisionCostText), collision_text, &asset_server)
            //     ])
            // ).insert(tw!("w-[350px]")).id();
            
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ()))
                .with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(ui_entity);
                    });
                    cmd.spawn(ui_main_container(&main, ())).insert(
                        tw!("flex flex-col-reverse")
                    ).with_children(|cmd|{
                            cmd.spawn(ui_sub_container(&sub, ()))
                            .with_children(|cmd| {
                                cmd.spawn(tw!("items-center justify-center w-full h-full gap-[1px]"),)
                                .add_children(&children)
                                .add_child(submit_button_entity);
                            });
                            cmd.spawn(ui_sub_container(&sub, ()))
                            .with_children(|cmd| {
                                cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                                .add_child(text_entity);
                            });
                            cmd.spawn(ui_sub_container(&sub, ()))
                            .with_children(|cmd| {
                                cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                                .add_child(text_entity1);
                            });
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
    time: Res<Time>,
    malfunction: Res<Malfunction>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    if let Some(material) = material_assets.get_mut(*material_handle) {
        if consts.is_loaded {
            material.time = (time.elapsed_wrapped() - consts.start_time).as_secs_f32();
        }
    }
    if spinny.is_locked {
        if spinny.angle < 0. {
            return;
        } // TODO: if engine broke dont allow
        let snapped_state = (spinny.angle / ANGLE_PER_COLLISION_SPINNY_STATE).floor() as usize;
        for (spinny_id, mut spinny_image_node) in spinny_q {
            if spinny_id.id == spinny.locked_id {
                if let Some(material) = material_assets.get_mut(*material_handle) {
                    if consts.is_loaded {
                        let mut is_active = 0.;
                        if malfunction.malfunction_types.contains(&MalfunctionType::Collision) {
                            is_active = 1.;
                        }
                        material.is_active = is_active;
                        match spinny_id.id {
                            0 => {
                                material.u = consts.consts2[0][snapped_state];
                            },
                            1 => {
                                material.r = consts.consts2[1][snapped_state];
                            },
                            _ => {}
                        }
                        println!("u {} r {}", material.u, material.r);
                    }
                }
                if let Some(texture_atlas) = &mut spinny_image_node.texture_atlas {
                    if texture_atlas.index != snapped_state {
                        event_writer.write(PlaySoundEvent::SpinnyClick);
                        texture_atlas.index = snapped_state;
                    }
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
    pub start_time: Duration,
}

pub fn generate_collision_minigame_consts(
    mut collision_consts: ResMut<CollisionMinigameConsts>,
    malfunction: Res<Malfunction>,
    time: Res<Time>,
) {
    if malfunction.malfunction_types.contains(&MalfunctionType::Collision) && !collision_consts.is_loaded {
        collision_consts.is_loaded = true;
        let a_mi = 0.;
        let a_ma = 5.;
    
        let b_mi = 0.1;
        let b_ma = 3.;

        let u_mi = 1.;
        let u_ma = 8.;
    
        let r_mi = 0.1;
        let r_ma = 1.;

        let a = a_mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (a_ma + 1. - a_mi) as f32);
        let b = b_mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (b_ma + 1. - b_mi) as f32);

        let u = gen_collision_rng(u_mi, u_ma);
        let r = gen_collision_rng(r_mi, r_ma);
        
        let ru = u.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
        let rr = r.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_COLLISION_STATES) as usize];
        collision_consts.consts1 = [a, b, ru, rr];
        collision_consts.consts2 = [u.1, r.1];
        let start_time = time.elapsed_wrapped();
        collision_consts.start_time = start_time;
    }
}

fn gen_collision_rng(mi: f32, ma: f32) -> (f32, Vec<f32>) {
    let a = mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (ma - mi) as f32);
    let mut t = (0..NUM_COLLISION_STATES as i32 / 2)
    .map(|i| mi + ((a - mi) / (NUM_COLLISION_STATES / 2.) * i as f32)).collect::<Vec<f32>>();
    t.extend((0..NUM_COLLISION_STATES as i32 / 2).map(|i| a + ((ma - a) / (NUM_COLLISION_STATES / 2.) * i as f32)));
    (a, t)
}

fn find_intersection(a: f32, b: f32, u: f32, r: f32, time: f32) -> bool {
    let x_min = 1.0;
    let x_max = 10.0;
    let step = 0.01;

    let mut last_diff = None;

    for i in 0..=((x_max - x_min) / step) as usize {
        let x = x_min + i as f32 * step;
        let fx  = u / (x.max(0.0).powf(r));
        let fx1 = ((x - a) * (b + time / 10.0)).max(0.0).sqrt() - x + a;
        let diff = fx - fx1;

        if let Some(last) = last_diff {
            if last * diff < 0.0 {
                return true;
            }
        }
        last_diff = Some(diff);
    }
    return false;
}

pub fn update_collision_minigame(
    mut interaction_query: Query<(&Interaction, &mut ImageNode), With<SubmitButton>>,
    text: Query<&mut Text, With<CollisionText>>,
    mut text1: Query<&mut Text, (With<CollisionCostText>, Without<CollisionText>)>,
    material_assets: Res<Assets<CollisionGraphMaterial>>,
    material_handle: Single<&MaterialNode<CollisionGraphMaterial>>,
    mut malfunction: ResMut<Malfunction>,
    mut submited: Local<bool>,
    mut prev: Local<Interaction>,
    spinny: Res<Spinny>,
    mut spinny_q: Query<&mut SpinnyIds>,
    mut energy: ResMut<Energy>,
    mut cost: Local<f32>,
) {
    let mut in_progress = false;
    if malfunction.malfunction_types.contains(&MalfunctionType::Collision) {
        in_progress = true;
    }
    
    const COST_PER_ANG: f32 = 0.5;
    if spinny.angle >= 0. {
        let mut t_cost = 0.;
        for mut spinny_id in spinny_q.iter_mut() {
            if spinny.locked_id == spinny_id.id {
                spinny_id.angle = spinny.angle;
            }
            let snapped = (spinny_id.angle / ANGLE_PER_COLLISION_SPINNY_STATE).floor();
            match spinny_id.id {
                0 => {
                    t_cost += COST_PER_ANG * snapped
                }
                1 => {
                    t_cost += COST_PER_ANG * snapped * 2.
                }
                _ => unreachable!()
            } 
        }
        *cost = t_cost;
    }
    for (interaction, mut node) in
        &mut interaction_query
    {
        let mut index = 0;
        if *interaction == Interaction::Pressed {
            index = 1;
        }
        if let Some(a) = &mut node.texture_atlas {
            if *prev == Interaction::Pressed && *interaction != Interaction::Pressed && in_progress {
                // submitted solution
                println!("submitted collision sol");
                *submited = true;
            }
            a.index = index;
        }
        for mut text in text1.iter_mut() {
            text.0 = format!("Maneuver cost: {:.2} GJ", *cost);
        }
        *prev = *interaction;
    }
    if let Some(material) = material_assets.get(*material_handle) {
        let intersects = find_intersection(material.a, material.b, material.u, material.r, material.time);
        for mut text in text {
            if in_progress {
                    if intersects {
                        text.0 = COLLISION_IMPENDING.to_string();
                    } else {
                        text.0 = COLLISION_AVOIDED.to_string();
                    }
                    if *submited {
                        if !intersects {
                            text.0 = TRAJECTORY_SAFE.to_string();
                        }
                        malfunction.resolved.push(Resolved {
                            resolved_type: MalfunctionType::Collision,
                            failed: intersects,
                        });
                        energy.increase_consumption = (*cost, Duration::from_secs_f32(30.));
                        *prev = Interaction::default();
                        *submited = false;
                        *cost = 0.;
                    }
                }
            }
    }
}
