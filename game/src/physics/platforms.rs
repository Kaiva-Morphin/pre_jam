use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use utils::{PingPongRem, WrappedDelta};




pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, 
            update_platforms
        );
    }
}


pub enum MovingPlatformMode {
    Loop,
    PingPong,
}
#[derive(Component)]
pub struct MovingPlatform {
    pub positions: Vec<Vec2>,
    pub index: usize,
    pub speed: f32,
    pub mode: MovingPlatformMode,
    pub velocity: Vec2,
}

impl MovingPlatform {
    pub fn bundle(positions: Vec<Vec2>, speed: f32, mode: MovingPlatformMode) -> impl Bundle {
        (
            Transform::from_translation(positions.first().unwrap().extend(0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Dominance::group(1),
            Velocity::zero(),
            GravityScale(0.0),
            // KinematicCharacterController{
            //     // apply_impulse_to_dynamic_bodies: true,
            //     // slide: false,
            //     // autostep: None,
            //     // snap_to_ground: None,
            //     ..default()
            // },
            
            Self {
                positions,
                index: 0,
                speed,
                mode,
                velocity: Vec2::ZERO,
            },
        )
    }

    fn get_index(&self, index: usize) -> usize {
        match self.mode {
            MovingPlatformMode::Loop => index % self.positions.len(),
            MovingPlatformMode::PingPong => index.pingpong_rem(self.positions.len()),
        }
    }
    pub fn tick(&mut self, dt: f32, current_position: &Vec3) -> Vec2 {
        let to_travel = self.speed * dt;
        let to_travel_sq = to_travel * to_travel;
        let curr = current_position.truncate();
        let next_index = self.get_index(self.index + 1);
        let next_position = self.positions.get(next_index).unwrap();
        let distance_sq = next_position.distance_squared(curr);
        let v = if to_travel_sq > distance_sq {
            let rest = to_travel - distance_sq.sqrt();
            let next_next_index = self.get_index(next_index + 2);
            let next_next_position = self.positions.get(next_next_index).unwrap();
            let vec = (next_next_position - next_position).normalize();
            self.index = next_index;
            next_position + vec * rest - curr
        } else {
            let vec = (next_position - curr).normalize();
            vec * to_travel
        };
        self.velocity = v;
        v
    }
}


pub fn update_platforms(
    mut transforms: Query<(&mut Transform, &mut Velocity, &mut MovingPlatform, Option<&mut KinematicCharacterController>)>,
    time: Res<Time>,
){
    let dt = time.dt();
    for (mut t, mut vel, mut platform, mut controller) in transforms.iter_mut(){
        let translation = platform.tick(dt, &t.translation);
        // controller.translation = Some(translation);
        vel.linvel = translation * 100.0;
        // t.translation += translation.extend(0.);
    }
}