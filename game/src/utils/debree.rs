use bevy::prelude::*;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use utils::WrappedDelta;

pub struct DebreePlugin;

impl Plugin for DebreePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DebreeLevel::default())
        .add_systems(Update, debree_level_management);
    }
}

#[derive(Resource, Default)]
pub struct DebreeLevel {
    pub base_level: f32,
    pub level: f32,
    pub chain_reaction: f32,
    pub malfunction_probability: f32,
}

pub fn debree_level_management(
    time: Res<Time>,
    mut debree_level: ResMut<DebreeLevel>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
) {
    // debree level 0..inf -> chain reaction 0..100% & malfunction probability per frame
    // causes player to manage chain reaction via hack+deorbit, antennas level and condition
    // debree level is not linearly prop to chain reaction; strategic deorbit can lower chain reaction
    debree_level.base_level = 1.2f32.powf(-10.0 + 0.6 * time.elapsed_secs_wrapped() * (1. / 60.));
    debree_level.chain_reaction = time.elapsed_secs_wrapped();
    overlay_text!(
        overlay_events;
        TopLeft;
        DEBREE_LEVEL:format!("Debree base level {:?}", debree_level.base_level),(255, 255, 255););
}