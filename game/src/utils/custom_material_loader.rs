use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use shaders::components::*;

use crate::interactions::{chain_reaction_display::ChainGraphMaterial, components::{InInteraction, Interactable, InteractableMaterial, InteractablesImageHandle, InteractionTypes, INTERACTABLE_CG, PLAYER_SENSOR_CG}, wave_modulator::WaveGraphMaterial};

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
    #[asset(path = "interactables/ChainGraph.png")]
    pub chain_graph_sprite: Handle<Image>,
    #[asset(path = "interactables/chain.png")]
    pub chain_interactable: Handle<Image>,
    #[asset(path = "interactables/wave.png")]
    pub wave_interactable: Handle<Image>,
}

pub struct SpritePreloadPlugin;

impl Plugin for SpritePreloadPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<LoadingStates>()
        .add_loading_state(
            LoadingState::new(LoadingStates::AssetLoading)
            .load_collection::<SpriteAssets>()
            .continue_to_state(LoadingStates::Next)
        )
        .add_plugins((
            // Material2dPlugin::<VelocityBufferMaterial>::default(),
            // Material2dPlugin::<GrassMaterial>::default(),
            Material2dPlugin::<InteractableMaterial>::default(),
            Material2dPlugin::<ChainGraphMaterial>::default(),
            Material2dPlugin::<WaveGraphMaterial>::default(),
        ))
        .insert_resource(VelocityBufferHandles::default())
        .insert_resource(TextureAtlasHandes::default())
        .add_event::<SpritePreloadEvent>()
        .add_systems(OnEnter(LoadingStates::Next), (preload_sprites, create_atlas))
        .add_systems(Update, (spawn_sprites).run_if(in_state(LoadingStates::Next)));
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
    pub interaction_type: InteractionTypes,
}

#[derive(Event)]
pub enum SpritePreloadEvent {
    Grass(SpritePreloadData),
    Interactable(SpritePreloadData),
}

pub fn preload_sprites(
    asset_server: ResMut<AssetServer>,
    mut writer: EventWriter<SpritePreloadEvent>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandes>,
    sprite_assets: Res<SpriteAssets>,
) {
    println!("preload sprites");
    texture_atlas_handles.image_handle = sprite_assets.key_e_sprite.clone();
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.chain_interactable.clone(),
        pos: Vec2::new(-40., 10.),
        interaction_type: InteractionTypes::ChainReactionDisplay,
    }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.wave_interactable.clone(),
        pos: Vec2::new(40., 10.),
        interaction_type: InteractionTypes::WaveModulator,
    }));
}

pub fn spawn_sprites(
    mut commands: Commands,
    image_assets: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut grass_materials: ResMut<Assets<GrassMaterial>>,
    mut interactable_materials: ResMut<Assets<InteractableMaterial>>,
    buffer_handles: Res<VelocityBufferHandles>,
    mut reader: EventReader<SpritePreloadEvent>,
    mut chain_graph_material: ResMut<Assets<ChainGraphMaterial>>,
    mut interactables_material_handle: ResMut<InteractablesImageHandle>,
    
) {
    for event in reader.read() {
        match event {
            SpritePreloadEvent::Grass(_) => {
            }
            SpritePreloadEvent::Interactable(sprite_data) => {
                let image = image_assets.get(&sprite_data.handle).unwrap();
                let width = image.width();
                let height = image.height();
                let material = InteractableMaterial {
                    time: 0.,
                    sprite_handle: sprite_data.handle.clone(),
                    _webgl2_padding_8b: 0,
                    _webgl2_padding_12b: 0,
                    _webgl2_padding_16b: 0,
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
                    sprite_data.interaction_type.clone(),
                ));
            }
        }
    }
}