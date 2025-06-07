use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::interactions::{collision_minigame::SubmitButton, wave_modulator::SpinnyIds};

pub const SUBMIT_BUTTON_SRC: &str = "atlases/E.png";
pub fn submit_button_bundle(a: &Res<AssetServer>, t: &mut ResMut<Assets<TextureAtlasLayout>>) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(19, 21), 1, 2, None, None);
    (a.load(SUBMIT_BUTTON_SRC),
    t.add(layout))
}

pub fn ui_submit_button(
    (handle, atlas): &(Handle<Image>, Handle<TextureAtlasLayout>),
    children: impl Bundle,
) -> impl Bundle {
    (
        ImageNode::from_atlas_image(handle.clone(), TextureAtlas{layout: atlas.clone(), index: 0},),
        Interaction::default(),
        RelativeCursorPosition::default(),
        SubmitButton,
        children,
        Name::new("SubmitButton"),
    )
}

pub fn ui_submit_button_hover(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut ImageNode,
            &SubmitButton,
        ),
    >,
    t: Res<Time>
) {
    for (entity, interaction, mut node, hack) in
        &mut interaction_query
    {
        let mut index = 0;
        if *interaction == Interaction::Pressed {
            index = 1;
        }
        if let Some(a) = &mut node.texture_atlas {
            a.index = index;
        }
    }
}
