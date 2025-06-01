use bevy::{prelude::*, window::PrimaryWindow};
use pixel_utils::camera::PixelCamera;

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
    camera_query: Single<(&Camera, &GlobalTransform), With<PixelCamera>>,
    windows: Single<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
){
    let (camera, camera_transform) = *camera_query;
    let window = *windows;
    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera.viewport_to_world_2d(camera_transform, screen_position).unwrap();
        cursor_position.screen_position = screen_position;
        cursor_position.world_position = world_position;
    }
}