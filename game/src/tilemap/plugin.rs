use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

use crate::core::states::{GlobalAppState, OnGame, PreGameTasks};


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TilemapPlugin,
                TiledMapPlugin(TiledMapPluginConfig { tiled_types_export_file: None }),
                TiledPhysicsPlugin::<TiledPhysicsRapierBackend>::default(),
            ))
            .add_systems(Startup, spawn_map)
            .add_systems(Update, (
                check_map
            ).run_if(in_state(GlobalAppState::AssetLoading)))
        ;
    }
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
    let map = assets.load("tilemaps/v1.0/map.tmx");
    cmd.insert_resource(MapAssets{map: map.clone()});
    cmd.spawn((
        TiledMapHandle(map),
        TilemapAnchor::Center,
        TiledPhysicsSettings::<TiledPhysicsRapierBackend>::default(),
    ));
    tasks.add("map_loading".to_string());
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