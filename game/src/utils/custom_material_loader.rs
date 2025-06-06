use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::{ConfigureLoadingState, LoadingStateConfig}, LoadingState, LoadingStateAppExt}};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use shaders::components::*;

use crate::{core::states::{AppLoadingAssetsSubState, GameUpdate, GlobalAppState, OnGame}, interactions::{chain_reaction_display::ChainGraphMaterial, collision_minigame::CollisionGraphMaterial, components::{InInteraction, Interactable, InteractableMaterial, InteractionTypes}, hack_minigame::{HACK_ATLAS_COLUMNS, HACK_ATLAS_ROWS, HACK_PIXEL_GRID_SIZE}, pipe_puzzle::PIPE_GRID_SIZE, warning_interface::{WARNING_GRID_COLUMNS, WARNING_GRID_ROWS, WARNING_GRID_SIZE}, wave_modulator::{WaveGraphMaterial, NUM_SPINNY_STATES, SPINNY_SIZE}}, physics::constants::*};





#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "pixel/grass.png")]
    pub grass_sprite: Handle<Image>,

    #[asset(path = "atlases/E.png")]
    pub key_f_atlas: Handle<Image>,
    #[asset(path = "atlases/Pipes.png")]
    pub pipes_atlas: Handle<Image>,
    #[asset(path = "atlases/spinny.png")]
    pub spinny_atlas: Handle<Image>,
    #[asset(path = "atlases/Warning.png")]
    pub warning_atlas: Handle<Image>,
    #[asset(path = "atlases/Warning.png")]
    pub hack_atlas: Handle<Image>,

    #[asset(path = "interactables/ChainGraph.png")]
    pub chain_graph_sprite: Handle<Image>,
    #[asset(path = "interactables/WaveGraph.png")]
    pub wave_graph_sprite: Handle<Image>,

    #[asset(path = "interactables/chain.png")]
    pub chain_interactable: Handle<Image>,
    #[asset(path = "interactables/wave.png")]
    pub wave_interactable: Handle<Image>,
    #[asset(path = "interactables/pipe.png")]
    pub pipe_interactable: Handle<Image>,
    #[asset(path = "interactables/collision.png")]
    pub collision_interactable: Handle<Image>,
    #[asset(path = "interactables/warning.png")]
    pub warning_interactable: Handle<Image>,

    #[asset(path = "interactables/reactor_mini.png")]
    pub reactor_mini: Handle<Image>,
}


pub struct SpritePreloadPlugin;

impl Plugin for SpritePreloadPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_loading_state(
            LoadingStateConfig::new(AppLoadingAssetsSubState::Loading)
                .load_collection::<SpriteAssets>(),
        )
        .add_plugins((
            // Material2dPlugin::<VelocityBufferMaterial>::default(),
            // Material2dPlugin::<GrassMaterial>::default(),
            Material2dPlugin::<InteractableMaterial>::default(),
            UiMaterialPlugin::<ChainGraphMaterial>::default(),
            UiMaterialPlugin::<WaveGraphMaterial>::default(),
            UiMaterialPlugin::<CollisionGraphMaterial>::default(),
        ))
        .insert_resource(VelocityBufferHandles::default())
        .insert_resource(TextureAtlasHandles::default())
        .insert_resource(SpinnyAtlasHandles::default())
        .insert_resource(PipesAtlasHandles::default())
        .insert_resource(WarningAtlasHandles::default())
        .insert_resource(HackAtlasHandles::default())
        .add_event::<SpritePreloadEvent>()
        .add_systems(OnGame, (preload_sprites, create_atlases))
        .add_systems(Update, spawn_sprites.run_if(in_state(GlobalAppState::InGame)));
    }
}



#[derive(Resource, Default)]
pub struct TextureAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SpinnyAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct PipesAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct WarningAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct HackAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

pub const KEYS_ATLAS_COLUMNS: u32 = 3;
pub const KEYS_ATLAS_ROWS: u32 = 1;
pub const KEYS_ATLAS_SIZE: u32 = KEYS_ATLAS_COLUMNS * KEYS_ATLAS_ROWS;

pub fn create_atlases(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
    mut spinny_atlas_handles: ResMut<SpinnyAtlasHandles>,
    mut pipes_atlas_handles: ResMut<PipesAtlasHandles>,
    mut warning_atlas_handles: ResMut<WarningAtlasHandles>,
    sprite_assets: Res<SpriteAssets>,
) {
    // f key
    texture_atlas_handles.image_handle = sprite_assets.key_f_atlas.clone();
    let atlas = TextureAtlasLayout::from_grid(
        UVec2::new(19, 21),
        KEYS_ATLAS_COLUMNS,
        KEYS_ATLAS_ROWS,
        None,
        None
    );
    let handle = texture_atlases.add(atlas);
    texture_atlas_handles.layout_handle = handle.clone();
    // spinny
    spinny_atlas_handles.image_handle = sprite_assets.spinny_atlas.clone();
    let spinny_atlas = TextureAtlasLayout::from_grid(
        SPINNY_SIZE,
        NUM_SPINNY_STATES as u32,
        1,
        None,
        None
    );
    let spinny_handle = texture_atlases.add(spinny_atlas);
    spinny_atlas_handles.layout_handle = spinny_handle;
    // pipes
    pipes_atlas_handles.image_handle = sprite_assets.pipes_atlas.clone();
    let pipes_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(PIPE_GRID_SIZE as u32),
        4,
        4,
        None,
        None
    );
    let pipes_handle = texture_atlases.add(pipes_atlas);
    pipes_atlas_handles.layout_handle = pipes_handle;
    // warning
    warning_atlas_handles.image_handle = sprite_assets.warning_atlas.clone();
    let warning_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(WARNING_GRID_SIZE),
        WARNING_GRID_COLUMNS,
        WARNING_GRID_ROWS,
        None,
        None
    );
    let warning_handle = texture_atlases.add(warning_atlas);
    warning_atlas_handles.layout_handle = warning_handle;
    // hack
    let hack_atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(HACK_PIXEL_GRID_SIZE),
        HACK_ATLAS_COLUMNS,
        HACK_ATLAS_ROWS,
        None,
        None
    );
    let hack_handle = texture_atlases.add(hack_atlas);
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
    mut writer: EventWriter<SpritePreloadEvent>,
    sprite_assets: Res<SpriteAssets>,
) {
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.chain_interactable.clone(),
        pos: Vec2::new(150., 10.),
        interaction_type: InteractionTypes::ChainReactionDisplay,
    }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.wave_interactable.clone(),
        pos: Vec2::new(80., 10.),
        interaction_type: InteractionTypes::WaveModulator,
    }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.pipe_interactable.clone(),
        pos: Vec2::new(200., 10.),
        interaction_type: InteractionTypes::PipePuzzle,
    }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.collision_interactable.clone(),
        pos: Vec2::new(40., 10.),
        interaction_type: InteractionTypes::CollisionMinigame,
    }));
    writer.write(SpritePreloadEvent::Interactable(SpritePreloadData {
        handle: sprite_assets.warning_interactable.clone(),
        pos: Vec2::new(-40., 10.),
        interaction_type: InteractionTypes::WarningInterface,
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
                    // ActiveCollisionTypes::KINEMATIC_STATIC | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
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