pub mod plugin {
    use bevy::{app::{Plugin, Startup, Update}, ecs::resource::Resource, input::ButtonInput, prelude::{KeyCode, Res, ResMut}};
    use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

    #[derive(Default)]
    pub struct SwitchableRapierDebugPlugin(pub bool);
    impl SwitchableRapierDebugPlugin {
        pub fn enabled() -> Self {
            Self(true)
        }
        pub fn disabled() -> Self {
            Self(false)
        } 
    }
    impl Plugin for SwitchableRapierDebugPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            let mut r = RapierDebugRenderPlugin::default();
            if !self.0 {r = r.disabled();}
            app
                .add_plugins(r)
                .add_systems(Update, update);
        }
    }

    fn update(
        q: Option<ResMut<DebugRenderContext>>,
        k: Res<ButtonInput<KeyCode>>,
    ){
        let Some(mut c) = q else {return};
        if k.just_pressed(KeyCode::F2){
            c.enabled = !c.enabled;
        }
    }
}