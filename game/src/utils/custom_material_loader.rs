use bevy::{audio::{PlaybackMode, Volume}, prelude::*, sprite::Material2dPlugin};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::{ConfigureLoadingState, LoadingStateConfig}, LoadingState, LoadingStateAppExt}};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, CollisionGroups, Group, Sensor};
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};
use shaders::components::*;

use crate::{core::states::{AppLoadingAssetsSubState, GameUpdate, GlobalAppState, OnGame}, interactions::{chain_reaction_display::ChainGraphMaterial, collision_minigame::CollisionGraphMaterial, components::{InInteraction, Interactable, InteractableMaterial, InteractionTypes}, hack_minigame::{HACK_ATLAS_COLUMNS, HACK_ATLAS_ROWS, HACK_PIXEL_GRID_SIZE}, pipe_puzzle::SINGLE_PIPE_TEX_SIZE, warning_interface::{WARNING_GRID_COLUMNS, WARNING_GRID_ROWS, WARNING_GRID_SIZE}, wave_modulator::{WaveGraphMaterial, NUM_SPINNY_STATES, SPINNY_SIZE}}, physics::{animator::PlayerAnimations, constants::*, player::Player}, tilemap::light::LightEmitter, utils::{mouse::CursorPosition, spacial_audio::SoundAssets}};





#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "atlases/E.png")]
    pub key_f_atlas: Handle<Image>,
    #[asset(path = "ui/pipes.png")]
    pub pipes_atlas: Handle<Image>,
    #[asset(path = "ui/spinner.png")]
    pub spinny_atlas: Handle<Image>,
    #[asset(path = "atlases/Warning.png")]
    pub warning_atlas: Handle<Image>,
    #[asset(path = "atlases/Warning.png")]
    pub hack_atlas: Handle<Image>,
    #[asset(path = "ui/malf.png")]
    pub malf_atlas: Handle<Image>,

    #[asset(path = "interactables/chaingraph.png")]
    pub chain_graph_sprite: Handle<Image>,
    #[asset(path = "interactables/wavegraph.png")]
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
    #[asset(path = "interactables/wires.png")]
    pub wires: Handle<Image>,
    #[asset(path = "ui/wire.png")]
    pub wire: Handle<Image>,

    #[asset(path = "interactables/hack.png")]
    pub hack: Handle<Image>,

    #[asset(path = "interactables/ururur.png")]
    pub faz: Handle<Image>,

    #[asset(path = "ui/send.png")]
    pub send: Handle<Image>,



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
        .insert_resource(MalfAtlasHandles::default())
        .add_systems(OnGame, (create_atlases, spawn_faz))
        .add_systems(Update, (click_faz).run_if(in_state(GlobalAppState::InGame)));
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
pub struct MalfAtlasHandles {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
}

pub const KEYS_ATLAS_COLUMNS: u32 = 3;
pub const KEYS_ATLAS_ROWS: u32 = 1;
pub const KEYS_ATLAS_SIZE: u32 = KEYS_ATLAS_COLUMNS * KEYS_ATLAS_ROWS;

pub const MALF_ATLAS_COLUMNS: u32 = 6;
pub const MALF_ATLAS_ROWS: u32 = 2;

pub fn create_atlases(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
    mut spinny_atlas_handles: ResMut<SpinnyAtlasHandles>,
    mut pipes_atlas_handles: ResMut<PipesAtlasHandles>,
    mut warning_atlas_handles: ResMut<WarningAtlasHandles>,
    mut malf_atlas_handles: ResMut<MalfAtlasHandles>,
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
        UVec2::splat(SINGLE_PIPE_TEX_SIZE as u32),
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
    // hack_atlas_handles.image_handle = sprite_assets.hack_atlas.clone();
    // let hack_atlas = TextureAtlasLayout::from_grid(
    //     UVec2::splat(HACK_PIXEL_GRID_SIZE),
    //     HACK_ATLAS_COLUMNS,
    //     HACK_ATLAS_ROWS,
    //     None,
    //     None
    // );
    // let hack_handle = texture_atlases.add(hack_atlas);
    // hack_atlas_handles.layout_handle = hack_handle;
    // malf
    malf_atlas_handles.image_handle = sprite_assets.malf_atlas.clone();
    let malf_handle = TextureAtlasLayout::from_grid(
        UVec2::splat(48),
        MALF_ATLAS_COLUMNS,
        MALF_ATLAS_ROWS,
        None,
        None
    );
    let malf_handle: Handle<TextureAtlasLayout> = texture_atlases.add(malf_handle);
    malf_atlas_handles.layout_handle = malf_handle;
}

#[derive(Component)]
pub struct Faz;

fn spawn_faz(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
) {
    commands.spawn((
        Sprite::from_image(sprite_assets.faz.clone()),
        Transform::from_translation(Vec3::new(300., 100., 0.,)),
        Faz,
        children![(
            FazLight,
            GlobalTransform::default(),
            Transform::from_translation(vec3(0.0, 50.0, 0.0)),
            LightEmitter{
                radius_px: 100.0,
                spot: 90.0,
                color_and_rotation: vec4(1.0, 1.0, 1.0, 90.0),
                intensity: 2.0,
            }
        )]
    ));
}

#[derive(Component)]
pub struct FazLight;



pub fn click_faz(
    windows: Single<&Window>,
    v: Res<pixel_utils::camera::PixelCameraVars>,
    cq: Single<&GlobalTransform, With<PixelCamera>>,
    mut commands: Commands,
    faz: Single<(&Transform, Entity), With<Faz>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    sound_assets: Res<SoundAssets>,
    mut p: Query<&mut Player>,
    mut r: ResMut<PlayerAnimations>,
    mut faz_light : Query<&mut LightEmitter, With<FazLight>>,
    t: Res<Time>,
){
    for p in p.iter() {if p.is_dancing(){
        let h = (t.elapsed_secs() * 360.) % 360.0;
        let c = Srgba::from(Color::hsl(h, 1.0, 0.5));
        for mut l in faz_light.iter_mut() {
            l.color_and_rotation = vec4(c.red, c.green, c.blue, 90.0);
        }
    } else {
        for mut l in faz_light.iter_mut() {
            l.color_and_rotation = vec4(1.0, 1.0, 1.0, 90.0);
        }
    }}

    let camera_transform = *cq;
    let window = *windows;
    let window_size = window.size();
    let target_size = Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32);
    if let Some(screen_position) = window.cursor_position() {
        let s = screen_position;
        
        let up = (s - window_size * 0.5) / (target_size * v.scale());
        let mut pos = camera_transform.translation().truncate();
        pos += up * target_size * vec2(1.0, -1.0);
        let hs = Vec2::new(30., 40.) / 2.;
        let mi = faz.0.translation.xy() - hs;
        let ma = faz.0.translation.xy() + hs;
        if pos.x >= mi.x && pos.x <= ma.x &&
        pos.y >= mi.y && pos.y <= ma.y
        && mouse_button.just_released(MouseButton::Left) {
            commands.entity(faz.1).insert((
                AudioPlayer::new(sound_assets.faz_sound.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::Linear(1.),
                    speed: 1.0,
                    paused: false,
                    muted: false,
                    spatial: false,
                    spatial_scale: None,
                },
            ));
            for mut p in p.iter_mut() {
                p.try_dance(&mut r, crate::physics::animator::PlayerAnimationNode::random_dance());
            }
        }
    }
}
