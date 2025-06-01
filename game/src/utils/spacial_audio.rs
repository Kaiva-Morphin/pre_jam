use bevy::prelude::*;

const AUDIO_SCALE: f32 = 1. / 100.0;

// pub struct SpacialAudioPlugin;

// impl Plugin for SpacialAudioPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         .add_plugins((
//             Material2dPlugin::<VelocityBufferMaterial>::default(),
//             Material2dPlugin::<GrassMaterial>::default(),
//             Material2dPlugin::<InteractableMaterial>::default(),
//         ))
//         .insert_resource(VelocityBufferHandles::default())
//         .insert_resource(TextureAtlasHandes::default())
//         .add_event::<SpritePreloadEvent>()
//         .add_systems(Startup, (preload_sprites, create_atlas))
//         .add_systems(Update, spawn_sprites);
//     }
// }

