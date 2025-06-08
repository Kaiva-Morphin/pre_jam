use std::time::Duration;

use bevy::prelude::*;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Energy::new())
        .add_systems(Update, manage_energy)
        ;
    }
}

#[derive(Resource, Default)]
pub struct Energy {
    pub generated: f32,
    pub engine_consumption: f32,
    pub gravity_consumption: f32,
    pub lamps_consumption: f32,
    pub surplus: f32,
    pub increase_consumption: (f32, Duration)
}

impl Energy {
    pub fn new() -> Self {
        Self {
            generated: 150.,
            engine_consumption: 20.,
            gravity_consumption: 10.,
            lamps_consumption: 10.,
            surplus: 0.,
            increase_consumption: (0., Duration::ZERO)
        }
    }
}

impl Energy {
    pub fn check_if_enough(&self, increased: f32) -> bool {
        self.generated - self.engine_consumption - self.gravity_consumption
        - self.lamps_consumption - increased > ENGINE_THRESHOLD
    }
}

const LAMPS_THRESHOLD: f32 = 90.;
const GRAVITY_THRESHOLD: f32 = 80.;
pub const ENGINE_THRESHOLD: f32 = 70.;

pub fn manage_energy(
    mut energy: ResMut<Energy>,
    time: Res<Time>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
) {
    if !energy.increase_consumption.1.is_zero() {
        energy.increase_consumption.1 = (energy.increase_consumption.1 - time.delta()).clamp(Duration::ZERO, Duration::MAX)
    } else {
        energy.increase_consumption.0 = 0.;
    }
    energy.surplus = energy.generated - energy.engine_consumption - energy.gravity_consumption
    - energy.lamps_consumption - energy.increase_consumption.0;
    overlay_text!(
        overlay_events;
        TopLeft;
        ENERGY:format!(
            "surp {} = generated {} - eng cons {} - grav cons {} - lamps cons {} - incr cons {}", 
            energy.surplus, energy.generated, energy.engine_consumption, energy.gravity_consumption, energy.lamps_consumption, 
            energy.increase_consumption.0
        ),
        (255, 255, 255);
    );
    if energy.surplus < LAMPS_THRESHOLD {
        // jiggle lamps
    }
    if energy.surplus < GRAVITY_THRESHOLD {
        // turn off gravity
    }
    if energy.surplus < ENGINE_THRESHOLD {
        // die
    }
}