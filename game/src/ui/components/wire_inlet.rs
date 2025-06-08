use bevy::{prelude::*, ui::RelativeCursorPosition};

pub const WIRE_INLET_SRC: &str = "ui/wire_plug.png";
pub fn wire_inlet_bundle(a: &Res<AssetServer>) -> Handle<Image> {
    a.load(WIRE_INLET_SRC)
}

pub fn ui_wire_inlet(
    handle: &Handle<Image>,
    color: Color,
    children: impl Bundle,
) -> impl Bundle {
    (
        ImageNode{
            image : handle.clone(),
            color: color, ..Default::default()
        },
        Interaction::default(),
        RelativeCursorPosition::default(),
        children,
        Name::new("WireInlet"),
    )
}