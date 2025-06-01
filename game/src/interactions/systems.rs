use std::time::Duration;

use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;
use shaders::VelocityEmmiter;
use utils::WrappedDelta;

use crate::utils::{custom_material_loader::{TextureAtlasHandes, KEYS_ATLAS_SIZE}, mouse::CursorPosition};

use super::components::{EKey, InInteraction, InteractGlowEvent, InteractableKeyLink, InteractableMaterial, KeyTimer};

pub fn interact(
    mut commands: Commands,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Single<(Entity, &VelocityEmmiter)>,
    mut writer: EventWriter<InteractGlowEvent>,
    mut in_interaction: Query<&mut InInteraction>,
    texture_atlas_handles: Res<TextureAtlasHandes>,
    mut link: Query<&mut InteractableKeyLink>,
) {
    for collision_event in collision_events.read() {
        // println!("{:?}", collision_event);
        match collision_event {
            // interactable - sender; sensor - reciever
            CollisionEvent::Started(reciever_entity, sender_entity, _) => {
            in_interaction.get_mut(*sender_entity).unwrap().data = true;
            let e_key_entity = commands.spawn((
                Sprite::from_atlas_image(
                    texture_atlas_handles.image_handle.clone(),
                    TextureAtlas::from(texture_atlas_handles.layout_handle.clone()),
                ),
                Transform::from_translation(Vec3::new(0., 100., 0.)),
                EKey,
                Name::new("EKey"),
            )).id();
            link.get_mut(*sender_entity).unwrap().entity = e_key_entity;
            }
            CollisionEvent::Stopped(reciever_entity, sender_entity, _) => {
            in_interaction.get_mut(*sender_entity).unwrap().data = false;
            commands.entity(link.get(*sender_entity).unwrap().entity).despawn();
            }
        }
    }
}

pub fn update_iteractables(
    mut material_assets: ResMut<Assets<InteractableMaterial>>,
    material_handles: Query<(&MeshMaterial2d<InteractableMaterial>, &InInteraction)>,
    mut e_keys: Single<&mut Sprite, With<EKey>>,
    time: Res<Time>,
    mut key_timer: ResMut<KeyTimer>,
) {
    for (material_handle, in_interaction) in material_handles {
        if in_interaction.data {
            let atlas = e_keys.texture_atlas.as_mut().unwrap();
            key_timer.timer.tick(Duration::from_secs_f32(time.dt()));
            if key_timer.timer.finished() {
                atlas.index = (atlas.index + 1) % KEYS_ATLAS_SIZE as usize;
            }
            if let Some(material) = material_assets.get_mut(material_handle) {
                material.time = time.elapsed_secs();
            }
        } else {
            if let Some(material) = material_assets.get_mut(material_handle) {
                material.time = 0.;
            }
        }
    }
}