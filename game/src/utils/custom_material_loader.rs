use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use shaders::components::*;

use crate::interactions::components::{InInteraction, Interactable, InteractableKeyLink, InteractableMaterial, INTERACTABLE_CG, PLAYER_SENSOR_CG};

pub struct SpritePreloadPlugin;

impl Plugin for SpritePreloadPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            Material2dPlugin::<VelocityBufferMaterial>::default(),
            Material2dPlugin::<GrassMaterial>::default(),
            Material2dPlugin::<InteractableMaterial>::default(),
        ))
        .insert_resource(VelocityBufferHandles::default())
        .insert_resource(TextureAtlasHandes::default())
        .add_event::<SpritePreloadEvent>()
        .add_systems(Startup, (preload_sprites, create_atlas))
        .add_systems(Update, spawn_sprites);
    }

}
#[derive(Resource, Default)]
pub struct TextureAtlasHandes {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

pub const KEYS_ATLAS_COLUMNS: u32 = 2;
pub const KEYS_ATLAS_ROWS: u32 = 1;
pub const KEYS_ATLAS_SIZE: u32 = KEYS_ATLAS_COLUMNS * KEYS_ATLAS_ROWS;

pub fn create_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandes>,
) {
    let atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(64),
        KEYS_ATLAS_COLUMNS,
        KEYS_ATLAS_ROWS,
        None,
        None
    );
    let handle = texture_atlases.add(atlas);
    texture_atlas_handles.layout_handle = handle.clone();
}

pub struct SpritePreloadData {
    pub handle: Handle<Image>,
    pub pos: Vec2,
}

#[derive(Event)]
pub enum SpritePreloadEvent {
    Grass(SpritePreloadData),
    Interactable(SpritePreloadData),
    Key(Handle<Image>),
}

pub fn preload_sprites(
    asset_server: ResMut<AssetServer>,
    mut writer: EventWriter<SpritePreloadEvent>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandes>,
) {
    let sprite_handle = asset_server.load("pixel/grass.png");
    let e_handle = asset_server.load("pixel/e.png");
    texture_atlas_handles.image_handle = e_handle;
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData { handle: sprite_handle.clone(), pos: Vec2::new(-50., 10.) }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData { handle: sprite_handle.clone(), pos: Vec2::new(50., 10.) }));
}

pub fn spawn_sprites(
    mut commands: Commands,
    image_assets: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut grass_materials: ResMut<Assets<GrassMaterial>>,
    mut interactable_materials: ResMut<Assets<InteractableMaterial>>,
    buffer_handles: Res<VelocityBufferHandles>,
    mut reader: EventReader<SpritePreloadEvent>,
) {
    for event in reader.read() {
        match event {
            SpritePreloadEvent::Grass(_) => {
            }
            SpritePreloadEvent::Key(sprite_handle) => {
            }
            SpritePreloadEvent::Interactable(sprite_data) => {
                let image = image_assets.get(&sprite_data.handle).unwrap();
                let width = image.width();
                let height = image.height();
                let material = InteractableMaterial {
                    time: 0.,
                    sprite_handle: sprite_data.handle.clone()
                };
                let handle = interactable_materials.add(material);
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(width as f32 / 2., height as f32 / 2.))),
                    MeshMaterial2d(handle.clone()),
                    Transform::from_translation(sprite_data.pos.extend(0.)),
                    Name::new("Interactable"),
                    Interactable,
                    Collider::cuboid(width as f32 / 4., height as f32 / 4.),
                    CollisionGroups::new(
                        Group::from_bits(INTERACTABLE_CG).unwrap(),
                        Group::from_bits(PLAYER_SENSOR_CG).unwrap(),
                    ),
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    InInteraction {data: false},
                    InteractableKeyLink {entity: Entity::PLACEHOLDER},
                ));
            }
        }
    }
}