use bevy::{asset, prelude::*};
use pixel_utils::camera::PixelCamera;

use crate::camera::plugin::camera_controller;


pub struct StarBackgroundPlugin;

impl Plugin for StarBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(PreUpdate, update.after(camera_controller));
    }
}

#[derive(Component)]
pub struct ParalaxLayer{
    pub offset: f32,
    pub size: Vec2,
}

fn setup(
    mut cmd: Commands,
    assets: Res<AssetServer>,
){
    cmd.spawn((Sprite{
        image: assets.load("textures/background/nebula.png"),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
        },
        custom_size: Some(vec2(3072.0, 3072.0)),
        ..default()
        },
        Transform::from_translation(vec3(0.0, 0.0, -900.0)),
        ParalaxLayer{
            offset: 0.4,
            size: Vec2::splat(1024.0)
        },
    ));

    for i in 0..4 {
        cmd.spawn((Sprite{
            image: assets.load("textures/background/stars.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            color: Color::srgba(1.0, 1.0, 1.0, 0.2 + 0.2 * i as f32),
            custom_size: Some(vec2(512.0, 512.0) * 3.0),
            flip_x: i % 2 == 0,
            flip_y: i / 2 == 0,
            ..default()
            },
            Transform::from_translation(vec3(0.0, 0.0, -900.0 + -10.0 * i as f32 - 10.0)),
            ParalaxLayer{
                offset: 0.6 - i as f32 * 0.15,
                size: Vec2::splat(512.0)
            },
        ));
    }   


    
}



fn update(
    mut layers: Query<(&mut Transform, &ParalaxLayer)>,
    camera: Single<&GlobalTransform, With<PixelCamera>>,
){
    let cam_pos = camera.translation().truncate();
    for (mut transform, layer) in layers.iter_mut() {
        transform.translation = ((cam_pos).div_euclid(layer.size) * layer.size + (cam_pos * layer.offset).rem_euclid(layer.size)).extend(transform.translation.z);
        // let mut t = (cam_pos).div_euclid(layer.size) ;
        // if t.x.is_sign_negative() {t.x += layer.size.x}
        // if t.y.is_sign_negative() {t.y += layer.size.y}
        // transform.translation = ((t) * layer.size  + (cam_pos * layer.offset) % (layer.size * 0.5)).extend(transform.translation.z);

    }
}