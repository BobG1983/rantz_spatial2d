use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Hash, Reflect)]
pub enum CompassRose {
    #[default]
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CompassRose {}

mod from {
    use super::CompassRose;
    use crate::{
        compass::Compass,
        components::Rotation2D,
        math::{Degrees, Radians},
    };

    impl From<Degrees> for CompassRose {
        fn from(degrees: Degrees) -> Self {
            match degrees.to_f32() {
                330.0..=360.0 | 0.0..30.0 => Self::E,
                30.0..60.0 => Self::NE,
                60.0..120.0 => Self::N,
                120.0..150.0 => Self::NW,
                150.0..210.0 => Self::W,
                210.0..240.0 => Self::SW,
                240.0..300.0 => Self::S,
                300.0..330.0 => Self::SE,
                _ => unreachable!(),
            }
        }
    }

    impl From<Radians> for CompassRose {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<Rotation2D> for CompassRose {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<Compass> for CompassRose {
        fn from(compass: Compass) -> Self {
            match compass {
                Compass::N => Self::N,
                Compass::S => Self::S,
                Compass::E => Self::E,
                Compass::W => Self::W,
            }
        }
    }

    impl<T> From<&T> for CompassRose
    where
        T: Into<CompassRose>,
    {
        fn from(value: &T) -> Self {
            value.into()
        }
    }
}

mod into {
    use super::CompassRose;
    use crate::math::Radians;
    use bevy::math::Vec2;

    impl From<CompassRose> for Vec2 {
        fn from(compass: CompassRose) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }

    impl From<&CompassRose> for Vec2 {
        fn from(compass: &CompassRose) -> Self {
            compass.into()
        }
    }
}

mod operators {}
