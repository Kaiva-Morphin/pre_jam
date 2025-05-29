use bevy::{prelude::*, winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage}};



pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_cursor_icon)
            .add_systems(PreUpdate, update_cursor_icon);
    }
}

fn setup_cursor_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window: Single<Entity, With<Window>>,
) {
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);


    commands.entity(*window).insert((
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            // Image to use as the cursor.
            handle: asset_server
                .load("pixel/cursor.png"),
            // Optional texture atlas allows you to pick a section of the image
            // and animate it.
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            flip_x: false,
            flip_y: false,
            // Optional section of the image to use as the cursor.
            rect: None,
            // The hotspot is the point in the cursor image that will be
            // positioned at the mouse cursor's position.
            hotspot: (15, 15),
        })),
    ));
}

fn update_cursor_icon(
    mut cursor: Single<&mut CursorIcon>,
    time: Res<Time>,
){
    let CursorIcon::Custom(CustomCursor::Image(c)) = &mut **cursor else {return;};
    let Some(a) = &mut c.texture_atlas else {return;};
    let t = time.elapsed_secs() * 16.0;
    a.index = t.floor() as usize % 3;
}