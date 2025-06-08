use std::time::Duration;

use bevy::prelude::*;
use components::{InInteractionArray, InteractGlowEvent, KeyTimer, ScrollSelector};
use systems::*;

use crate::{core::states::{GlobalAppState, OnGame}, interactions::{chain_reaction_display::*, collision_minigame::*, hack_minigame::*, pipe_puzzle::*, warning_interface::*, wave_modulator::*, wires_minigame::*}, ui::components::hack_button::ui_hack_button_hover};

mod systems;
pub mod components;
pub mod chain_reaction_display;
pub mod wave_modulator;
pub mod pipe_puzzle;
pub mod collision_minigame;
pub mod warning_interface;
pub mod hack_minigame;
pub mod wires_minigame;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<InteractGlowEvent>()
        .insert_resource(KeyTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .insert_resource(ScrollSelector::default())
        .insert_resource(InInteractionArray {
            in_interaction: components::InteractionTypes::ChainReactionDisplay,
            in_any_interaction: false,
        })
        .insert_resource(Spinny::default())
        .insert_resource(WaveModulatorConsts::default())
        .insert_resource(CollisionMinigameConsts::default())
        .insert_resource(PipeMinigame::default())
        .insert_resource(HackGrid::default())
        .insert_resource(WireMinigame::default())
        .insert_resource(WarningTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .add_systems(Update, (
            (interact, update_interactables,
                (update_pipes, open_pipe_puzzle_display, 
                open_warning_interface_display,
                update_warning_interface_display,
                (
                    open_chain_graph_display, update_chain_graph_display
                ).chain(),
                (
                    generate_wave_modulator_consts, open_wave_modulator_display, touch_wavemod_spinny,
                    interact_with_wavemod_spinny, update_wave_modulator_display
                ).chain(),
                (
                    generate_collision_minigame_consts, open_collision_minigame_display, interact_with_spinny_collision,
                    update_collision_minigame
                ).chain(),
                (
                    init_hack_display, open_hack_display, update_hack_display.before(ui_hack_button_hover)
                ).chain(),
                (
                    open_wires_display, touch_wires_inlet
                ).chain(),
            )
            .run_if(in_state(GlobalAppState::InGame))
            ).chain(),
        ))
        ;
    }
}
