use std::time::Duration;

use bevy::prelude::*;
use bevy_tailwind::tw;
use utils::WrappedDelta;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::{containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::{text_display_green_handle, ui_text_display_green_with_text}}, ui_atlas_container::ui_atlas_container}, target::LowresUiContainer}, utils::{custom_material_loader::{MalfAtlasHandles, SpriteAssets, WarningAtlasHandles}, debree::{Malfunction, MalfunctionType, TIME_TO_RESOLVE}, energy::{Energy, ENGINE_THRESHOLD}, spacial_audio::PlaySoundEvent}};

pub const WARNING_GRID_COLUMNS: u32 = 2;
pub const WARNING_GRID_ROWS: u32 = 2;
pub const WARNING_GRID_SIZE: u32 = 50;

#[derive(Component)]
pub struct WarningScreen;

#[derive(Default)]
pub struct WarningData {
    pub color: bool, // 0 - red; 1 - yellow
    pub text: String,
}

#[derive(Component)]
pub struct WarningText;

#[derive(Component)]
pub struct SurplusText;

#[derive(Component, Clone)]
pub struct TimerText {
    pub malfunction_type: MalfunctionType,
}

#[derive(Component, Clone)]
pub struct MalfMini {
    pub malfunction_type: MalfunctionType,
}

pub fn open_warning_interface_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    waning_atlas_handles: Res<WarningAtlasHandles>,
    malfunction: Res<Malfunction>,
    mut event_writer: EventWriter<PlaySoundEvent>,
    energy: Res<Energy>,
    malf_atlas_handles: Res<MalfAtlasHandles>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WarningInterface && in_interaction_array.in_any_interaction {
            event_writer.write(PlaySoundEvent::OpenUi);
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            let text_bundle = text_display_green_handle(&asset_server);
            let mut warning_text = "No Malfunctions";
            if malfunction.in_progress {
                warning_text = &malfunction.warning_data[malfunction.warning_data.len() - 1].text;
            }
            let text_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_text_display_green_with_text(&text_bundle, (WarningText, WarningText), &warning_text, &asset_server)
                ])
            ).id();

            let surplus_text = format!("Power Surplus : {} GW", energy.surplus - ENGINE_THRESHOLD);
            let surplus_text_entity = commands.spawn(
            ui_main_container(&main, children![
                ui_text_display_green_with_text(&text_bundle, (SurplusText, SurplusText), &surplus_text, &asset_server)
                ])
            ).id();

            let mut children = vec![];
            let mut malf_entities = vec![];
            for i in 0..5 {
                let malfunction_type;
                match i {
                    0 => {
                        malfunction_type = MalfunctionType::Collision;
                    }
                    1 => {
                        malfunction_type = MalfunctionType::Hack;
                    }
                    2 => {
                        malfunction_type = MalfunctionType::Reactor;
                    }
                    3 => {
                        malfunction_type = MalfunctionType::Waves;
                    }
                    4 => {
                        malfunction_type = MalfunctionType::Engine;
                    }
                    _ => {unreachable!()}
                }
                let text = TimerText {
                    malfunction_type: malfunction_type.clone(),
                };
                children.push(commands.spawn(
                ui_main_container(&main, children![
                    ui_text_display_green_with_text(&text_bundle, (text.clone(), text), "NaN", &asset_server)
                ])).id());
                let mini = MalfMini {
                    malfunction_type,
                };
                malf_entities.push(commands.spawn(
                ui_main_container(&main, children![
                    ui_atlas_container(&(malf_atlas_handles.image_handle.clone(), malf_atlas_handles.layout_handle.clone()), mini)
                    ])
                ).id());
            }

            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(
                //    ui_main_container(&main, ())
                ()
                ).insert(tw!("flex-col p-[2px] items-center gap-[1px]")) //items-stretch 
                .with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(text_entity);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_child(surplus_text_entity);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_children(&children);
                    });
                    cmd.spawn(ui_sub_container(&sub, ()))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                        .add_children(&malf_entities);
                    });
                });
            }).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

#[derive(Resource)]
pub struct WarningTimer {
    pub timer: Timer
}

pub fn update_warning_interface_display(
    mut warning_timer: ResMut<WarningTimer>,
    time: Res<Time>,
    malfunction: Res<Malfunction>,
    energy: Res<Energy>,
    text: Query<&mut Text, With<SurplusText>>,
    timer_text: Query<(&mut Text, &TimerText), Without<SurplusText>>,
    warning_text: Query<&mut Text, (Without<SurplusText>, Without<TimerText>, With<WarningText>)>,
    mini_image_nodes: Query<(&mut ImageNode, &MalfMini)>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    warning_timer.timer.tick(Duration::from_secs_f32(time.dt()));
    for mut text in text {
        text.0 = format!("Power Surplus : {} GW", energy.surplus - ENGINE_THRESHOLD);
    }
    for (mut text, timer_text) in timer_text {
        let mut time = "NaN".to_string();
        if let Some(index) = malfunction.malfunction_types.iter().position(|r| r == &timer_text.malfunction_type) {
            time = format!("{:.1}", TIME_TO_RESOLVE - malfunction.malfunction_timers[index].elapsed_secs());
        }
        text.0 = time;
    }
    for (mut node, mini) in mini_image_nodes {
        if let Some(atlas) = &mut node.texture_atlas {
            let mut node_index = 0;
            match mini.malfunction_type {
                MalfunctionType::NoMalfunction => {
                    unreachable!()
                },
                MalfunctionType::Reactor => {
                    node_index = 2;
                },
                MalfunctionType::Collision => {
                    node_index = 4;
                },
                MalfunctionType::Hack => {
                    node_index = 1;
                },
                MalfunctionType::Waves => {
                    node_index = 3;
                },
                MalfunctionType::Engine => {
                    node_index = 5;
                },
            }
            if malfunction.malfunction_types.contains(&mini.malfunction_type) {
                node_index += 6;
                if warning_timer.timer.elapsed_secs() < warning_timer.timer.duration().as_secs_f32() / 2. {
                    node_index = 0;
                    event_writer.write(PlaySoundEvent::Beep);
                }
            }
            atlas.index = node_index;
        }
    }
    if malfunction.in_progress {
        for mut text in warning_text {
            text.0 = malfunction.warning_data[malfunction.warning_data.len() - 1].text.clone();
        }
    }
}