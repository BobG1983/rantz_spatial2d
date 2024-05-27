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
                // Split into 6 pie segments with E as 0
                -101.25..=-78.75 => Self::N,
                -78.75..=-56.25 => Self::NNE,
                -56.25..=-33.75 => Self::NE,
                -33.75..=-11.25 => Self::ENE,
                -11.25..=11.25 => Self::E,
                11.25..=33.75 => Self::ESE,
                33.75..=56.25 => Self::SE,
                56.25..=78.75 => Self::SSE,
                78.75..=101.25 => Self::S,
                101.25..=123.75 => Self::SSW,
                123.75..=146.25 => Self::SW,
                146.25..=168.75 => Self::WSW,
                168.75..=180. | -180.0..=-168.75 => Self::W,
                -168.75..=-146.25 => Self::WNW,
                -146.25..=-123.75 => Self::NW,
                -123.75..=-101.25 => Self::NNW,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Degrees> for CompassHalfwinds {
        fn from(degrees: &Degrees) -> Self {
            Self::from(degrees.clone())
        }
    }

    impl From<Radians> for CompassHalfwinds {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<&Radians> for CompassHalfwinds {
        fn from(radians: &Radians) -> Self {
            Self::from(radians.clone())
        }
    }

    impl From<Rotation2D> for CompassHalfwinds {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<&Rotation2D> for CompassHalfwinds {
        fn from(rotation: &Rotation2D) -> Self {
            Self::from(rotation.clone())
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

    impl From<&Compass> for CompassHalfwinds {
        fn from(compass_cardinal: &Compass) -> Self {
            Self::from(compass_cardinal.clone())
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

    impl From<&CompassRose> for CompassHalfwinds {
        fn from(compass_cardinal: &CompassRose) -> Self {
            Self::from(compass_cardinal.clone())
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

    impl From<&CompassHalfwinds> for Vec2 {
        fn from(compass: &CompassHalfwinds) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }
}

mod operators {}
