use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::{ActiveEvents, CoefficientCombineRule, CollisionGroups, Friction, Group, Sensor};

use crate::{core::states::{GlobalAppState, OnGame, PreGameTasks}, physics::constants::{LADDERS_CG, PLAYER_CG}};


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut path = std::env::current_dir().unwrap();
        path.push("assets_raw/objects.json");
        app
            .add_plugins((
                TilemapPlugin,
                TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: Some(path) }),
                TiledPhysicsPlugin::<CustomRapierPhysicsBackend>::default(),
            ))
            .register_type::<MapObject>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, (
                check_map, event_map_created, display_custom_tiles
            ).run_if(in_state(GlobalAppState::AssetLoading)))
        ;
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
enum MapObject {
    #[default]
    Ladder,
    Interactable
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
    let map = assets.load("tilemaps/v1.0/test.tmx");
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

// TiledObjectCreated
#[allow(clippy::type_complexity)]
fn display_custom_tiles(
    mut cmd: Commands,
    q_tile: Query<
        (Entity, &MapObject, &Children),
    >,
    mut e: EventReader<TiledObjectCreated>,
) {
    for e in e.read(){
        let Ok((e, m, c)) = q_tile.get(e.entity) else {continue;};
        
        match m {
            MapObject::Ladder => {
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
            MapObject::Interactable => {
                // commands.entity(e).insert(Interactable);
            }
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