use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::interactions::wave_modulator::SpinnyIds;

pub fn ui_spinny(
    (handle, atlas): &(Handle<Image>, Handle<TextureAtlasLayout>),
    spinny: SpinnyIds,
    children: impl Bundle,
) -> impl Bundle {
    (
        ImageNode::from_atlas_image(handle.clone(), TextureAtlas{layout: atlas.clone(), index: 0},),
        Interaction::default(),
        RelativeCursorPosition::default(),
        spinny,
        children,
        Name::new("Spinny"),
    )
}