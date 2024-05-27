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
                // Degrees in range of -180 to 180, split into 8 pie segments with E as 0
                -112.5..=-67.5 => Self::N,
                -67.5..=-22.5 => Self::NE,
                -22.5..=22.5 => Self::E,
                22.5..=67.5 => Self::SE,
                67.5..=112.5 => Self::S,
                112.5..=157.5 => Self::SW,
                157.5..=180.0 | -180.0..=-157.5 => Self::W,
                -157.5..=-112.5 => Self::NW,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Degrees> for CompassRose {
        fn from(degrees: &Degrees) -> Self {
            Self::from(degrees.clone())
        }
    }

    impl From<Radians> for CompassRose {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<&Radians> for CompassRose {
        fn from(radians: &Radians) -> Self {
            Self::from(radians.clone())
        }
    }

    impl From<Rotation2D> for CompassRose {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<&Rotation2D> for CompassRose {
        fn from(rotation: &Rotation2D) -> Self {
            Self::from(rotation.clone())
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

    impl From<&Compass> for CompassRose {
        fn from(compass: &Compass) -> Self {
            Self::from(compass.clone())
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
