use std::collections::HashSet;

use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};
use bevy_tailwind::tw;






pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GlobalAppState>()
            .add_sub_state::<AppLoadingAssetsSubState>()
            .add_loading_state(
                LoadingState::new(AppLoadingAssetsSubState::Loading)
                .continue_to_state(AppLoadingAssetsSubState::Done)
            )
            .insert_resource(PreGameTasks::default())
            .add_systems(PreStartup, spawn_loading_screen)
            .add_systems(PostUpdate, try_translate.run_if(in_state(GlobalAppState::AssetLoading)))
        ;
    }
}

#[derive(Resource, Debug)]
pub struct PreGameTasks {
    tasks: HashSet<String>
}

impl PreGameTasks {
    pub fn add(&mut self, task: String) {
        self.tasks.insert(task);
    }
    pub fn done(&mut self, task: String) {
        self.tasks.remove(&task);
    }
    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Default for PreGameTasks {
    fn default() -> Self {
        Self {
            tasks: HashSet::new()
        }
    }
}

#[allow(non_upper_case_globals)]
pub const OnGame : OnEnter<crate::core::states::GlobalAppState> = OnEnter(crate::core::states::GlobalAppState::InGame);


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameUpdate;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GamePreUpdate;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GamePostUpdate;

use itertools::Itertools;

pub fn try_translate(
    mut next_state: ResMut<NextState<GlobalAppState>>,
    loading_state: Res<State<AppLoadingAssetsSubState>>,
    tasks: Res<PreGameTasks>,
    s: Option<Single<Entity, With<LoadingScreenText>>>,
    ls: Option<Single<Entity, With<LoadingScreen>>>,
    mut cmd: Commands,
){
    if *loading_state == AppLoadingAssetsSubState::Done &&
    tasks.is_empty() {
        next_state.set(GlobalAppState::InGame);
        if let Some(ls) = ls {
            cmd.entity(*ls).despawn();
        }
    } else {
        if let Some(s) = s {
            let t = tasks.tasks.iter().join("\n");
            cmd.entity(*s).insert(
                Text::new(format!("Loading:\nAssets: {:?}\nTasks:\n{}", loading_state, t)),
            );
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GlobalAppState {
    #[default]
    AssetLoading,
    InGame,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GlobalAppState = GlobalAppState::AssetLoading)]
pub enum AppLoadingAssetsSubState {
    #[default]
    Loading,
    Done,
}


#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingScreenText;

pub fn spawn_loading_screen(
    mut cmd: Commands,
    assets: Res<AssetServer>
){
    info!("Loading screen added!");
    cmd.spawn((
        tw!("absolute w-full h-full p-[10px] z-10"),
        LoadingScreen,
        LoadingScreenText,
        Text::new("Loading..."),
        TextFont {
            font: assets.load("fonts/orp_regular.ttf"),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb_u8(39, 223, 141)),
    ));
}