use bevy::prelude::*;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::target::LowresUiContainer};


pub fn open_pipe_puzzle_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::PipePuzzle && in_interaction_array.in_any_interaction {
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
            )).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}