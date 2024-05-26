use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Hash, Reflect)]
pub enum CompassHalfwinds {
    #[default]
    N,
    NNE,
    NE,
    ENE,
    E,
    ESE,
    SE,
    SSE,
    S,
    SSW,
    SW,
    WSW,
    W,
    WNW,
    NW,
    NNW,
}

impl CompassHalfwinds {}

mod from {
    use super::CompassHalfwinds;
    use crate::{
        compass::Compass,
        components::{CompassRose, Rotation2D},
        math::{Degrees, Radians},
    };

    impl From<Degrees> for CompassHalfwinds {
        fn from(degrees: Degrees) -> Self {
            match degrees.to_f32() {
                348.75..=360.0 | 0.0..11.25 => Self::N,
                11.25..33.75 => Self::NNE,
                33.75..56.25 => Self::NE,
                56.25..78.75 => Self::ENE,
                78.75..101.25 => Self::E,
                101.25..123.75 => Self::ESE,
                123.75..146.25 => Self::SE,
                146.25..168.75 => Self::SSE,
                168.75..191.25 => Self::S,
                191.25..213.75 => Self::SSW,
                213.75..236.25 => Self::SW,
                236.25..258.75 => Self::WSW,
                258.75..281.25 => Self::W,
                281.25..303.75 => Self::WNW,
                303.75..326.25 => Self::NW,
                326.25..348.75 => Self::NNW,
                _ => unreachable!(),
            }
        }
    }

    impl From<Radians> for CompassHalfwinds {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<Rotation2D> for CompassHalfwinds {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<Compass> for CompassHalfwinds {
        fn from(compass_cardinal: Compass) -> Self {
            match compass_cardinal {
                Compass::N => Self::N,
                Compass::S => Self::S,
                Compass::E => Self::E,
                Compass::W => Self::W,
            }
        }
    }

    impl From<CompassRose> for CompassHalfwinds {
        fn from(compass_cardinal: CompassRose) -> Self {
            match compass_cardinal {
                CompassRose::N => Self::N,
                CompassRose::S => Self::S,
                CompassRose::E => Self::E,
                CompassRose::W => Self::W,
                CompassRose::NE => Self::NE,
                CompassRose::SE => Self::SE,
                CompassRose::SW => Self::SW,
                CompassRose::NW => Self::NW,
            }
        }
    }

    impl<T> From<&T> for CompassHalfwinds
    where
        T: Into<CompassHalfwinds>,
    {
        fn from(value: &T) -> Self {
            value.into()
        }
    }
}

mod into {
    use crate::math::Radians;

    use super::CompassHalfwinds;
    use bevy::math::Vec2;

    impl From<CompassHalfwinds> for Vec2 {
        fn from(compass: CompassHalfwinds) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }
}

mod operators {}
