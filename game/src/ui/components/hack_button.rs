#![allow(unused)]
use bevy::prelude::*;



pub const HACK_BUTTON_SRC: &str = "ui/hack_button.png";
pub fn hack_button_bundle(a: &Res<AssetServer>, t: &mut ResMut<Assets<TextureAtlasLayout>>) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 6, 6, None, None);
    (a.load(HACK_BUTTON_SRC),
    t.add(layout))
}

pub fn ui_hack_button(
    (h, a): &(Handle<Image>, Handle<TextureAtlasLayout>),
    btn: HackButton,
    component: impl Bundle,
) -> impl Bundle {
    (
        ImageNode::from_atlas_image(h.clone(), TextureAtlas{layout: a.clone(), index: btn.get_idx(false, false)},),
        Interaction::default(),
        BoxShadow(vec![ShadowStyle {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(5.0),
            spread_radius: Val::Px(5.0),
            blur_radius: Val::Px(5.0),
        }]),
        btn,
        component
    )
}


#[derive(Component)]
pub struct HackButton {
    pub state: HackButtonState,
    pub index: usize,
}

impl HackButton {
    pub fn get_idx(&self, hovered: bool, pressed: bool) -> usize {
        match self.state {
            HackButtonState::Disabled => {
                0 + if pressed {18} else {0}
            },
            HackButtonState::Enabled => {
                self.index + 1 + if pressed {18} else {0}
            },
            HackButtonState::Active => {
                if hovered && !pressed{
                    return self.index + 1 + 12;
                }
                self.index + 1 + 6 + if pressed {18 + 6} else {0}
            },
            HackButtonState::SuperActive => {
                self.index + 1 + 12 + if pressed {18} else {0}
            },
        }
    }
}

pub enum HackButtonState {
    Disabled,
    Enabled,
    Active,
    SuperActive
}

pub fn ui_hack_button_hover(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut ImageNode,
            &mut HackButton,
        ),
        Changed<Interaction>,
    >,
    t: Res<Time>
) {
    for (entity, interaction, mut node, hack) in
        &mut interaction_query
    {
        let h = Interaction::Hovered == *interaction;
        let p = Interaction::Pressed == *interaction;
        if let Some(a) = &mut node.texture_atlas {
            a.index = hack.get_idx(h, p);
        }
    }
}

pub const HACK_BUTTON_NAMES : [&'static str ; 5] = [
    "AA",
    "5E",
    "2D",
    "3G",
    "4A"
];