use bevy::prelude::*;
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};
use std::collections::HashMap;
use crate::{interactions::components::{InInteractionArray, InteractionTypes}, physics::player::Player, tilemap::light::LIT_OVERLAY_LAYER, utils::debree::{Malfunction, MalfunctionType}};











pub struct MalfunctionGuider;
impl Plugin for MalfunctionGuider {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, guide) ;       
    }
}

#[derive(Component)]
struct GuideArrow;

pub fn guide(
    mut cmd: Commands,
    malf: Query<(&GlobalTransform, &InteractionTypes)>,
    cam: Single<&GlobalTransform, With<PixelCamera>>,
    malfunction: Res<Malfunction>,
    in_interaction_array: Res<InInteractionArray>,
    mut arrows: Local<HashMap<InteractionTypes, Entity>>,
    asset_server: Res<AssetServer>,
    player: Single<&GlobalTransform, With<Player>>,
){
    let target_size = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
    let i = asset_server.load("pixel/arrow.png");
    let i2 = asset_server.load("pixel/arrow2.png");
    for (pos, t) in malf.iter() {
        let malf = t.as_malfunction();
        if malf == MalfunctionType::NoMalfunction {continue;}
        // let h = &mut *arrows;
        let e = if let Some(e) = arrows.get(t) {e} else {
            let i = if matches!(t, InteractionTypes::PipePuzzle | InteractionTypes::CollisionMinigame) {i.clone()} else {i2.clone()};

            let ne = cmd.spawn((
                Sprite::from_image(i),
                GuideArrow,
                GlobalTransform::default(),
                Transform::default(),
                LIT_OVERLAY_LAYER,
            )).id();
            arrows.insert(t.clone(), ne);
            arrows.get(t).unwrap()
        };
        let dir = pos.translation().truncate() - cam.translation().truncate();
        
        // pos += up * target_size * vec2(1.0, -1.0);
        let rp = dir.to_angle();
        let l = dir.length();
        let dir = dir.normalize_or_zero();
        let att = 100.0;
        let min_att = 30.0;
        let pos = cam.translation().truncate() + dir * l.min(att);
        // info!("{:?} {} {}", malf,  l < min_att, malfunction.malfunction_types.contains(&malf));
        cmd.entity(*e).insert((
            Transform::from_translation(pos.extend(0.)).with_rotation(Quat::from_rotation_z(rp)),
            if l < min_att || !malfunction.malfunction_types.contains(&t.as_malfunction()) {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            }
        ));
    }
}

