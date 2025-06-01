use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use shaders::components::*;

use crate::interactions::{chain_reaction_display::{ChainGraphMaterial, ChainGraphMaterialHandle}, components::{InInteraction, Interactable, InteractableMaterial, InteractionTypes, INTERACTABLE_CG, PLAYER_SENSOR_CG}};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum LoadingStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "pixel/grass.png")]
    pub grass_sprite: Handle<Image>,
    #[asset(path = "keys/e.png")]
    pub key_e_sprite: Handle<Image>,
}

pub struct SpritePreloadPlugin;

impl Plugin for SpritePreloadPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<LoadingStates>()
        .add_loading_state(
            LoadingState::new(LoadingStates::AssetLoading)
                .continue_to_state(LoadingStates::Next)
                .load_collection::<SpriteAssets>(),
        )
        .add_plugins((
            Material2dPlugin::<VelocityBufferMaterial>::default(),
            Material2dPlugin::<GrassMaterial>::default(),
            Material2dPlugin::<InteractableMaterial>::default(),
            Material2dPlugin::<ChainGraphMaterial>::default(),
        ))
        .insert_resource(VelocityBufferHandles::default())
        .insert_resource(TextureAtlasHandes::default())
        .add_event::<SpritePreloadEvent>()
        .add_systems(OnEnter(LoadingStates::Next), (preload_sprites, create_atlas))
        .add_systems(Update, spawn_sprites);
    }

}
#[derive(Resource, Default)]
pub struct TextureAtlasHandes {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

pub const KEYS_ATLAS_COLUMNS: u32 = 3;
pub const KEYS_ATLAS_ROWS: u32 = 1;
pub const KEYS_ATLAS_SIZE: u32 = KEYS_ATLAS_COLUMNS * KEYS_ATLAS_ROWS;

pub fn create_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandes>,
) {
    let atlas = TextureAtlasLayout::from_grid(
        UVec2::new(19, 21),
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
    ChainGraph(Handle<Image>),
}

pub fn preload_sprites(
    asset_server: ResMut<AssetServer>,
    mut writer: EventWriter<SpritePreloadEvent>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandes>,
    sprite_assets: Res<SpriteAssets>,
) {
    texture_atlas_handles.image_handle = sprite_assets.key_e_sprite.clone();
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData { handle: sprite_assets.grass_sprite.clone(), pos: Vec2::new(-40., 10.) }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData { handle: sprite_assets.grass_sprite.clone(), pos: Vec2::new(40., 10.) }));
}

pub fn spawn_sprites(
    mut commands: Commands,
    image_assets: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut grass_materials: ResMut<Assets<GrassMaterial>>,
    mut interactable_materials: ResMut<Assets<InteractableMaterial>>,
    buffer_handles: Res<VelocityBufferHandles>,
    mut reader: EventReader<SpritePreloadEvent>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    mut chain_graph_material_handle: ResMut<ChainGraphMaterialHandle>,
) {
    for event in reader.read() {
        match event {
            SpritePreloadEvent::Grass(_) => {
            }
            SpritePreloadEvent::ChainGraph(sprite_handle) => {
                let material = ChainGraphMaterial {
                    chain: 0.,
                    sprite_handle: sprite_handle.clone()
                };
                chain_graph_material.add(material);
                chain_graph_material_handle.handle = sprite_handle.clone();
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
                    InteractionTypes::ChainReactionDisplay,
                ));
            }
        }
    }
}