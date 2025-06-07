use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::{ActiveEvents, CoefficientCombineRule, CollisionGroups, Friction, Group, Sensor};

use crate::{core::states::{GlobalAppState, OnGame, PreGameTasks}, physics::constants::{LADDERS_CG, PLAYER_CG}};


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        let mut d = std::env::current_dir().unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        d.push("assets_raw/objects.json");
        #[cfg(not(target_arch = "wasm32"))]
        let path = Some(d);
        #[cfg(target_arch = "wasm32")]
        let path = None;

        app
            .add_plugins((
                TilemapPlugin,
                TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: path}),
                TiledPhysicsPlugin::<CustomRapierPhysicsBackend>::default(),
            ))
            .register_type::<MapObject>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, handle_layer_spawn)
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








#[allow(clippy::type_complexity)]
fn handle_layer_spawn(
    mut cmd: Commands,
    q_c: Query<
        (Entity, &Children)
    >,
    // trigger: Trigger<TiledLayerCreated>,
    mut e: EventReader<TiledLayerCreated>,
    map_asset: Res<Assets<TiledMap>>,
) {
    for e in e.read(){
        let Some(layer) = e.get_layer(&map_asset) else {continue};
        info!("Object created: {layer:?}");

        let Ok((e, c)) = q_c.get(e.entity) else {continue;};
        
        match layer.name.as_str() {
            "LADDERS" => {
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
            "PLATFORMS" => {
        //     cmd.entity(c).insert((
        //         Sensor,
        //         LadderCollider,
        //         ActiveEvents::COLLISION_EVENTS,
        //         CollisionGroups{
        //             memberships: Group::from_bits(LADDERS_CG).unwrap(),
        //             filters: Group::from_bits(PLAYER_CG).unwrap(),
        //         }
        //     ));
            }
            _ => {}
        }
        // if layer.name == "ladders" {
        //     for c in layer.objects.iter() {
        //         cmd.entity(c).insert((
        //             Sensor,
        //             LadderCollider,
        //             ActiveEvents::COLLISION_EVENTS,
        //             CollisionGroups{
        //                 memberships: Group::from_bits(LADDERS_CG).unwrap(),
        //                 filters: Group::from_bits(PLAYER_CG).unwrap(),
        //             }
        //         ));
        //     }
        // }
        // for c in c.iter() {
        //     cmd.entity(c).insert((
        //         Sensor,
        //         LadderCollider,
        //         ActiveEvents::COLLISION_EVENTS,
        //         CollisionGroups{
        //             memberships: Group::from_bits(LADDERS_CG).unwrap(),
        //             filters: Group::from_bits(PLAYER_CG).unwrap(),
        //         }
        //     ));
        // }
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
    for e in map_events.read() {
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
        colliders
    }
}