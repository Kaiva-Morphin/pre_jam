use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};
use debug_utils::{debug_overlay::DebugOverlayEvent, overlay_text};
use utils::{wrap, ExpDecay, WrappedDelta};
use pixel_utils::camera::{setup_camera, PixelCamera};

use crate::physics::player::{Player, PlayerState};


pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, camera_controller)
            .add_systems(PreStartup, setup.after(setup_camera));
    }
}


#[derive(Component, PartialEq, Eq)]
pub enum CameraMode {
    Free,
    Following,
}

#[derive(Component)]
pub struct CameraFocus {
    pub priority: usize,
}



wrap!(pub ZoomTarget(pub f32));

impl Default for ZoomTarget {
    fn default() -> Self {
        ZoomTarget(0.0)
    }
}

pub fn setup(
    pixel_camera: Single<Entity, With<PixelCamera>>,
    mut cmd: Commands
){
    cmd.entity(*pixel_camera).insert(CameraMode::Following);
}

const CAMERA_SPEED: f32 = 0.5;
const CAMERA_FOLLOW_SPEED: f32 = 2.0;

pub fn camera_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pixel_camera: Single<(&mut Projection, &mut Transform, &mut CameraMode), With<PixelCamera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut target_zoom: Local<ZoomTarget>,
    to_focus: Query<(&GlobalTransform, &CameraFocus)>,
    time: Res<Time>,
    state: Single<&Player>,
){
    let dt = time.dt();
    let (projection, camera_transform, mode) = &mut *pixel_camera;
    
    let mut follow: Option<&GlobalTransform> = None;
    let mut p = 0;
    for (t, f) in to_focus.iter() {
        if let Some(_) = follow {
            if p < f.priority {continue;}
            follow = Some(t);
            p = f.priority;
        } else {
            follow = Some(t);
            p = f.priority;
        }
    };
    
    let Projection::Orthographic(projection) = &mut **projection else {return;}; 
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if **mode == CameraMode::Following {
            **mode = CameraMode::Free;
        } else {
            **mode = CameraMode::Following;
        }
    }
    
    let mut m_dt = Vec3::ZERO;
    for event in mouse_wheel_events.read() {
        let v =  event.y * if let bevy::input::mouse::MouseScrollUnit::Line = event.unit {1.0} else {1.0 / event.y.abs()};
        m_dt.z += v;
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

    match **mode {
        CameraMode::Following => {
            if let Some(target) = follow {
                m_dt.z = 0.0;
                
                if state.is_spacewalking() {
                    // camera_transform.translation = camera_transform.translation.exp_decay(target.translation(), CAMERA_FOLLOW_SPEED * 3.0, dt);
                    camera_transform.translation.smooth_nudge(&target.translation(), CAMERA_FOLLOW_SPEED, dt);
                    // camera_transform.translation = target.translation();
                    camera_transform.rotation = target.rotation();  
                } else {
                    // camera_transform.translation = camera_transform.translation.exp_decay(target.translation(), CAMERA_FOLLOW_SPEED, dt);
                    camera_transform.translation.smooth_nudge(&target.translation(), CAMERA_FOLLOW_SPEED, dt);
                    camera_transform.rotation = camera_transform.rotation.slerp(target.rotation(), dt * 5.0);
                }
            }
        } 
        CameraMode::Free => {
            m_dt.z = 0.0;
            camera_transform.rotation.z = 0.0;
            camera_transform.translation += m_dt * target_scale * CAMERA_SPEED;
        }
    }
}