use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;
use bevy_ecs_tiled::world::asset;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, Sensor};
use bevy_tailwind::tw;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;

use core::plugin::CorePlugin;

use crate::core::states::OnGame;
use crate::physics::constants::{INTERACTABLE_CG, PLAYER_SENSOR_CG};
use crate::physics::player::{spawn_player, Player, PlayerPlugin};
use crate::tilemap::plugin::MapPlugin;
use crate::ui::components::containers::base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container};
use crate::ui::components::containers::text_display::{text_display_green_handle, ui_text_display_green_with_text};
use crate::ui::components::hack_button::{hack_button_bundle, ui_hack_button, HackButton, HackButtonState};
use crate::ui::target::LowresUiContainer;
use crate::utils::background::StarBackgroundPlugin;

mod core;
mod ui;
mod camera;
mod utils;
mod physics;
mod interactions;
mod tilemap;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            CorePlugin,
            StarBackgroundPlugin,
            // PlayerPlugin,
            // MapPlugin,
            SwitchableEguiInspectorPlugin::default(),
            SwitchableRapierDebugPlugin::default(),
            DebugOverlayPlugin::default(),
        ))
        .add_systems(OnGame, spawn.after(spawn_player))
        .run();
}






pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    e: Single<Entity, With<LowresUiContainer>>,
) {
    // let crt = asset_server.load("ui/crt_overlay.png");
    let hack = hack_button_bundle(&asset_server, &mut texture_atlases);
    let text = text_display_green_handle(&asset_server);
    let main = main_container_handle(&asset_server);
    let sub = sub_container_handle(&asset_server);
    panic!("ВАДИМ Я ПОМЕНЯЛ ui_text_display_green_with_text");
    commands.entity(*e).with_children(|cmd| {
        cmd.spawn((
            tw!("items-center justify-center w-full h-full"),
            children![
                ui_main_container(&main, children![
                    (
                    tw!("flex flex-row items-center justify-center gap-px"),
                    children![
                        // ui_text_display_green_with_text(&text, (), "bebra :D", &asset_server), 
                        ui_main_container(&sub, children![
                        (
                            tw!("flex flex-col items-center justify-center gap-px"),
                            children![
                                ui_main_container(&main, children![(
                                    ui_hack_button(&hack, HackButton{state: HackButtonState::Disabled, index: 2}, ()),
                                )]),
                                ui_main_container(&main, children![(
                                    ui_hack_button(&hack, HackButton{state: HackButtonState::Enabled, index: 3}, ()),
                                )]),
                                ui_main_container(&main, children![(
                                    ui_hack_button(&hack, HackButton{state: HackButtonState::Active, index: 3}, ()),
                                )]),
                                ui_main_container(&main, children![(
                                    ui_hack_button(&hack, HackButton{state: HackButtonState::SuperActive, index: 3}, ()),
                                )]),
                            ]),
                        ]),
                    ],
                    ),
                ])
            ],
        ));
    });
}
