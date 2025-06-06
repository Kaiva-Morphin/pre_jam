#![allow(unused)]

use bevy::prelude::*;
use bevy_tailwind::tw;


pub const VIEWPORT_SRC: &str = "ui/viewport.png";
pub fn viewport_handle(a: &Res<AssetServer>) -> Handle<Image> { a.load(VIEWPORT_SRC) }
pub const VIEWPORT_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect::all(4.0),
    center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    max_corner_scale: 1.0,
};


pub fn ui_viewport_container(h: &Handle<Image>, component: impl Bundle) -> impl Bundle {
    (
        ImageNode {
            image: h.clone(),
            image_mode: NodeImageMode::Sliced(VIEWPORT_SLICER),
            ..default()
        },
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        component
    )
}