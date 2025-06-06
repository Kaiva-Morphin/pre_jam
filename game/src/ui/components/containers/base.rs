#![allow(unused)]

use bevy::prelude::*;
use bevy_tailwind::tw;

pub const MAIN_CONTAINER_SRC: &str = "ui/main_container.png";
pub fn main_container_handle(a: &Res<AssetServer>) -> Handle<Image> { a.load(MAIN_CONTAINER_SRC) }
pub const MAIN_CONTAINER_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect::all(16.0),
    center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    max_corner_scale: 1.0,
};


pub fn ui_main_container(h: &Handle<Image>, component: impl Bundle) -> impl Bundle {
    (
        ImageNode {
            image: h.clone(),
            image_mode: NodeImageMode::Sliced(MAIN_CONTAINER_SLICER),
            ..default()
        },
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        component
    )
}


pub const SUB_CONTAINER_SRC: &str = "ui/sub_container.png";
pub fn sub_container_handle(a: &Res<AssetServer>) -> Handle<Image> { a.load(SUB_CONTAINER_SRC) }
pub const SUB_CONTAINER_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect::all(16.0),
    center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    max_corner_scale: 1.0,
};
pub fn ui_sub_container(h: &Handle<Image>, component: impl Bundle) -> impl Bundle {
    (
        ImageNode {
            image: h.clone(),
            image_mode: NodeImageMode::Sliced(SUB_CONTAINER_SLICER),
            ..default()
        },
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        component
    )
}




