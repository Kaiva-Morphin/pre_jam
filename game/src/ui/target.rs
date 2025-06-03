use bevy::{prelude::*, window::WindowResized};
use debug_utils::debug_overlay::DebugOverlayRoot;
use pixel_utils::camera::{true_pixel_switch, PixelCameraVars, PIXEL_SWITCH_TRIGGER, TARGET_HEIGHT, TARGET_WIDTH};

use crate::core::debug_ui_to_camera;



pub struct UiRetargetPlugin;

impl Plugin for UiRetargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (retarget_debug.after(debug_ui_to_camera), init).chain());
        app.add_systems(PreUpdate, resize.after(true_pixel_switch));
    }
}

#[derive(Component)]
pub struct HighresUiRoot;


#[derive(Component)]
pub struct LowresUiContainer;



pub fn init(
    mut cmd: Commands,
    debug: Option<Single<Entity, With<DebugOverlayRoot>>>,
){
    let r = cmd.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        HighresUiRoot,
        Name::new("HighresUiRoot"),
        // BackgroundColor(Color::linear_rgba(1.0, 0.0, 0.0, 0.3)),
    )).id();
    let lowres = cmd.spawn((
        Node {
            width: Val::Px(TARGET_WIDTH as f32),
            height: Val::Px(TARGET_HEIGHT as f32),
            ..default()
        },
        Name::new("LowresUiContainer"),
        LowresUiContainer,
        // BackgroundColor(Color::linear_rgba(0.0, 1.0, 0.0, 0.3)),
    )).id();
    cmd.entity(r).add_child(lowres);

    if let Some(debug) = debug {
        cmd.entity(lowres).add_child(*debug);
    }
}

pub fn retarget_debug(
    mut cmd: Commands,
    root: Single<Entity, With<DebugOverlayRoot>>,
){
    cmd.entity(*root).remove::<UiTargetCamera>();
}

pub fn resize(
    keys: Res<ButtonInput<KeyCode>>,
    mut resize_events: EventReader<WindowResized>,
    v: Res<PixelCameraVars>,
    mut s: ResMut<UiScale>
){
    if keys.just_pressed(PIXEL_SWITCH_TRIGGER){
        // info!("Retarget: {} -> {}", s.0, v.scale());
        s.0 = v.scale();
        return;
    }
    for _ in resize_events.read() {
        // info!("Retarget: {} -> {}", s.0, v.scale());
        s.0 = v.scale();
        return;
    }
}


