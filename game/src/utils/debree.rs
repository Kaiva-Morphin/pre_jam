use std::collections::HashMap;

use bevy::prelude::*;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use tiled::PropertyValue;
use utils::WrappedDelta;

use crate::{core::states::GlobalAppState, interactions::warning_interface::WarningData, utils::custom_material_loader::SpriteAssets};

pub struct DebreePlugin;

impl Plugin for DebreePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(DebreeLevel::default())
        .insert_resource(Malfunction::default())
        .add_systems(Update, (debree_level_management, manage_malfunctions, resolve_malfunctions).run_if(in_state(GlobalAppState::InGame)));
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
    debree_level.malfunction_probability = 0.;
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

#[derive(Clone)]
pub struct Resolved {
    pub resolved_type: MalfunctionType,
    pub failed: bool,
}

#[derive(Resource, Default)]
pub struct Malfunction {
    pub in_progress: bool,
    pub malfunction_types: Vec<MalfunctionType>,
    pub warning_data: Vec<WarningData>,
    pub resolved: Vec<Resolved>,
    pub added_new_malfunction: bool,
}

#[derive(Default, PartialEq, Clone)]
pub enum MalfunctionType {
    #[default]
    NoMalfunction,
    Reactor,
    Collision,
    Hack,
    Waves,
}

impl MalfunctionType {
    pub fn from_properties(properties: &HashMap<String, PropertyValue>) -> Option<Self> {
        let Some(PropertyValue::StringValue(s)) = properties.get("type") else {return None};
        match s.as_str() {
            "MAINFRAME" => None,
            "HACK" => Some(Self::Hack),
            "REACTOR" => Some(Self::Reactor),
            "ENGINE" => Some(Self::Collision),
            "ANTENNA" => Some(Self::Waves),
            _ => None
        }
    }
}

const MALFUNCTION_TYPES_NUM: usize = 5;
const ALL_MALFUNCTION_TYPES: [MalfunctionType; MALFUNCTION_TYPES_NUM - 1] = [
    MalfunctionType::Reactor,
    MalfunctionType::Collision,
    MalfunctionType::Hack,
    MalfunctionType::Waves,
];

pub fn manage_malfunctions(
    debree_level: Res<DebreeLevel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut malfunction: ResMut<Malfunction>,
    sprite_assets: Res<SpriteAssets>,
) {
    if (getrandom::u32().unwrap() as f32 / u32::MAX as f32) < debree_level.malfunction_probability || keyboard.just_pressed(KeyCode::KeyP) {
        malfunction.in_progress = true;
        let mut available_for_malfunction = vec![];
        for malf_type in ALL_MALFUNCTION_TYPES.iter() {
            if !malfunction.malfunction_types.contains(malf_type) {
                available_for_malfunction.push(malf_type.clone());
            }
        }
        if available_for_malfunction.is_empty() {
            println!("all possible malfunctions are in progress");
            malfunction.added_new_malfunction = false;
            return;
        }
        malfunction.added_new_malfunction = true;
        let malfunc_type_idx = ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * available_for_malfunction.len() as f32) as usize;
        let malfunc_type = available_for_malfunction[malfunc_type_idx].clone();
        match malfunc_type {
            MalfunctionType::Reactor => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "Reactor malfunctioned!".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            MalfunctionType::Collision => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "The ship is on a trajectory to collide with debree!".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            MalfunctionType::Hack => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: true,
                    text: "A sattelite is on a collision trajectory!".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            MalfunctionType::Waves => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: true,
                    text: "Antenna malfunctioned!".to_string(),
                    handle: sprite_assets.reactor_mini.clone(),
                });
            },
            MalfunctionType::NoMalfunction => unreachable!()
        };
        println!("new malfunc: {:?}", malfunction.warning_data[malfunction.warning_data.len() - 1].text);
    }
}

pub fn get_random_range(mi: f32, ma: f32) -> f32 {
    let rand = getrandom::u32().unwrap() as f32 / (u32::MAX as f32);
    mi + rand * (ma + 1. - mi)
}

pub fn resolve_malfunctions(
    mut malfunction: ResMut<Malfunction>,
    mut debree_level: ResMut<DebreeLevel>,
) {
    if !malfunction.resolved.is_empty() {
        for resolved in malfunction.resolved.clone() {
            let index = malfunction.malfunction_types.iter().position(|r| r == &resolved.resolved_type).unwrap();
            let to_be_resolved = malfunction.malfunction_types.remove(index);
            match to_be_resolved {
                MalfunctionType::Hack => {
                    if resolved.failed {
                        println!("failed hack");
                    } else {
                        println!("resolved hack");
                    }
                },
                MalfunctionType::Collision => {
                    if resolved.failed {
                        println!("failed collision");
                    } else {
                        println!("resolved collision");
                    }
                },
                MalfunctionType::Reactor => {
                    if resolved.failed {
                        println!("failed reactor");
                    } else {
                        println!("resolved reactor");
                    }
                },
                MalfunctionType::Waves => {
                    if resolved.failed {
                        println!("failed waves");
                    } else {
                        println!("resolved waves");
                    }
                },
                MalfunctionType::NoMalfunction => {unreachable!()}
            }
        }
        malfunction.resolved = vec![];
        if malfunction.malfunction_types.is_empty() {
            malfunction.in_progress = false;
        }
    }
}