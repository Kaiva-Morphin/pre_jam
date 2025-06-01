use std::time::Duration;

use bevy::prelude::*;
use chain_reaction_display::ChainGraphMaterialHandle;
use components::{InInteractionArray, InteractGlowEvent, KeyTimer, ScrollSelector};
use systems::*;

mod systems;
pub mod components;
pub mod chain_reaction_display;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<InteractGlowEvent>()
        .insert_resource(KeyTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .insert_resource(ScrollSelector::default())
        .insert_resource(InInteractionArray {in_interaction: [false], in_any_interaction: false})
        .insert_resource(ChainGraphMaterialHandle {handle: Handle::default()})
        .add_systems(Update, (interact, update_interactables).chain())
        ;
    }
}
