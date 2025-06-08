use bevy::{asset::LoadState, audio::{PlaybackMode, Volume}, color::palettes::css::GREEN, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::{ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Friction, Group, Sensor};
use tiled::{ObjectShape, PropertyValue};

use crate::{core::states::{GlobalAppState, OnGame, PreGameTasks}, interactions::components::{InInteraction, Interactable, InteractableMaterial, InteractionTypes}, physics::constants::{INTERACTABLE_CG, LADDERS_CG, PLATFORMS_CG, PLAYER_CG, PLAYER_SENSOR_CG, STRUCTURES_CG}, tilemap::light::LightEmitter, utils::{custom_material_loader::SpriteAssets, debree::{Malfunction, MalfunctionType}, spacial_audio::{AlarmSpeaker, SoundAssets}}};


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let path = None;

        // #[cfg(not(target_arch = "wasm32"))]
        // let mut d = std::env::current_dir().unwrap();
        // #[cfg(not(target_arch = "wasm32"))]
        // d.push("assets_raw/objects.json");
        // #[cfg(not(target_arch = "wasm32"))]
        // let path = Some(d);
        #[cfg(target_arch = "wasm32")]
        let path = None;

        app
            .add_plugins((
                TilemapPlugin,
                TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: path}),
                TiledPhysicsPlugin::<CustomRapierPhysicsBackend>::default(),
            ))
            .insert_resource(Aboba::default())
            .register_type::<MapObject>()
            .register_type::<LightEmitter>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, ((handle_layer_spawn, hihihaha), handle_object_spawn.run_if(in_state(GlobalAppState::InGame))))
            // .add_observer(handle_layer_spawn)
            .add_systems(Update, (
                check_map, event_map_created, 
            ).run_if(in_state(GlobalAppState::AssetLoading)))
        ;
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
enum MapObject {
    #[default]
    Ladder,
    Interactable,
    PlayerSpawn,
    Light,
    Platform
}

#[derive(Resource)]
pub struct MapAssets {
    map: Handle<TiledMap>
}

pub fn spawn_map(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut tasks: ResMut<PreGameTasks>,
){
    let map = assets.load("tilemaps/v2.0/main.tmx");
    cmd.insert_resource(MapAssets{map: map.clone()});
    cmd.spawn((
        TiledMapHandle(map),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<CustomRapierPhysicsBackend> {
                    objects_layer_filter: TiledName::All,
                    ..default()
        }
    ));
    tasks.add("map_loading".to_string());
    tasks.add("map_spawn".to_string());
}

#[derive(Component)]
pub struct LadderCollider;

#[derive(Component)]
pub struct SpacewalkCollider;

#[derive(Resource, Default)]
pub struct Aboba {
    pub data: Vec<TiledObjectCreated>,
}

fn hihihaha(
    mut e: EventReader<TiledObjectCreated>,
    mut aboba: ResMut<Aboba>,
) {
    for e in e.read() {
        aboba.data.push(e.clone());
    }
}

fn handle_object_spawn(
    mut cmd: Commands,
    q_c: Query<
        (Entity, &Children)
    >,
    map_asset: Res<Assets<TiledMap>>,
    image_assets: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut interactable_materials: ResMut<Assets<InteractableMaterial>>,
    sprite_assets: Res<SpriteAssets>,
    // FUCKING FUCK YARO
    mut aboba: ResMut<Aboba>,
    sound_assets: Res<SoundAssets>,
) {
    for e in aboba.data.iter() {
        let Some(object) = e.get_object(&map_asset) else {continue;};

        if let Some(l) = LightEmitter::from_properties(&object.properties) {
            cmd.entity(e.entity).with_child((l, GlobalTransform::IDENTITY, Transform::default()));
        }
        if let Some(PropertyValue::BoolValue(true)) = object.properties.get("spacewalk") {
            let Ok((_e, object_children_with_collider)) = q_c.get(e.entity) else {return;};
            for c in object_children_with_collider {
                cmd.entity(*c).insert((Sensor,SpacewalkCollider));
            }
        }
        if let Some(PropertyValue::BoolValue(true)) = object.properties.get("speaker") {
            cmd.entity(e.entity).with_child(
                (
                AlarmSpeaker,
                Name::new("AlarmSpeaker"),
                Transform::default(),
                )
            );
        }
        if let Some(PropertyValue::StringValue(t)) = object.properties.get("ambience") {
            let handle;
            let name;
            match t.as_str() {
                "EngineAmbience" => {
                    handle = sound_assets.engine_ambience.clone();
                    name = t.clone();
                }
                "IndustrialAmbience" => {
                    handle = sound_assets.industrial_ambience.clone();
                    name = t.clone();
                }
                _ => {unreachable!()}
            }
            cmd.entity(e.entity).with_child(
                (
                    AudioPlayer::new(handle),
                    PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::Linear(1.),
                        speed: 1.0,
                        paused: false,
                        muted: false,
                        spatial: true,
                        spatial_scale: None,
                    },
                    Name::new(name),
                    Transform::default(),
                )
            );
        }
        if let Some(interaction) = InteractionTypes::from_properties(&object.properties){
            let Ok((_e, object_children_with_collider)) = q_c.get(e.entity) else {return;};
            for c in object_children_with_collider {
                let handle;
                match interaction {
                    InteractionTypes::ChainReactionDisplay => {
                        handle = sprite_assets.chain_interactable.clone();
                    },
                    InteractionTypes::WaveModulator => {
                        handle = sprite_assets.wave_interactable.clone();
                    },
                    InteractionTypes::PipePuzzle => {
                        handle = sprite_assets.pipe_interactable.clone();
                    },
                    InteractionTypes::CollisionMinigame => {
                        handle = sprite_assets.collision_interactable.clone();
                    },
                    InteractionTypes::WarningInterface => {
                        handle = sprite_assets.warning_interactable.clone();
                    },
                    InteractionTypes::HackMinigame => {
                        handle = sprite_assets.faz.clone();
                    },
                    InteractionTypes::WiresMinigame => {
                        handle = sprite_assets.faz.clone();
                    },
                }
                let image = image_assets.get(&handle).unwrap();
                let width = image.width();
                let height = image.height();
                let material = InteractableMaterial {
                    time: 0.,
                    sprite_handle: handle.clone(),
                    _webgl2_padding_8b: 0,
                    _webgl2_padding_12b: 0,
                    _webgl2_padding_16b: 0,
                };
                let interactable_material_handle = interactable_materials.add(material);
                cmd.entity(*c).insert((
                    Mesh2d(meshes.add(Rectangle::new(width as f32, height as f32))),
                    MeshMaterial2d(interactable_material_handle.clone()),
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
                    interaction.clone(),
                ));
                break
            }
            
        }
        // if let Some(m) = MalfunctionType::from_properties(&object.properties) {
        //     info!("MALF: {:?}", object.properties);
        //     let Ok((_e, object_children_with_collider)) = q_c.get(e.entity) else {return;};
        //     info!("MALF CHILD");
        //     /*
        //     EXISTS:
        //         WARNING
        //         ANTENNA
        //         ENGINE
        //         REACTOR
        //         HACK
        //         WAVE
        //     */
        //     info!("INSERTING SENSOR");
        //     for c in object_children_with_collider {
        //         cmd.entity(*c).insert((
        //             // LIKE THAT:
        //             Name::new("INSERTED"),
        //             Sensor,
        //             // LadderCollider,
        //             // ActiveEvents::COLLISION_EVENTS,
        //             // CollisionGroups{
        //             //     memberships: Group::from_bits(LADDERS_CG).unwrap(),
        //             //     filters: Group::from_bits(PLAYER_CG).unwrap(),
        //             // }
        //         ));
        //     }
        // }
    }
    aboba.data  = vec![];
}



