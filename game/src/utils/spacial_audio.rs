use std::time::Duration;

use bevy::{audio::{PlaybackMode, Volume}, color::palettes::css::GREEN, prelude::*};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{config::{ConfigureLoadingState, LoadingStateConfig}, LoadingStateAppExt}};
use utils::WrappedDelta;

use crate::{core::states::{AppLoadingAssetsSubState, GlobalAppState, OnGame}, utils::debree::Malfunction};

pub struct SpacialAudioPlugin;

impl Plugin for SpacialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_loading_state(
            LoadingStateConfig::new(AppLoadingAssetsSubState::Loading)
                .load_collection::<SoundAssets>(),
        )
        .add_event::<PlaySoundEvent>()
        .add_systems(OnGame, spawn_alarm_speakers)
        .add_systems(Update, (play_sounds, play_alarm_speakers).run_if(in_state(GlobalAppState::InGame)))
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct SoundAssets {
    #[asset(path = "sounds/192343__zimbot__nastyalarmloop.wav")]
    pub alarm_sound: Handle<AudioSource>,
    #[asset(path = "sounds/mem.mp3")]
    pub faz_sound: Handle<AudioSource>,
    #[asset(path = "sounds/button_press_1.wav")]
    pub hack_press_sound: Handle<AudioSource>,
    #[asset(path = "sounds/button_press_2.wav")]
    pub hack_release_sound: Handle<AudioSource>,
    #[asset(path = "sounds/submit_button_press.wav")]
    pub submit_press_sound: Handle<AudioSource>,
    #[asset(path = "sounds/submit_button_release.wav")]
    pub submit_release_sound: Handle<AudioSource>,
    #[asset(path = "sounds/spinny_click.wav")]
    pub spinny_click_sound: Handle<AudioSource>,
    #[asset(path = "sounds/open_ui.wav")]
    pub open_ui_sound: Handle<AudioSource>,
    #[asset(path = "sounds/Concrete 1.wav")]
    pub concrete_1_sound: Handle<AudioSource>,
    #[asset(path = "sounds/Concrete 2.wav")]
    pub concrete_2_sound: Handle<AudioSource>,
    #[asset(path = "sounds/118230__joedeshon__hotel_card_key.wav")]
    pub open_wires_sound: Handle<AudioSource>,
    #[asset(path = "sounds/wire.wav")]
    pub wires_sound: Handle<AudioSource>,
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
                // AudioPlayer::new(sound_assets.alarm_sound.clone()),
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

#[derive(Event)]
pub enum PlaySoundEvent {
    HackButtonPress,
    HackButtonRelease,
    SubmitButtonPress,
    SubmitButtonRelease,
    SpinnyClick,
    OpenUi,
    Concrete1,
    Concrete2,
    OpenWires,
    WireClick,
}

pub fn play_sounds(
    mut commands: Commands,
    mut event_reader: EventReader<PlaySoundEvent>,
    sound_assets: Res<SoundAssets>,
    mut offsets: Local<Vec<(Timer, PlaySoundEvent)>>,
    time: Res<Time>,
) {
    // let mut toremove = vec![];
    // for (id, (timer, event)) in offsets.iter_mut().enumerate() {
    //     timer.tick(Duration::from_secs_f32(time.dt()));
    //     if timer.finished() {
    //         match_sounds(&mut commands, &sound_assets, event);
    //         toremove.push(id);
    //     }
    // }
    // for (dumb_offset, id) in toremove.into_iter().enumerate() {
    //     offsets.remove(id - dumb_offset);
    // }
    for event in event_reader.read() {
        match_sounds(&mut commands, &sound_assets, event);
    }
}

fn match_sounds(
    commands: &mut Commands,
    sound_assets: &Res<SoundAssets>,
    event: &PlaySoundEvent,
) {
    match event {
        PlaySoundEvent::HackButtonPress => {
            commands.spawn(sound_bundle(sound_assets.hack_press_sound.clone()));
        },
        PlaySoundEvent::HackButtonRelease => {
            commands.spawn(sound_bundle(sound_assets.hack_release_sound.clone()));
        },
        PlaySoundEvent::SubmitButtonPress => {
            commands.spawn(sound_bundle(sound_assets.submit_press_sound.clone()));
        },
        PlaySoundEvent::SubmitButtonRelease => {
            commands.spawn(sound_bundle(sound_assets.submit_release_sound.clone()));
        },
        PlaySoundEvent::SpinnyClick => {
            commands.spawn(sound_bundle(sound_assets.spinny_click_sound.clone()));
        },
        PlaySoundEvent::OpenUi => {
            commands.spawn(sound_bundle(sound_assets.open_ui_sound.clone()));
        },
        PlaySoundEvent::Concrete1 => {
            commands.spawn(sound_bundle(sound_assets.concrete_1_sound.clone()));
        }, // delta 440ms
        PlaySoundEvent::Concrete2 => {
            commands.spawn(sound_bundle(sound_assets.concrete_2_sound.clone()));
        },
        PlaySoundEvent::OpenWires => {
            commands.spawn(sound_bundle(sound_assets.open_wires_sound.clone()));
        },
        PlaySoundEvent::WireClick => {
            commands.spawn(sound_bundle(sound_assets.wires_sound.clone()));
        },
    }
}

fn sound_bundle(
    handle: Handle<AudioSource>,
) -> impl Bundle {
    (
        AudioPlayer::new(handle),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(1.),
            speed: 1.0,
            paused: false,
            muted: false,
            spatial: false,
            spatial_scale: None,
        },
    )
}