use bevy::{prelude::*, ui::RelativeCursorPosition};

pub const WIRE_INLET_SRC: &str = "interactables/ururur.png";
pub fn wire_inlet_bundle(a: &Res<AssetServer>) -> Handle<Image> {
    a.load(WIRE_INLET_SRC)
}

pub fn ui_wire_inlet(
    handle: &Handle<Image>,
    children: impl Bundle,
) -> impl Bundle {
    (
        ImageNode::from(handle.clone()),
        Interaction::default(),
        RelativeCursorPosition::default(),
        children,
        Name::new("WireInlet"),
    )
}