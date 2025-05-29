use bevy::time::Time;




pub trait WrappedDelta {
    fn dt(&self) -> f32;
}

impl WrappedDelta for Time {
    fn dt(&self) -> f32 {
        self.delta_secs().min(0.5)
    }
}

#[macro_export]
macro_rules! wrap {
    ($name:ident($ty:ty)) => {
        struct $name($ty);
        $crate::wrap!(@$name($ty));
    };
    ($name:ident(pub $ty:ty)) => {
        struct $name(pub $ty);
        $crate::wrap!(@$name($ty));
    };
    (pub $name:ident($ty:ty)) => {
        pub struct $name($ty);
        $crate::wrap!(@$name($ty));
    };
    (pub $name:ident(pub $ty:ty)) => {
        pub struct $name(pub $ty);
        $crate::wrap!(@$name($ty));
    };
    (@$name:ident($ty:ty)) => {
        impl std::ops::Deref for $name {
            type Target = $ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $ty {
                &mut self.0
            }
        }
    };
}

pub trait PingPongRem{
    fn pingpong_rem(&self, max: Self) -> Self;
}


impl PingPongRem for u32 {
    fn pingpong_rem(&self, max: Self) -> Self {
        let r = self.rem_euclid((max * 2).max(1));
        if r / max > 0 {
            max - r / max * r % max
        } else {
            r
        }
    }
}

impl PingPongRem for usize {
    fn pingpong_rem(&self, max: Self) -> Self {
        let r = self.rem_euclid((max * 2).max(1));
        if r / max > 0 {
            max - r / max * r % max
        } else {
            r
        }
    }
}

