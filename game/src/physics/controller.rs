use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

use bevy::app::FixedMain;
use bevy::color::palettes::css::{GREEN, RED};
use bevy::ecs::query::{QueryData, WorldQuery};
use bevy::input::keyboard;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::picking::pointer::PointerLocation;
use bevy::{gizmos, prelude::*};


use bevy_inspector_egui::bevy_egui::{EguiContext, EguiContextPass, EguiContexts};
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayEvent;
use debug_utils::overlay_text;
use pixel_utils::camera::PixelCamera;
use utils::{wrap, MoveTowards, WrappedDelta};

use crate::physics::animator::{PlayerAnimationNode, PlayerAnimations};
use crate::physics::player::{ControllerConstants, GlobalGravity, GravityOverride, Player, PlayerMesh, REG_FRICTION};

use super::platforms::{MovingPlatform, MovingPlatformMode};

pub struct ControllersPlugin;

impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        // app
        //     .add_systems(Update, update_controllers)
        //     .add_systems(FixedPreUpdate, tick_controllers)
        //     .add_systems(EguiContextPass, debug)
        //     .insert_resource(GlobalGravity(Vec2::new(0.0, -981. / 2.0)))
        //     ;
    }
}












pub fn debug(
    mut contexts: EguiContexts,
    mut controller: Single<&mut Controller, (With<Player>, Without<PlayerMesh>)>,
){
    let ctx = contexts.ctx_mut();
    egui::Window::new("A").show(ctx, |ui| {
        ui.heading("Controller");
        ui.label("");
        ui.add(egui::Slider::new(&mut controller.time_in_air, 0.0..=1.0));
    });
}
