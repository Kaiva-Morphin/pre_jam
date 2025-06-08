use bevy::prelude::*;


pub fn ui_atlas_container(
    (handle, atlas): &(Handle<Image>, Handle<TextureAtlasLayout>),
    children: impl Bundle,
) -> impl Bundle {
    (
        ImageNode::from_atlas_image(handle.clone(), TextureAtlas{layout: atlas.clone(), index: 0},),
        children,
        Name::new("Ui atlas container"),
    )
}