use std::time::Duration;

use bevy::prelude::*;
use components::{InteractGlowEvent, KeyTimer};
use systems::*;

mod systems;
pub mod components;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<InteractGlowEvent>()
        .insert_resource(KeyTimer {timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)})
        .add_systems(Update, (interact, update_iteractables).chain())
        ;
    }
}
