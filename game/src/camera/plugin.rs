use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};
use utils::wrap;
use std::ops::{Deref, DerefMut};
use pixel_utils::camera::PixelCamera;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, camera_controller);
    }
}



#[derive(Component)]
enum CameraMode {
    Free,
    Following,
}

#[derive(Component)]
struct CameraFocus{
    priority: usize,
    entity: Entity,
    speed: f32,
    attention : f32,
}




wrap!(pub ZoomTarget(pub f32));

impl Default for ZoomTarget {
    fn default() -> Self {
        ZoomTarget(1.0)
    }
}


const CAMERA_SPEED: f32 = 0.5;

pub fn camera_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pixel_camera: Single<(&mut Projection, &mut Transform), With<PixelCamera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut target_zoom: Local<ZoomTarget>
){
    let (projection, camera_transform) = &mut *pixel_camera;
    let Projection::Orthographic(projection) = &mut **projection else {return;}; 
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        **target_zoom = 1.0;
        projection.scale = 1.0;
        camera_transform.translation = Vec3::ZERO;
        return;
    }
    let mut m_dt = Vec3::ZERO;
    for event in mouse_wheel_events.read() {
        m_dt.z += event.y;
    };
    if mouse.pressed(MouseButton::Middle) {
        for event in mouse_motion.read() {
            m_dt.x -= event.delta.x;
            m_dt.y += event.delta.y;
        }
    }
    **target_zoom = (**target_zoom + m_dt.z).clamp(-5.0, 40.0);
    let target_scale = (2.0_f32).powf(-**target_zoom * 0.2);
    projection.scale = target_scale;
    m_dt.z = 0.0;
    camera_transform.translation += m_dt * target_scale * CAMERA_SPEED;
}