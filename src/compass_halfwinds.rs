use bevy::prelude::{Component, Reflect};

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

mod from {
    use super::CompassHalfwinds;
    use crate::{
        compass::Compass,
        components::{CompassRose, Rotation2D},
        math::{Degrees, Radians},
    };

    impl From<Degrees> for CompassHalfwinds {
        fn from(degrees: Degrees) -> Self {
            let angle = degrees.to_f32();
            let mut normalized_angle = ((angle % 360.0) + 360.) % 360.0;
            if normalized_angle > 180. {
                normalized_angle -= 360.;
            }
            match normalized_angle {
                -101.25..=-78.75 => Self::S,
                -78.75..=-56.25 => Self::SSE,
                -56.25..=-33.75 => Self::SE,
                -33.75..=-11.25 => Self::ESE,
                -11.25..=11.25 => Self::E,
                11.25..=33.75 => Self::ENE,
                33.75..=56.25 => Self::NE,
                56.25..=78.75 => Self::NNE,
                78.75..=101.25 => Self::N,
                101.25..=123.75 => Self::NNW,
                123.75..=146.25 => Self::NW,
                146.25..=168.75 => Self::WNW,
                168.75..=180. | -180.0..=-168.75 => Self::W,
                -168.75..=-146.25 => Self::WSW,
                -146.25..=-123.75 => Self::SW,
                -123.75..=-101.25 => Self::SSW,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Degrees> for CompassHalfwinds {
        fn from(degrees: &Degrees) -> Self {
            Self::from(*degrees)
        }
    }

    impl From<Radians> for CompassHalfwinds {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<&Radians> for CompassHalfwinds {
        fn from(radians: &Radians) -> Self {
            Self::from(*radians)
        }
    }

    impl From<Rotation2D> for CompassHalfwinds {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<&Rotation2D> for CompassHalfwinds {
        fn from(rotation: &Rotation2D) -> Self {
            Self::from(*rotation)
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
            Self::from(*compass_cardinal)
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
            Self::from(*compass_cardinal)
        }
    }
}

mod into {
    use super::CompassHalfwinds;
    use crate::math::Radians;
    use bevy::math::Vec2;

    impl From<CompassHalfwinds> for Vec2 {
        fn from(compass: CompassHalfwinds) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }

    impl From<&CompassHalfwinds> for Vec2 {
        fn from(compass: &CompassHalfwinds) -> Self {
            (*compass).into()
        }
    }
}
