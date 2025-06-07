use bevy::{prelude::*, window::PrimaryWindow};
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};

pub struct CursorPositionPlugin;

impl Plugin for CursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CursorPosition::default())
        .add_systems(Update, update_cursor_position);
    }
}

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
    pub screen_position: Vec2,
}

pub fn update_cursor_position(
    camera_query: Single<(&Camera, &GlobalTransform, &Transform), With<PixelCamera>>,
    windows: Single<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
    v: Res<pixel_utils::camera::PixelCameraVars>
){
    let (camera, camera_gtransform, camera_transform) = *camera_query;
    let window = *windows;
    let window_size = window.size();
    let target_size = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera.viewport_to_world_2d(camera_gtransform, screen_position).unwrap();
        cursor_position.screen_position = screen_position;
        cursor_position.world_position = (world_position - window_size * 0.5) / (window_size * 0.5)
        * target_size * v.scale() * 0.5 + camera_transform.translation.xy();
    }
}