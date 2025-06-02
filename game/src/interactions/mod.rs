use std::time::Duration;

use bevy::prelude::*;
use chain_reaction_display::{open_display, update_chain};
use components::{InInteractionArray, InteractGlowEvent, InteractablesImageHandle, KeyTimer, ScrollSelector};
use systems::*;
use unteractables_ui::{spawn_ui_camera};

use crate::utils::custom_material_loader::{preload_sprites, LoadingStates};

mod systems;
pub mod components;
pub mod chain_reaction_display;
pub mod unteractables_ui;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<InteractGlowEvent>()
        .insert_resource(KeyTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .insert_resource(ScrollSelector::default())
        .insert_resource(InInteractionArray {in_interaction: [false], in_any_interaction: false})
        .insert_resource(InteractablesImageHandle::default())
        .add_systems(Update, (
            (interact, update_interactables, open_display).chain(),
        ))
        .add_systems(OnEnter(LoadingStates::Next), spawn_ui_camera)
        .add_systems(Update, update_chain)
        ;
    }
}
