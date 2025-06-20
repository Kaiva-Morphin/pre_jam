use std::{collections::{HashMap, VecDeque}, time::Duration};

use bevy::prelude::*;
use bevy_tailwind::tw;
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use tiled::PropertyValue;
use utils::WrappedDelta;

use crate::{core::states::GlobalAppState, interactions::{chain_reaction_display::CHAIN_GRAPH_LENGTH, pipe_puzzle::PipeMinigame, warning_interface::WarningData}, ui::target::LowresUiContainer, utils::{custom_material_loader::SpriteAssets, energy::Energy, spacial_audio::PlaySoundEvent}};

pub struct DebreePlugin;

impl Plugin for DebreePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<GameEndEvent>()
        .insert_resource(DebreeLevel::new())
        .insert_resource(Malfunction::default())
        .insert_resource(DebreeTimer {timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)})
        .add_systems(Update, (debree_level_management, manage_malfunctions, resolve_malfunctions,
            tick_malfunctions, end_game).run_if(in_state(GlobalAppState::InGame)));
    }
}

#[derive(Resource, Default)]
pub struct DebreeLevel {
    pub base_level: f32,
    pub const_add: f32,
    pub level: f32,
    pub chain_reaction: f32,
    pub malfunction_probability: f32,
    pub chain_reaction_graph: VecDeque<f32>,
}

impl DebreeLevel {
    pub fn new() -> Self {
        Self {
            const_add: 0.0008,
            ..default()
        }
    }
}
#[derive(Resource)]
pub struct DebreeTimer {
    pub timer: Timer,
}

pub fn debree_level_management(
    time: Res<Time>,
    mut debree_level: ResMut<DebreeLevel>,
    mut overlay_events: EventWriter<DebugOverlayEvent>,
    mut timer: ResMut<DebreeTimer>,
) {
    // debree level 0..inf -> chain reaction 0..100% & malfunction probability per frame
    // causes player to manage chain reaction via hack+deorbit, antennas level and condition
    // debree level is not linearly prop to chain reaction; strategic deorbit can lower chain reaction
    let t = time.elapsed_secs_wrapped();
    let start = 0.00035;
    let growth = 0.004;
    debree_level.base_level = start * (growth * t).exp();

    debree_level.level = debree_level.base_level + debree_level.const_add;
    debree_level.malfunction_probability = debree_level.level;
    // debree_level.malfunction_probability = 0.;
    // malfunc prob is perframe
    debree_level.chain_reaction = debree_level.level / 0.7;
    overlay_text!(
        overlay_events;
        TopLeft;
        DEBREE_LEVEL:format!(
            "Debree base level {:.5?}
            Malfunction probability {:.2} %
            ",
            debree_level.base_level, debree_level.malfunction_probability * 100.),(255, 255, 255);
    );
    timer.timer.tick(Duration::from_secs_f32(time.dt()));
    if timer.timer.finished() {
        if debree_level.chain_reaction_graph.len() >= CHAIN_GRAPH_LENGTH * 4 {
            debree_level.chain_reaction_graph.pop_front();
        }
        let cr = debree_level.chain_reaction;
        debree_level.chain_reaction_graph.push_back(cr);
    }
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
    pub malfunction_timers: Vec<Timer>,
    pub warning_data: Vec<WarningData>,
    pub resolved: Vec<Resolved>,
    pub added_new_malfunction: bool,
}

#[derive(Default, PartialEq, Clone, Debug, Hash, Eq)]
pub enum MalfunctionType {
    #[default]
    NoMalfunction,
    Reactor,
    Collision,
    Hack,
    Waves,
    Engine,
}

const MALFUNCTION_TYPES_NUM: usize = 6;
const ALL_MALFUNCTION_TYPES: [MalfunctionType; MALFUNCTION_TYPES_NUM - 1] = [
    MalfunctionType::Reactor,
    MalfunctionType::Collision,
    MalfunctionType::Hack,
    MalfunctionType::Waves,
    MalfunctionType::Engine,
];

pub fn manage_malfunctions(
    debree_level: Res<DebreeLevel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut malfunction: ResMut<Malfunction>,
    sprite_assets: Res<SpriteAssets>,
    mut pipe_minigame: ResMut<PipeMinigame>,
    time: Res<Time>,
    mut minimal_delta: Local<Duration>
) {
    *minimal_delta += Duration::from_secs_f32(time.dt());
    let rand = getrandom::u32().unwrap() as f32 / u32::MAX as f32;
    if rand < debree_level.malfunction_probability /*|| keyboard.just_released(KeyCode::KeyP)*/ {
        // println!("{:?}", minimal_delta);
        if minimal_delta.as_secs_f32() > 10. {
            // println!("AAAAAAAAAAAAAAAAAAAAAAAAAAA");
            *minimal_delta = Duration::ZERO
        } else {
            return;
        }
        malfunction.in_progress = true;
        let mut available_for_malfunction = vec![];
        for malf_type in ALL_MALFUNCTION_TYPES.iter() {
            if !malfunction.malfunction_types.contains(malf_type) {
                available_for_malfunction.push(malf_type.clone());
            }
        }
        if available_for_malfunction.is_empty() {
            println!("all possible malfunctions are in progress {} {}", rand, debree_level.malfunction_probability);
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
                    color: true,
                    text: "Reactor malfunctioned!".to_string(),
                });
                malfunction.malfunction_timers.push(Timer::new(Duration::from_secs_f32(TIME_TO_RESOLVE), TimerMode::Once));
            },
            MalfunctionType::Collision => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "The ship is on a trajectory to collide with debree!".to_string(),
                });
                malfunction.malfunction_timers.push(Timer::new(Duration::from_secs_f32(TIME_TO_RESOLVE), TimerMode::Once));
            },
            MalfunctionType::Hack => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: true,
                    text: "A sattelite is on a collision trajectory!".to_string(),
                });
                malfunction.malfunction_timers.push(Timer::new(Duration::from_secs_f32(TIME_TO_RESOLVE), TimerMode::Once));
            },
            MalfunctionType::Waves => {
                malfunction.malfunction_types.push(malfunc_type);
                malfunction.warning_data.push(WarningData {
                    color: true,
                    text: "Antenna malfunctioned!".to_string(),
                });
                malfunction.malfunction_timers.push(Timer::new(Duration::from_secs_f32(TIME_TO_RESOLVE1), TimerMode::Once));
            },
            MalfunctionType::Engine => {
                malfunction.malfunction_types.push(malfunc_type);
                pipe_minigame.fill_solved();
                pipe_minigame.shuffle();
                malfunction.warning_data.push(WarningData {
                    color: false,
                    text: "Engine malfunctioned!".to_string(),
                });
                malfunction.malfunction_timers.push(Timer::new(Duration::from_secs_f32(TIME_TO_RESOLVE1), TimerMode::Once));
            },
            MalfunctionType::NoMalfunction => unreachable!()
        };
        println!("new malfunc: {:?}", malfunction.warning_data[malfunction.warning_data.len() - 1].text);
    }
}

