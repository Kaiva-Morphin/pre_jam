use std::f32::consts::{PI, TAU};

use bevy::{color::palettes::css::{BLUE, RED}, prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureUsages}, sprite::{AlphaMode2d, Material2d}, ui::RelativeCursorPosition};
use bevy_tailwind::tw;

use crate::{interactions::components::PlayerSensor, ui::{components::{containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::{text_display_green_handle, ui_text_display_green_with_text}, viewport_container::{ui_viewport_container, viewport_handle}}, spinny::ui_spinny, ui_submit_button::{submit_button_bundle, ui_submit_button}}, target::LowresUiContainer}, utils::{custom_material_loader::{SpinnyAtlasHandles, SpriteAssets}, debree::{Malfunction, MalfunctionType, Resolved}, mouse::CursorPosition, spacial_audio::PlaySoundEvent}};

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

const WAVEGRAPH_MATERIAL_PATH: &str = "shaders/wave_graph.wgsl";

impl UiMaterial for WaveGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVEGRAPH_MATERIAL_PATH.into()
    }
}

#[derive(Component)]
pub struct WaveModText;

const ANTENNAS_WORK: &str =          "Antennas Work Normally";
const WAVES_SYNCHRONISED: &str =     "  Waves Synchronised  ";
const WAVES_NOT_SYNCHRONISED: &str = "Waves Not Synchronised ";

#[derive(Component)]
pub struct WaveButton;

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
    mut malfunction: ResMut<Malfunction>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WaveModulator && in_interaction_array.in_any_interaction {
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
            let text_bundle = text_display_green_handle(&asset_server);
            let submit_bundle = submit_button_bundle(&asset_server, &mut texture_atlases);

            let mut children = vec![];
            for i in 0..4 {
                children.push(commands.spawn(
                ui_main_container(&main, children![(
                    ui_spinny(&(spinny_atlas_handles.image_handle.clone(), spinny_atlas_handles.layout_handle.clone()), SpinnyIds { id: i, angle: 0. }, ()),
                )])).id());
            };

            let mut is_active = 0.;
            let mut wave_text = ANTENNAS_WORK;
            if malfunction.malfunction_types.contains(&MalfunctionType::Waves) {
                is_active = 1.;
                wave_text = WAVES_NOT_SYNCHRONISED;
            }
            let text_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_text_display_green_with_text(&text_bundle, (WaveModText, WaveModText), wave_text, &asset_server)
                ])
            ).id();
            
            let submit_button_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_submit_button(&submit_bundle, WaveButton)
                ])
            ).id();

            let material = MaterialNode(wave_graph_material.add(
            WaveGraphMaterial {
                a: modulator_consts.consts1[0],
                b: modulator_consts.consts1[1],
                c: modulator_consts.consts1[2],
                d: modulator_consts.consts1[3],
                ra: modulator_consts.consts1[4],
                rb: modulator_consts.consts1[5],
                rc: modulator_consts.consts1[6],
                rd: modulator_consts.consts1[7],
                time: 0.,
                is_active,
                _webgl2_padding_12b: 0,
                _webgl2_padding_16b: 0,
                sprite_handle,
                base_sprite_handle: sprite_assets.wave_graph_sprite.clone(),
            }));

            let ui_entity = commands.spawn(
            ui_main_container(&main, children![(
                ui_viewport_container(&view, 
                    children![(
                        material,
                        tw!("z-10 w-[128px] h-[128px]")
                )]),)])
            ).id();

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
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_children(&children);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(submit_button_entity);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(text_entity);
                    });
                });
            }).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub const NUM_SPINNY_STATES: f32 = 8.;
pub const SPINNY_SIZE: UVec2 = UVec2::splat(38);
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
    pub angle: f32,
}

#[derive(Component)]
pub struct WaveGraph;

pub fn touch_wavemod_spinny(
    spinny_q: Query<(&RelativeCursorPosition, &mut SpinnyIds)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut spinny: ResMut<Spinny>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        spinny.is_locked = false;
    }
    for (cursor_rel_pos, mut spinny_id) in spinny_q {
        if let Some(rel_pos) = cursor_rel_pos.normalized {
            let changed_pos = (rel_pos - Vec2::ONE / 2.) * -2.;
            if (changed_pos.x * changed_pos.x + changed_pos.y * changed_pos.y) < 1. &&
            mouse_button.just_pressed(MouseButton::Left) {
                spinny.is_locked = true;
                spinny.locked_id = spinny_id.id;
            }
            if spinny.is_locked && spinny.locked_id == spinny_id.id {
                let angle = changed_pos.to_angle();
                spinny.angle = angle;
                spinny_id.angle = angle;
            }
        }
    }
}

