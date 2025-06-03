use std::time::Duration;

use bevy::prelude::*;
use chain_reaction_display::{open_chain_graph_display};
use components::{InInteractionArray, InteractGlowEvent, KeyTimer, ScrollSelector};
use systems::*;
use wave_modulator::{open_wave_modulator_display};

use crate::{interactions::{pipe_puzzle::open_pipe_puzzle_display, wave_modulator::{interact_with_spinny, touch_spinny, Spinny, WaveModulatorConsts}}, utils::custom_material_loader::{preload_sprites, LoadingStates}};

mod systems;
pub mod components;
pub mod chain_reaction_display;
pub mod wave_modulator;
pub mod pipe_puzzle;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<InteractGlowEvent>()
        .insert_resource(KeyTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .insert_resource(ScrollSelector::default())
        .insert_resource(InInteractionArray {in_interaction: components::InteractionTypes::ChainReactionDisplay, in_any_interaction: false})
        .insert_resource(Spinny::default())
        .insert_resource(WaveModulatorConsts::default())
        .add_systems(Update, (
            (interact, update_interactables, open_pipe_puzzle_display, update_graphs_time, touch_spinny, interact_with_spinny,
                (open_chain_graph_display, open_wave_modulator_display)
                .run_if(in_state(LoadingStates::Next))
            ).chain(),
        ))
        ;
    }
}
