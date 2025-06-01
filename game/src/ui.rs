use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::text::FontSmoothing;
use bevy_rapier2d::prelude::*;
use debug_utils::debug_overlay::DebugOverlayPlugin;
use debug_utils::inspector::plugin::SwitchableEguiInspectorPlugin;
use debug_utils::rapier::plugin::SwitchableRapierDebugPlugin;
use bevy_ecs_tiled::prelude::*;
use pixel_utils::camera::{PixelCamera, PixelCameraPlugin};
use core::CorePlugin;

use crate::camera::plugin::CameraFocus;
use crate::physics::controller::{Controller, ControllersPlugin};
use crate::physics::scene::{spawn_player, Player};

use bevy_ecs_tilemap::TilemapPlugin;
mod core;
mod camera;
mod utils;
mod physics;
mod interactions;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((CorePlugin,
            SwitchableEguiInspectorPlugin::default(),
            DebugOverlayPlugin::default(),
        ))
        .add_systems(Startup, start)
        .run();
}

pub fn start(
    mut cmd: Commands,
    pc: Single<Entity, With<PixelCamera>>,
    asset_server: Res<AssetServer>,
){
    let image = asset_server.load("raw/ui.png");

    let slicer = TextureSlicer {
        border: BorderRect::all(10.0),
        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
        max_corner_scale: 1.0,
    };

    cmd
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
        
            ..default()
        },
        UiTargetCamera(*pc)
        ))
        .with_children(|parent| {
            for [w, h] in [[100.0, 150.0], [300.0, 150.0], [100.0, 300.0]] {
                parent
                    .spawn((
                        Button,
                        ImageNode {
                            image: image.clone(),
                            image_mode: NodeImageMode::Sliced(slicer.clone()),
                            ..default()
                        },
                        Node {
                            width: Val::Px(w),
                            height: Val::Px(h),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ))
                    .with_child((
                        Text::new("Text"),
                        TextFont {
                            font_smoothing: FontSmoothing::None,
                            font: asset_server.load("fonts/orp_regular.ttf"),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
            }
        });
}