pub fn get_random_range(mi: f32, ma: f32) -> f32 {
    let rand = getrandom::u32().unwrap() as f32 / (u32::MAX as f32);
    mi + rand * (ma - mi) // TODO: IS THERE MA + 1????
}

const WAVE_COST: f32 = 0.0004;
const HACK_COST: f32 = 0.0004;

pub fn resolve_malfunctions(
    mut malfunction: ResMut<Malfunction>,
    mut debree_level: ResMut<DebreeLevel>,
    mut energy: ResMut<Energy>,
    mut event_writer: EventWriter<GameEndEvent>,
) {
    if !malfunction.resolved.is_empty() {
        for resolved in malfunction.resolved.clone() {
            let index = malfunction.malfunction_types.iter().position(|r: &MalfunctionType| r == &resolved.resolved_type);
            let Some(index) = index else {warn!("NOTHING TO REMOVE?"); continue};
            let to_be_resolved = malfunction.malfunction_types.remove(index);
            malfunction.malfunction_timers.remove(index);
            match to_be_resolved {
                MalfunctionType::Hack => {
                    if resolved.failed {
                        debree_level.const_add += HACK_COST;
                        println!("failed hack");
                    } else {
                        debree_level.const_add -= HACK_COST / 2.;
                        println!("resolved hack");
                    }
                },
                MalfunctionType::Collision => {
                    if resolved.failed {
                        println!("failed collision"); // end
                        event_writer.write(GameEndEvent);
                    } else {
                        println!("resolved collision"); // go on
                    }
                },
                MalfunctionType::Reactor => {
                    if resolved.failed {
                        energy.generated *= 0.9;
                        println!("failed reactor");
                    } else {
                        energy.generated *= 1.1;
                        println!("resolved reactor");
                    }
                },
                MalfunctionType::Waves => {
                    if resolved.failed {
                        debree_level.const_add += WAVE_COST;
                        println!("failed waves");
                    } else {
                        debree_level.const_add -= WAVE_COST / 2.;
                        println!("resolved waves");
                    }
                },
                MalfunctionType::Engine => {
                    if resolved.failed {
                        event_writer.write(GameEndEvent);
                        println!("failed engine"); // end
                    } else {
                        println!("resolved engine");
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

pub const TIME_TO_RESOLVE: f32 = 60.;
pub const TIME_TO_RESOLVE1: f32 = TIME_TO_RESOLVE * 2.;

pub fn tick_malfunctions(
    mut malfunction: ResMut<Malfunction>,
    time: Res<Time>,
) {
    // TODO: check for resolved errors and nulling locals after failing
    for idx in 0..malfunction.malfunction_types.len() {
        let timer = &mut malfunction.malfunction_timers[idx];
        timer.tick(Duration::from_secs_f32(time.dt()));
        if timer.finished() {
            let resolved_type = malfunction.malfunction_types[idx].clone();
            println!("FAILED DUE TO TIME {:?}", resolved_type);
            malfunction.resolved.push(Resolved {
                resolved_type,
                failed: true,
            });
        }
    }
}

#[derive(Event)]
pub struct GameEndEvent;

pub fn end_game(
    debree_level: Res<DebreeLevel>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GlobalAppState>>,
    mut event_reader: EventReader<GameEndEvent>,
    mut event_writer: EventWriter<PlaySoundEvent>,
    asset_server: Res<AssetServer>,
    mut cmd: Commands,
    ui: Single<Entity, With<LowresUiContainer>>,
    c: Query<&Children, With<LowresUiContainer>>
) {
    let mut end = false;
    for _event in event_reader.read() {
        end = true;
        println!("END {}", time.elapsed_secs());
    }
    if debree_level.chain_reaction >= 100. {
        end = true;
        println!("END {}", time.elapsed_secs());
    }
    if end {
        event_writer.write(PlaySoundEvent::Boom);
        next_state.set(GlobalAppState::Defeat);
        for c in c {
            for c in c {
                cmd.entity(*c).despawn();
            }
        }
        cmd.entity(*ui).insert((
            tw!("flex w-full h-full bg-black items-center content-center justify-center"),
            children![(
                Text::new("YOUR SPACESHIP CRASHED INTO THE DEBRIS..."),
                TextFont {
                    font: asset_server.load("fonts/orp_regular.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(200, 200, 200)),
                tw!("z-10"),
            )]
        ));
    }
    
}

