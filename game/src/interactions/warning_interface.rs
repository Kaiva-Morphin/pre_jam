use std::time::Duration;

use bevy::prelude::*;
use bevy_tailwind::tw;
use utils::WrappedDelta;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::{text_display_green_handle, ui_text_display_green_with_text}}, target::LowresUiContainer}, utils::{custom_material_loader::{SpriteAssets, WarningAtlasHandles}, debree::Malfunction, energy::{Energy, ENGINE_THRESHOLD}, spacial_audio::PlaySoundEvent}};

pub const WARNING_GRID_COLUMNS: u32 = 2;
pub const WARNING_GRID_ROWS: u32 = 2;
pub const WARNING_GRID_SIZE: u32 = 50;

#[derive(Component)]
pub struct WarningScreen;

#[derive(Default)]
pub struct WarningData {
    pub color: bool, // 0 - red; 1 - yellow
    pub text: String,
    pub handle: Handle<Image>,
}

#[derive(Component)]
pub struct WarningText;

#[derive(Component)]
pub struct SurplusText;

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

            // let warning_atlas = commands.spawn()
            // (
                    //     // color warning screen
                    //     WarningScreen,
                    //     BackgroundColor::from(Color::Srgba(Srgba::new(0., 0., 1., 0.5))),
                    //     Node {
                    //         width: Val::Px(100.),
                    //         height: Val::Px(100.),
                    //         ..default()
                    //     },
                    //     ImageNode::from_atlas_image(
                    //         waning_atlas_handles.image_handle.clone(),
                    //         TextureAtlas::from(waning_atlas_handles.layout_handle.clone())
                    //     ),
                    // ),
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ()))
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
    mut image_node: Single<&mut ImageNode, With<WarningScreen>>,
    mut warning_timer: ResMut<WarningTimer>,
    time: Res<Time>,
    malfunction: Res<Malfunction>,
    energy: Res<Energy>,
    text: Query<&mut Text, With<SurplusText>>,
) {
    for mut text in text {
        text.0 = format!("Power Surplus : {} GW", energy.surplus - ENGINE_THRESHOLD);
    }
    if malfunction.in_progress {
    //     if let Some(atlas) = &mut image_node.texture_atlas {
    //         warning_timer.timer.tick(Duration::from_secs_f32(time.dt()));
    //         let mut color = 2;
    //         if !malfunction.warning_data.is_empty() {
    //             let mut sub_color = true;
    //             for warning_data in malfunction.warning_data.iter() {
    //                 if !warning_data.color {
    //                     sub_color = false;
    //                     break;
    //                 }
    //             }
    //             color = sub_color as usize;
    //         }
    //         if warning_timer.timer.finished() {
    //             atlas.index = 2 * color + (atlas.index + 1) % 2;
    //         }
    //     }
    }
}