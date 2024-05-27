use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Hash, Reflect)]
pub enum Compass {
    #[default]
    N,
    S,
    E,
    W,
}

mod from {
    use super::Compass;
    use crate::{
        components::Rotation2D,
        math::{Degrees, Radians},
    };

    impl From<Degrees> for Compass {
        fn from(degrees: Degrees) -> Self {
            let angle = degrees.to_f32();
            let mut normalized_angle = ((angle % 360.0) + 360.) % 360.0;
            if normalized_angle > 180. {
                normalized_angle -= 360.;
            }
            match normalized_angle {
                -135.0..=-45.0 => Self::S,
                -45.0..=45.0 => Self::E,
                45.0..=135.0 => Self::N,
                135.0..=180.0 | -180.0..=-135.0 => Self::W,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Degrees> for Compass {
        fn from(degrees: &Degrees) -> Self {
            Self::from(degrees.clone())
        }
    }

    impl From<Radians> for Compass {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<&Radians> for Compass {
        fn from(radians: &Radians) -> Self {
            Self::from(radians.clone())
        }
    }

    impl From<Rotation2D> for Compass {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(rotation.degrees())
        }
    }

    impl From<&Rotation2D> for Compass {
        fn from(rotation: &Rotation2D) -> Self {
            Self::from(rotation.clone())
        }
    }
}

mod into {
    use super::Compass;
    use crate::math::Radians;
    use bevy::math::Vec2;

    impl From<Compass> for Vec2 {
        fn from(compass: Compass) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }

    impl From<&Compass> for Vec2 {
        fn from(compass: &Compass) -> Self {
            compass.into()
        }
    }
}
