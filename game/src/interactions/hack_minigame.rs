use bevy::prelude::*;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::target::LowresUiContainer, utils::custom_material_loader::HackAtlasHandles};

pub const HACK_GRID_SIZE: u32 = 6;
pub const HACK_PIXEL_GRID_SIZE: u32 = 50;
pub const HACK_ATLAS_COLUMNS: u32 = 5;
pub const HACK_ATLAS_ROWS: u32 = 3;

pub fn open_hack_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    hack_atlas_handles: Res<HackAtlasHandles>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::HackMinigame && in_interaction_array.in_any_interaction {
            let mut childern = vec![];
            for y in 0..HACK_GRID_SIZE {
                for x in 0..HACK_GRID_SIZE {
                    childern.push(commands.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            left: Val::Px((HACK_PIXEL_GRID_SIZE * x) as f32),
                            bottom: Val::Px((HACK_PIXEL_GRID_SIZE * y) as f32),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            hack_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(hack_atlas_handles.layout_handle.clone())
                        ),
                        Button,
                    )).id());
                }
            }
            let entity = commands.spawn((
                BackgroundColor::from(Color::Srgba(Srgba::new(0., 1., 0., 0.5))),
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
            )).add_children(&childern).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}