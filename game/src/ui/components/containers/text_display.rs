#![allow(unused)]

use bevy::prelude::*;
use bevy_tailwind::tw;

pub const TEXT_DISPLAY_GREEN_SRC: &str = "ui/text_display_green.png";
pub fn text_display_green_handle(a: &Res<AssetServer>) -> Handle<Image> { a.load(TEXT_DISPLAY_GREEN_SRC) }
pub const TEXT_DISPLAY_GREEN_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect::all(5.0),
    center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
    max_corner_scale: 1.0,
};


pub fn ui_text_display_green(h: &Handle<Image>, component: impl Bundle) -> impl Bundle {
    (
        ImageNode {
            image: h.clone(),
            image_mode: NodeImageMode::Sliced(TEXT_DISPLAY_GREEN_SLICER),
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

pub fn ui_text_display_green_with_text(h: &Handle<Image>, component: (impl Bundle, impl Bundle), text: &str, asset_server : &Res<AssetServer>) -> impl Bundle {
    ui_text_display_green(h, (
        children![(
            Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/orp_regular.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(82, 112, 67)),
                tw!("z-10"),
                component.0
            ),
            (
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/orp_regular.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                tw!("absolute mt-[4px] ml-[4px]"),
                TextColor(Color::srgb_u8(21, 24, 19)),
                component.1
            )
        ],
        // вадим я перетащил компонент внутрь я вообще не ебу к хуям он тут нужен вне текста 🤷‍♂️🤷‍♂️🤷‍♂️
    ))
}