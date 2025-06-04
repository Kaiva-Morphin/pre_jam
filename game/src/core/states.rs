use std::collections::HashSet;

use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt}};






pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, GameUpdate.run_if(in_state(GlobalAppState::InGame)))
            .configure_sets(PreUpdate, GamePreUpdate.run_if(in_state(GlobalAppState::InGame)))
            .configure_sets(PostUpdate, GamePostUpdate.run_if(in_state(GlobalAppState::InGame)))
            .init_state::<GlobalAppState>()
            .add_sub_state::<AppLoadingAssetsSubState>()
            .add_loading_state(
                LoadingState::new(AppLoadingAssetsSubState::Loading)
                .continue_to_state(AppLoadingAssetsSubState::Done)
            )
            .insert_resource(PreGameTasks::default())
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


pub fn try_translate(
    mut next_state: ResMut<NextState<GlobalAppState>>,
    loading_state: Res<State<AppLoadingAssetsSubState>>,
    tasks: Res<PreGameTasks>
){
    if *loading_state == AppLoadingAssetsSubState::Done &&
    tasks.is_empty() {
        next_state.set(GlobalAppState::InGame);
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
