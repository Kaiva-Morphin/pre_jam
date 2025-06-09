use bevy::prelude::*;
use pixel_utils::camera::PixelCamera;
use std::collections::HashMap;
use crate::{interactions::components::{InInteractionArray, InteractionTypes}, physics::player::Player, utils::debree::MalfunctionType};











pub struct MalfunctionGuider;
impl Plugin for MalfunctionGuider {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, guide) ;       
    }
}

#[derive()]
struct GuideArrow;

pub fn guide(
    malf: Query<(&InteractionTypes)>, // &GlobalTransform, 
    cam: Single<&GlobalTransform, With<PixelCamera>>,
    in_interaction_array: Res<InInteractionArray>,
    arrows: Local<HashMap<MalfunctionType, Entity>>,
    player: Single<&GlobalTransform, With<Player>>,
){
    // info!("{:?}", malf);
    // for malf in malf {}
}

