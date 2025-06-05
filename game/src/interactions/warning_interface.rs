use std::time::Duration;

use bevy::prelude::*;
use utils::WrappedDelta;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::target::LowresUiContainer, utils::{custom_material_loader::{SpriteAssets, WarningAtlasHandles}, debree::Malfunction}};

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

pub fn open_warning_interface_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    waning_atlas_handles: Res<WarningAtlasHandles>,
    malfunction: Res<Malfunction>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WarningInterface && in_interaction_array.in_any_interaction {
            
            let entity = commands.spawn((
                BackgroundColor::from(Color::Srgba(Srgba::new(1., 0., 0., 0.5))),
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
                        // main warning screen
                        BackgroundColor::from(Color::Srgba(Srgba::new(0., 1., 0., 0.5))),
                        Node {
                            width: Val::Percent(20.),
                            height: Val::Percent(40.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        children![
                            (
                                TextLayout::new_with_justify(JustifyText::Center),
                                Text::new(malfunction.warning_data[malfunction.warning_data.len() - 1].text.clone()),
                                TextFont {
                                    font: asset_server.load("fonts/orp_regular.ttf"),
                                    font_size: 67.0,
                                    ..default()
                                },
                            ),
                            (
                                Node {
                                    width: Val::Percent(50.),
                                    height: Val::Percent(50.),
                                    ..default()
                                },
                                ImageNode::new(malfunction.warning_data[malfunction.warning_data.len() - 1].handle.clone()),
                            ),
                        ]
                    ),
                    (
                        // color warning screen
                        WarningScreen,
                        BackgroundColor::from(Color::Srgba(Srgba::new(0., 0., 1., 0.5))),
                        Node {
                            width: Val::Px(100.),
                            height: Val::Px(100.),
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            waning_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(waning_atlas_handles.layout_handle.clone())
                        ),
                    ),
                ]
            )).id();
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
) {
    if malfunction.in_progress {
        if let Some(atlas) = &mut image_node.texture_atlas {
            warning_timer.timer.tick(Duration::from_secs_f32(time.dt()));
            let mut color = 2;
            if !malfunction.warning_data.is_empty() {
                let mut sub_color = true;
                for warning_data in malfunction.warning_data.iter() {
                    if !warning_data.color {
                        sub_color = false;
                        break;
                    }
                }
                color = sub_color as usize;
            }
            if warning_timer.timer.finished() {
                atlas.index = 2 * color + (atlas.index + 1) % 2;
            }
        }
    }
}