fn handle_layer_spawn(
    mut cmd: Commands,
    q_c: Query<
        (Entity, &Children)
    >,
    mut e: EventReader<TiledLayerCreated>,
    map_asset: Res<Assets<TiledMap>>,
) {
    for e in e.read(){
        let Some(layer) = e.get_layer(&map_asset) else {continue};
        let Ok((_e, c)) = q_c.get(e.entity) else {continue;};
        // LEGACY CODE :p
        match layer.name.as_str() {
            "LADDERS" => {
                for c in c.iter() {
                    let Ok((_e, c)) = q_c.get(c) else {continue;};
                    for c in c.iter() {
                        cmd.entity(c).insert((
                            Sensor,
                            LadderCollider,
                            ActiveEvents::COLLISION_EVENTS,
                            CollisionGroups{
                                memberships: Group::from_bits(LADDERS_CG).unwrap(),
                                filters: Group::from_bits(PLAYER_CG).unwrap(),
                            }
                        ));
                    }
                }
            }
            "PLATFORMS" => {
                for c in c.iter() {
                    let Ok((_e, c)) = q_c.get(c) else {continue;};
                    for c in c.iter() {
                        cmd.entity(c).insert(
                            CollisionGroups{
                                memberships: Group::from_bits(PLATFORMS_CG).unwrap(),
                                filters: Group::from_bits(PLAYER_CG).unwrap(),
                            }
                        );
                    }
                }
            }
            _ => {}
        }

    }
}


pub fn check_map(
    asset_server: Res<AssetServer>,
    mut tasks: ResMut<PreGameTasks>,
    assets: Option<Res<MapAssets>>,
    mut cmd: Commands
){


    let Some(assets) = assets else {return;};
    let p = asset_server.get_load_state(&assets.map);
    if let Some(s) = p {
        match s {
            LoadState::Loaded => {}
            LoadState::Failed(e) => {error!("Error loading asset: {:?}, ignoring", e);}
            _ => {return;}
        }
    }
    tasks.done("map_loading".to_string());
    cmd.remove_resource::<MapAssets>();
}

fn event_map_created(
    mut map_events: EventReader<TiledMapCreated>,
    mut tasks: ResMut<PreGameTasks>,
) {
    for _e in map_events.read() {
        tasks.done("map_spawn".to_string());
    }
}


#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
struct CustomRapierPhysicsBackend(TiledPhysicsRapierBackend);

impl TiledPhysicsBackend for CustomRapierPhysicsBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        tiled_map: &TiledMap,
        filter: &TiledNameFilter,
        collider: &TiledCollider,
        anchor: &TilemapAnchor,
    ) -> Vec<TiledColliderSpawnInfos> {
        let colliders = self
            .0
            .spawn_colliders(commands, tiled_map, filter, collider, anchor);
        for c in colliders.iter() {
            commands.entity(c.entity).insert(
                CollisionGroups{
                    memberships: Group::from_bits(PLATFORMS_CG | STRUCTURES_CG).unwrap(),
                    filters: Group::from_bits(PLAYER_CG).unwrap(),
                }
            );
        }
        colliders
    }
}