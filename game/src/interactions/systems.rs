use std::time::Duration;

use bevy::{input::mouse::{MouseMotion, MouseWheel}, platform::collections::HashMap, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;
use shaders::VelocityEmmiter;
use utils::{Easings, WrappedDelta};

use crate::utils::{custom_material_loader::{TextureAtlasHandes, KEYS_ATLAS_SIZE}, debree::DebreeLevel, mouse::CursorPosition};

use super::{chain_reaction_display::ChainGraphMaterial, components::{EKey, InInteraction, InInteractionArray, InteractGlowEvent, InteractableMaterial, InteractionTypes, KeyTimer, ScrollSelector}, wave_modulator::WaveGraphMaterial};

pub fn interact(
    mut commands: Commands,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Single<(Entity, &VelocityEmmiter)>,
    mut writer: EventWriter<InteractGlowEvent>,
    mut interactable: Query<(&mut InInteraction, &Transform)>,
    texture_atlas_handles: Res<TextureAtlasHandes>,
    mut scroll_selector: ResMut<ScrollSelector>,
    keyboard: Res<ButtonInput<KeyCode>>,
    interaction_types: Query<&InteractionTypes>,
    mut in_interaction_array: ResMut<InInteractionArray>
) {
    // TODO: stop player in interaction
    // println!("{:?}", in_interaction_array);
    if in_interaction_array.in_any_interaction {
        if keyboard.just_released(KeyCode::KeyE) {
            in_interaction_array.in_any_interaction = false;
            //exit
            // TODO: add "press E again to exit sign"
        }
        return;
    }
    if keyboard.just_released(KeyCode::KeyE) && !scroll_selector.current_displayed.is_none() {
        let current_entity = scroll_selector.selection_options[scroll_selector.current_selected];
        let interaction_type = interaction_types.get(current_entity).unwrap().clone();
        in_interaction_array.in_any_interaction = true;
        in_interaction_array.in_interaction = interaction_type;
    }
    let mut mouse_scroll_delta = 0.;
    for event in mouse_wheel_events.read() {
        let v =  event.y * if let bevy::input::mouse::MouseScrollUnit::Line = event.unit {1.0} else {(1. / event.y).abs()};
        mouse_scroll_delta += v;
    };
    if scroll_selector.selection_options.len() > 0 {
        let new;
        if mouse_scroll_delta < 0. {
            if scroll_selector.current_selected == 0 {
                new = scroll_selector.selection_options.len() - 1;
            } else {
                // TODO: fix this shit
                new = (scroll_selector.current_selected - (mouse_scroll_delta as usize).min(scroll_selector.selection_options.len())) % scroll_selector.selection_options.len();
            }
        } else {
            new = (scroll_selector.current_selected + mouse_scroll_delta as usize) % scroll_selector.selection_options.len();
        }
        if new != scroll_selector.current_selected {
            if let Some(current_displayed) = scroll_selector.current_displayed {
                commands.entity(current_displayed).despawn();
                scroll_selector.current_displayed = None;
            }
            scroll_selector.current_selected = new;
        }
    }
    // println!("{} {:?}", scroll_selector.current_selected, scroll_selector.selection_options);
    
    for collision_event in collision_events.read() {
        // println!("{:?}", collision_event);
        match collision_event {
            // interactable - sender; sensor - reciever
            CollisionEvent::Started(reciever_entity, sender_entity, _) => {
                let (mut in_interaction, _) = interactable.get_mut(*sender_entity).unwrap();
                scroll_selector.selection_options.push(*sender_entity);
                in_interaction.data = true;
            }
            CollisionEvent::Stopped(reciever_entity, sender_entity, _) => {
                let (mut in_interaction, interactable_transform) = interactable.get_mut(*sender_entity).unwrap();
                in_interaction.data = false;
                if let Some(index) = scroll_selector.selection_options.iter().position(|&e| e == *sender_entity) {
                    scroll_selector.selection_options.remove(index);
                    if let Some(current_displayed) = scroll_selector.current_displayed {
                        commands.entity(current_displayed).despawn();
                        scroll_selector.current_displayed = None;
                        scroll_selector.current_selected = 0;
                    }
                }
            }
        }
    }
    for option_entity in scroll_selector.selection_options.clone() {
        if scroll_selector.selection_options[scroll_selector.current_selected] == option_entity && scroll_selector.current_displayed.is_none() {
            // print!("{:?}", option_entity);
            let interactable_pos = interactable.get_mut(option_entity).unwrap().1.translation;
            let e_key_entity = commands.spawn((
                Sprite::from_atlas_image(
                    texture_atlas_handles.image_handle.clone(),
                    TextureAtlas::from(texture_atlas_handles.layout_handle.clone()),
                ),
                Transform::from_translation(interactable_pos + Vec3::Y * 50.),
                EKey,
                Name::new("EKey"),
            )).id();
            scroll_selector.current_displayed = Some(e_key_entity.clone());
        }
    }
}

pub fn update_interactables(
    mut material_assets: ResMut<Assets<InteractableMaterial>>,
    material_handles: Query<(&MeshMaterial2d<InteractableMaterial>, &InInteraction)>,
    mut e_keys: Query<&mut Sprite, With<EKey>>,
    time: Res<Time>,
    mut key_timer: ResMut<KeyTimer>,
) {
    for (material_handle, in_interaction) in material_handles {
        if in_interaction.data {
            if let Ok(mut e_keys) = e_keys.single_mut() {
                let atlas = e_keys.texture_atlas.as_mut().unwrap();
                key_timer.timer.tick(Duration::from_secs_f32((time.dt() * 5.).ease_out_quad()));
                if key_timer.timer.finished() {
                    atlas.index = (atlas.index + 1) % KEYS_ATLAS_SIZE as usize;
                }
                if let Some(material) = material_assets.get_mut(material_handle) {
                    material.time = time.elapsed_secs();
                }
            }
        } else {
            if let Some(material) = material_assets.get_mut(material_handle) {
                material.time = 0.;
            }
        }
    }
}

pub fn update_graphs_time(
    debree_level: Res<DebreeLevel>,
    chain_material_handle: Query<&MeshMaterial2d<ChainGraphMaterial>>,
    wave_material_handle: Query<&MeshMaterial2d<WaveGraphMaterial>>,
    mut chain_material_assets: ResMut<Assets<ChainGraphMaterial>>,
    mut wave_material_assets: ResMut<Assets<WaveGraphMaterial>>,
    time: Res<Time>,
) {
    if let Ok(chain_material_handle) = chain_material_handle.single() {
        if let Some(material) = chain_material_assets.get_mut(chain_material_handle) {
            material.chain = time.elapsed_secs();
        }
    }
    if let Ok(wave_material_handle) = wave_material_handle.single() {
        if let Some(material) = wave_material_assets.get_mut(wave_material_handle) {
            material.time = time.elapsed_secs();
        }
    }
}