pub fn interact_with_wavemod_spinny(
    spinny: Res<Spinny>,
    spinny_q: Query<(&SpinnyIds, &mut ImageNode)>,
    material_handle: Single<&MaterialNode<WaveGraphMaterial>>,
    mut material_assets: ResMut<Assets<WaveGraphMaterial>>,
    modulator_consts: Res<WaveModulatorConsts>,
    malfunction: Res<Malfunction>,
    time: Res<Time>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    if let Some(material) = material_assets.get_mut(*material_handle) {
        if modulator_consts.is_loaded {
            material.time = time.elapsed_secs();
        }
    }
    if spinny.is_locked {
        if spinny.angle < 0. {
            return;
        }
        for (spinny_id, mut spinny_image_node) in spinny_q {
            if spinny_id.id == spinny.locked_id {
                let snapped_state = (spinny.angle / ANGLE_PER_SPINNY_STATE).floor() as usize;
                if let Some(material) = material_assets.get_mut(*material_handle) {
                    if modulator_consts.is_loaded {
                        let mut is_active = 0.;
                        if malfunction.malfunction_types.contains(&MalfunctionType::Waves) {
                            is_active = 1.;
                        }
                        material.is_active = is_active;
                        match spinny_id.id {
                            0 => {
                                material.a = modulator_consts.consts2[0][snapped_state];
                            },
                            1 => {
                                material.b = modulator_consts.consts2[1][snapped_state];
                            },
                            2 => {
                                material.c = modulator_consts.consts2[2][snapped_state];
                            },
                            3 => {
                                material.d = modulator_consts.consts2[3][snapped_state];
                            },
                            _ => {}
                        }
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

#[derive(Resource, Default)]
pub struct WaveModulatorConsts {
    pub consts1: [f32; 8],
    pub consts2: [Vec<f32>; 4],
    pub is_loaded: bool,
}

pub fn generate_wave_modulator_consts(
    mut consts: ResMut<WaveModulatorConsts>,
    malfunction: Res<Malfunction>,
) {
    if malfunction.malfunction_types.contains(&MalfunctionType::Waves) && !consts.is_loaded {
        consts.is_loaded = true;
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

        let ra = a.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_SPINNY_STATES) as usize];
        let rb = b.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_SPINNY_STATES) as usize];
        let rc = c.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_SPINNY_STATES) as usize];
        let rd = d.1[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * NUM_SPINNY_STATES) as usize];
        consts.consts1 = [a.0, b.0, c.0, d.0, ra, rb, rc, rd];
        consts.consts2 = [a.1, b.1, c.1, d.1];
    }
}

fn gen_wave_rng(mi: f32, ma: f32) -> (f32, Vec<f32>) {
    let a = mi as f32 + ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (ma - mi) as f32);
    let mut t = (0..NUM_SPINNY_STATES as i32 / 2)
    .map(|i| mi + ((a - mi) / (NUM_SPINNY_STATES / 2.) * i as f32)).collect::<Vec<f32>>();
    t.extend((0..NUM_SPINNY_STATES as i32 / 2).map(|i| a + ((ma - a) / (NUM_SPINNY_STATES / 2.) * i as f32)));
    (a, t)
}

pub fn update_wave_modulator_display(
    mut malfunction: ResMut<Malfunction>,
    modulator_consts: Res<WaveModulatorConsts>,
    material_handle: Single<&MaterialNode<WaveGraphMaterial>>,
    mut material_assets: ResMut<Assets<WaveGraphMaterial>>,
    mut interaction_query: Query<(&Interaction, &mut ImageNode), With<WaveButton>>,
    mut submited: Local<bool>,
    mut prev: Local<Interaction>,
    text: Query<&mut Text, With<WaveModText>>,
) {
    let mut in_progress = false;
    if malfunction.malfunction_types.contains(&MalfunctionType::Waves) {
        in_progress = true;
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
                println!("submitted waves");
                *submited = true;
            }
            a.index = index;
        }
        *prev = *interaction;
    }
    if in_progress && modulator_consts.is_loaded {
        if let Some(material) = material_assets.get_mut(*material_handle) {
            let a1 = material.ra;
            let b1 = material.rb;
            let c1 = material.rc;
            let d1 = material.rd;

            let [a2, b2, c2, d2] = [
                material.a,
                material.b,
                material.c,
                material.d,
            ];

            let mut in_sync = false;
            if a1 == a2 && b1 == b2 && c1 == c2 && d1 == d2 {
                in_sync = true;
            }
            for mut text in text {
                if in_sync {
                    text.0 = WAVES_SYNCHRONISED.to_string();
                } else {
                    text.0 = WAVES_NOT_SYNCHRONISED.to_string();
                }
                if *submited {
                    if in_sync {
                        text.0 = ANTENNAS_WORK.to_string();
                    }
                    malfunction.resolved.push(Resolved {
                        resolved_type: MalfunctionType::Waves,
                        failed: !in_sync,
                    });
                    *prev = Interaction::default();
                    *submited = false;
                }
            }
        }
    }
}