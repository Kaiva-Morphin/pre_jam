use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};
use utils::{wrap, ExpDecay, WrappedDelta};
use pixel_utils::camera::{setup_camera, PixelCamera};

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
        ZoomTarget(1.0)
    }
}

pub fn setup(
    pixel_camera: Single<Entity, With<PixelCamera>>,
    mut cmd: Commands
){
    cmd.entity(*pixel_camera).insert(CameraMode::Following);
}

const CAMERA_SPEED: f32 = 0.5;
const CAMERA_FOLLOW_SPEED: f32 = 4.0;

pub fn camera_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pixel_camera: Single<(&mut Projection, &mut Transform, &mut CameraMode), With<PixelCamera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut target_zoom: Local<ZoomTarget>,
    to_focus: Query<(&GlobalTransform, &CameraFocus)>,
    time: Res<Time>,
){
    let dt = time.dt();
    let (projection, camera_transform, mode) = &mut *pixel_camera;
    
    let mut follow: Option<Vec3> = None;
    let mut p = 0;
    for (t, f) in to_focus.iter() {
        if let Some(_) = follow {
            if p < f.priority {continue;}
            follow = Some(t.translation());
            p = f.priority;
        } else {
            follow = Some(t.translation());
            p = f.priority;
        }
    };
    
    let Projection::Orthographic(projection) = &mut **projection else {return;}; 
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        if **mode == CameraMode::Following {
            **mode = CameraMode::Free;
        } else {
            **mode = CameraMode::Following;
        }
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

    match **mode {
        CameraMode::Following => {
            let target = follow.unwrap_or(Vec3::ZERO);
            m_dt.z = 0.0;
            camera_transform.translation = camera_transform.translation.exp_decay(target, CAMERA_FOLLOW_SPEED, dt);
        } 
        CameraMode::Free => {
            m_dt.z = 0.0;
            camera_transform.translation += m_dt * target_scale * CAMERA_SPEED;
        }
    }
}