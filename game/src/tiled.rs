use bevy::prelude::*;
use bevy::scene::SceneLoader;
use bevy_inspector_egui::bevy_egui::{EguiContextPass, EguiContexts};
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use bevy_ecs_tiled::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use pixel_utils::camera::HIGH_RES_LAYERS;
use ::utils::{ExpDecay, MoveTowards, WrappedDelta};
use core::plugin::CorePlugin;
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

use crate::camera::plugin::CameraFocus;
use crate::physics::animator::{PlayerAnimationNode, PlayerAnimations};
use crate::physics::controller::{Controller, ControllersPlugin};
use crate::physics::player::PlayerPlugin;
use crate::tilemap::plugin::MapPlugin;
use crate::utils::background::StarBackgroundPlugin;

use bevy_ecs_tilemap::TilemapPlugin;
mod core;
mod camera;
mod utils;
mod physics;
mod interactions;
mod ui;
mod tilemap;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            DebugOverlayPlugin::default(),
            SwitchableRapierDebugPlugin::disabled(),
            StarBackgroundPlugin,
            MapPlugin,
            PlayerPlugin
        ))
        .add_systems(EguiContextPass, debug)
        .run();
}


fn debug(
    mut contexts: EguiContexts,
    mut player: Single<(Entity, &mut AnimationPlayer)>,
    mut animations: ResMut<PlayerAnimations>,
) {
    let ctx = contexts.ctx_mut();
    let (_e, p) = &mut *player;
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.heading("Weights");
        for (a, i) in animations.nodes().clone().iter() {
            ui.horizontal(|ui|{
                let t= ui.button(format!("{:?}: ", a));
                if t.clicked() {
                    animations.target = *a;
                }
                let a = p.animation_mut(*i);
                let Some(a) = a else {return;};
                let mut w = a.weight();
                let r = ui.add(Slider::new(&mut w, 0.0..=1.0));
                if r.changed() {
                    a.set_weight(w);
                }
            });
        }
        ui.separator();
        ui.heading(format!("Targeting: {:?}", animations.target));
    });
}













