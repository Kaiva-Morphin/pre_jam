use bevy::prelude::*;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use utils::WrappedDelta;

use crate::{core::states::GlobalAppState, interactions::warning_interface::WarningData, utils::custom_material_loader::SpriteAssets};

pub struct DebreePlugin;

impl Plugin for DebreePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DebreeLevel::default())
        .insert_resource(Malfunction::default())
        .add_systems(Update, (debree_level_management, manage_malfunctions).run_if(in_state(GlobalAppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct DebreeLevel {
    pub base_level: f32,
    pub const_add: f32,
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
    let t = time.elapsed_secs_wrapped();
    let start = 0.00019;
    let growth = 0.003;
    debree_level.base_level = start * (growth * t).exp();

    debree_level.level = debree_level.base_level + debree_level.const_add;
    debree_level.malfunction_probability = debree_level.level;
    // malfunc prob is perframe
    debree_level.chain_reaction = time.elapsed_secs_wrapped();
    overlay_text!(
        overlay_events;
        TopLeft;
        DEBREE_LEVEL:format!(
            "Debree base level {:.5?}
            Malfunction probability {:.2} %
            ",
            debree_level.base_level, debree_level.malfunction_probability * 100.),(255, 255, 255);
        );
}

#[derive(Resource, Default)]
pub struct Malfunction {
    pub in_progress: bool,
    pub malfunction_types: Vec<MalfunctionType>,
    pub warning_data: Vec<WarningData>,
}

#[derive(Default)]
pub enum MalfunctionType {
    #[default]
    NoMalfunction,
    Reactor,
    Collision,
}

const MALFUNCTION_TYPES_NUM: usize = 3;

pub fn manage_malfunctions(
    debree_level: Res<DebreeLevel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut malfunction: ResMut<Malfunction>,
    sprite_assets: Res<SpriteAssets>,
) {
    if (getrandom::u32().unwrap() as f32 / u32::MAX as f32) < debree_level.malfunction_probability || keyboard.just_pressed(KeyCode::KeyP) {
        // TODO: && not in progress
        malfunction.in_progress = true;
        let malfunc_type = ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * (MALFUNCTION_TYPES_NUM as f32 - 1.)) as usize;
        match malfunc_type {
            0 => {
                malfunction.malfunction_types.push(MalfunctionType::Reactor);
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "Reactor malfunctioned".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            1 => {
                malfunction.malfunction_types.push(MalfunctionType::Collision);
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "Collision".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            _ => unreachable!()
        };
    }
}