use std::{collections::BTreeMap, sync::Arc};

use bevy::{audio::{AudioPlugin, SpatialScale}, ecs::system::SystemParam, prelude::*, state::app::StatesPlugin, text::FontStyle, window::WindowResolution, winit::{cursor::{CursorIcon, CustomCursor, CustomCursorImage}, WinitWindows}};
use bevy_inspector_egui::{bevy_egui::{EguiContexts, EguiPlugin, EguiPreUpdateSet}, egui::{self, style::TextCursorStyle, CornerRadius, Stroke, Style, TextStyle, Visuals}};
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{BevyPhysicsHooks, ContactModificationContextView}, render::RapierDebugRenderPlugin};
use debug_utils::debug_overlay::DebugOverlayRoot;
use pixel_utils::camera::{PixelCamera, PixelCameraPlugin};

use crate::{camera::plugin::CameraControllerPlugin, core::states::GameStatesPlugin, interactions::InteractionsPlugin, physics::platforms::PlatformsPlugin, ui::{plugin::UiSystemPlugin, target::UiRetargetPlugin}, utils::{cursor::CursorPlugin, custom_material_loader::SpritePreloadPlugin, debree::DebreePlugin, malf_guide::MalfunctionGuider, mouse::CursorPositionPlugin, spacial_audio::SpacialAudioPlugin}};

const AUDIO_SCALE: f32 = 1. / 100.0;

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                (DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            // resolution: WindowResolution::new(1000., 1000.),
                            title: "Game".to_string(),
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: true,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(AssetPlugin {
                        // meta_check: bevy::asset::AssetMetaCheck::Never,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest())
                    .set(AudioPlugin {
                        default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                        ..default()
                    }),
                RapierPhysicsPlugin::<()>::pixels_per_meter(16.0),
                EguiPlugin { enable_multipass_for_primary_context: true },
                GameStatesPlugin,
                PixelCameraPlugin,
                CameraControllerPlugin,
                UiSystemPlugin),
                UiRetargetPlugin,
                CursorPlugin,
                SpacialAudioPlugin,
                MalfunctionGuider,
                bevy_framepace::FramepacePlugin,
                InteractionsPlugin,
                CursorPositionPlugin,
                SpritePreloadPlugin,
                DebreePlugin,
            ))
            .insert_resource(bevy_framepace::FramepaceSettings{limiter: bevy_framepace::Limiter::from_framerate(60.0)})
            .add_systems(Startup, init_egui_font.after(EguiPreUpdateSet::InitContexts))
            .add_systems(PreStartup, debug_ui_to_camera.after(pixel_utils::camera::setup_camera).after(debug_utils::debug_overlay::init))
        ;
    }
}

#[derive(SystemParam)]
pub struct FrictionPhysicsHook;

impl BevyPhysicsHooks for FrictionPhysicsHook {
    fn modify_solver_contacts(&self, context: ContactModificationContextView) {
        let normal = context.raw.normal.normalize();
        let is_x_aligned = normal.y.abs() < 0.1;

        for solver_contact in &mut *context.raw.solver_contacts {
            if is_x_aligned {
                solver_contact.friction = 0.0;
            }
        }
    }
}


pub fn debug_ui_to_camera(
    mut cmd: Commands,
    pc: Single<Entity, With<PixelCamera>>,
    root: Single<Entity, With<DebugOverlayRoot>>,
){
    cmd.entity(*root).insert(UiTargetCamera(*pc));
}


pub fn init_egui_font(
   mut egui_context: EguiContexts,
){
    let ctx: &mut egui::Context = egui_context.ctx_mut();

    let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("Font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!("../../assets/fonts/orp_regular.ttf")))
    );

    fonts.families.insert(egui::FontFamily::Name("Font".into()), vec!["Font".to_owned()]);
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "Font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .insert(0, "Font".to_owned());

    ctx.set_fonts(fonts);

    let style = Style {
        //override_text_style: Some(egui::TextStyle::Monospace),
        
        //drag_value_text_style: todo!(),
        //wrap: todo!(),
        //spacing: todo!(),
        //interaction: todo!(),
        text_styles: BTreeMap::from([
            (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(20.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
        ]),
        visuals: Visuals {
            dark_mode: true,
            override_text_color: Some(egui::Color32::from_rgba_unmultiplied(220, 220, 220, 255)),
            //widgets: todo!(),
            //selection: todo!(),
            //hyperlink_color: todo!(),
            //faint_bg_color: todo!(),
            
            // window_rounding: Rounding::ZERO,
            window_shadow: egui::Shadow::NONE,
            window_fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 230),
            // window_stroke: egui::Stroke{
            //     width: 1.,
            //     color: egui::Color32::from_rgba_unmultiplied(220, 220, 220, 255)
            // },
            window_stroke: Stroke::NONE,
            button_frame: false,
            interact_cursor: None,
            menu_corner_radius: CornerRadius::ZERO,
            window_corner_radius: CornerRadius::ZERO,
            
            //menu_rounding: todo!(),
            //panel_fill: todo!(),
            //popup_shadow: todo!(),
            //resize_corner_size: todo!(),
            //text_cursor_width: todo!(),
            //text_cursor_preview: todo!(),
            //clip_rect_margin: todo!(),
            //button_frame: todo!(),
            //collapsing_header_frame: todo!(),
            //indent_has_left_vline: todo!(),
            //striped: todo!(),
            //slider_trailing_fill: todo!(),
            ..default()
        },
        animation_time: 0.,
        //debug: todo!(),
        //explanation_tooltips: todo!(),
        ..default()
    };

    ctx.set_style(style.clone());
}