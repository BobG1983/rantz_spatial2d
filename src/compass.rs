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
    use bevy::prelude::debug;

    use super::Compass;
    use crate::{
        components::Rotation2D,
        math::{Degrees, Radians},
    };

    impl From<Degrees> for Compass {
        fn from(degrees: Degrees) -> Self {
            debug!("Compass from Degrees: {:?}", degrees);
            match degrees.to_f32() {
                315.0..=360.0 | 0.0..45.0 => Self::E,
                45.0..135.0 => Self::N,
                135.0..225.0 => Self::W,
                225.0..315.0 => Self::S,
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
            debug!("Compass from Rotation2D: {:?}", rotation);
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
