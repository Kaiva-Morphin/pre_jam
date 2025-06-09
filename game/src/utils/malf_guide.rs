use bevy::prelude::*;
use pixel_utils::camera::PixelCamera;
use std::collections::HashMap;
use crate::{interactions::components::{InInteractionArray, InteractionTypes}, utils::debree::MalfunctionType};











pub struct MalfunctionGuider;
impl Plugin for MalfunctionGuider {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, guide) ;       
    }
}


pub fn guide(
    malf: Query<&GlobalTransform, With<InteractionTypes>>,
    cam: Single<&GlobalTransform, With<PixelCamera>>,
    in_interaction_array: Res<InInteractionArray>,
    tracked: Local<HashMap<MalfunctionType, Entity>>,
){
    info!("{:?}", malf);
}

