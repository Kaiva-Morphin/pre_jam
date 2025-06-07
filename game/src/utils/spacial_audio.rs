use bevy::{audio::{PlaybackMode, Volume}, color::palettes::css::GREEN, prelude::*};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::{ConfigureLoadingState, LoadingStateConfig}, LoadingStateAppExt}};

use crate::{core::states::{AppLoadingAssetsSubState, GlobalAppState, OnGame}, utils::debree::Malfunction};

pub struct SpacialAudioPlugin;

impl Plugin for SpacialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_loading_state(
            LoadingStateConfig::new(AppLoadingAssetsSubState::Loading)
                .load_collection::<SoundAssets>(),
        )
        .add_systems(OnGame, spawn_alarm_speakers)
        .add_systems(Update, play_alarm_speakers.run_if(in_state(GlobalAppState::InGame)))
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct SoundAssets {
    #[asset(path = "sounds/192343__zimbot__nastyalarmloop.wav")]
    pub alarm_sound: Handle<AudioSource>,
    #[asset(path = "sounds/mem.mp3")]
    pub faz_sound: Handle<AudioSource>,
}

#[derive(Component)]
pub struct AlarmSpeaker;

fn spawn_alarm_speakers(
    mut commands: Commands,
) {
    commands.spawn(
        (
        Transform::from_xyz(200., 200., 0.),
        Sprite::from_color(Color::Srgba(GREEN), Vec2::splat(20.0)),
        AlarmSpeaker,
        Name::new("AlarmSpeaker"),
        )
    );
}

fn play_alarm_speakers(
    mut commands: Commands,
    speakers: Query<Entity, With<AlarmSpeaker>>,
    malfunction: Res<Malfunction>,
    sound_assets: Res<SoundAssets>,
) {
    if malfunction.is_changed() && malfunction.in_progress && malfunction.added_new_malfunction {
        println!("play");
        for speaker_entity in speakers {
            commands.entity(speaker_entity).insert((
                AudioPlayer::new(sound_assets.alarm_sound.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::Linear(0.2),
                    speed: 1.0,
                    paused: false,
                    muted: false,
                    spatial: false,
                    spatial_scale: None,
                },
            ));
        }
    }
}