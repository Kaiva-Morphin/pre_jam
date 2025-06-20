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
        .add_systems(OnGame, play_main_theme)
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
    #[asset(path = "sounds/main.mp3")]
    pub main_theme: Handle<AudioSource>,
    #[asset(path = "sounds/Retro Beeep 06.wav")]
    pub beep_sound: Handle<AudioSource>,
    #[asset(path = "sounds/Retro Ambience 02.wav")]
    pub engine_ambience: Handle<AudioSource>,
    #[asset(path = "sounds/173273__tomlija__janitors-bedroom-ambience.wav")]
    pub industrial_ambience: Handle<AudioSource>,
    #[asset(path = "sounds/vent_ambience.wav")]
    pub vent_ambience: Handle<AudioSource>,
    #[asset(path = "sounds/Retro PickUp Coin StereoUP 04.wav")]
    pub success: Handle<AudioSource>,
    #[asset(path = "sounds/Retro Negative Short 23.wav")]
    pub fail: Handle<AudioSource>,
    #[asset(path = "sounds/144912__thesoundcatcher__metal_creek_-crash_smash.wav")]
    pub boom: Handle<AudioSource>,
}

#[derive(Component)]
pub struct AlarmSpeaker;

fn play_alarm_speakers(
    mut commands: Commands,
    speakers: Query<Entity, With<AlarmSpeaker>>,
    mut malfunction: ResMut<Malfunction>,
    sound_assets: Res<SoundAssets>,
) {
    // TODO: IF FLYING => DISABLE
    if malfunction.is_changed() && malfunction.in_progress && malfunction.added_new_malfunction {
        malfunction.added_new_malfunction = false;
        for speaker_entity in speakers {
            commands.entity(speaker_entity).insert((
                AudioPlayer::new(sound_assets.alarm_sound.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    volume: Volume::Linear(0.2),
                    speed: 1.0,
                    paused: false,
                    muted: false,
                    spatial: true,
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
    Beep,
    Success,
    Fail,
    Boom,
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
            commands.spawn(sound_bundle(sound_assets.hack_press_sound.clone(), 1.));
        },
        PlaySoundEvent::HackButtonRelease => {
            commands.spawn(sound_bundle(sound_assets.hack_release_sound.clone(), 1.));
        },
        PlaySoundEvent::SubmitButtonPress => {
            commands.spawn(sound_bundle(sound_assets.submit_press_sound.clone(), 1.));
        },
        PlaySoundEvent::SubmitButtonRelease => {
            commands.spawn(sound_bundle(sound_assets.submit_release_sound.clone(), 1.));
        },
        PlaySoundEvent::SpinnyClick => {
            commands.spawn(sound_bundle(sound_assets.spinny_click_sound.clone(), 1.));
        },
        PlaySoundEvent::OpenUi => {
            commands.spawn(sound_bundle(sound_assets.open_ui_sound.clone(), 1.));
        },
        PlaySoundEvent::Concrete1 => {
            commands.spawn(sound_bundle(sound_assets.concrete_1_sound.clone(), 1.));
        }, // delta 440ms
        PlaySoundEvent::Concrete2 => {
            commands.spawn(sound_bundle(sound_assets.concrete_2_sound.clone(), 1.));
        },
        PlaySoundEvent::OpenWires => {
            commands.spawn(sound_bundle(sound_assets.open_wires_sound.clone(), 1.));
        },
        PlaySoundEvent::WireClick => {
            commands.spawn(sound_bundle(sound_assets.wires_sound.clone(), 1.));
        },
        PlaySoundEvent::Beep => {
            commands.spawn(sound_bundle(sound_assets.beep_sound.clone(), 0.2));
        },
        PlaySoundEvent::Success => {
            commands.spawn(sound_bundle(sound_assets.success.clone(), 0.8));
        },
        PlaySoundEvent::Fail => {
            commands.spawn(sound_bundle(sound_assets.fail.clone(), 0.8));
        },
        PlaySoundEvent::Boom => {
            commands.spawn(sound_bundle(sound_assets.boom.clone(), 1.));
        },
    }
}

fn sound_bundle(
    handle: Handle<AudioSource>,
    volume: f32,
) -> impl Bundle {
    (
        AudioPlayer::new(handle),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(volume),
            speed: 1.0,
            paused: false,
            muted: false,
            spatial: false,
            spatial_scale: None,
        },
    )
}

fn play_main_theme(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
) {
    commands.spawn((
        AudioPlayer::new(sound_assets.main_theme.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(1.),
            speed: 1.0,
            paused: false,
            muted: false,
            spatial: false,
            spatial_scale: None,
        },
    ));
}