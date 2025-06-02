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
    #[derive(Resource)]
    pub struct RapierDebugSwitch(pub bool);
    impl Plugin for SwitchableRapierDebugPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app
                .add_plugins(RapierDebugRenderPlugin::default())
                .add_systems(Update, update)
                .add_systems(Startup, start)
                .insert_resource(RapierDebugSwitch(self.0));
        }
    }

    fn start (
        q: Option<ResMut<DebugRenderContext>>,
        s: Res<RapierDebugSwitch>,
    ){
        let Some(mut c) = q else {return};
        c.enabled = !s.0;
    }
    fn update(
        q: Option<ResMut<DebugRenderContext>>,
        k: Res<ButtonInput<KeyCode>>,
        mut s: ResMut<RapierDebugSwitch>,
    ){
        let Some(mut c) = q else {return};
        if k.just_pressed(KeyCode::F2){
            s.0 = !s.0;
            c.enabled = !s.0;
        }
    }
}