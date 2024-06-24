#![allow(clippy::type_complexity)]
mod compass;
mod compass_halfwinds;
mod compass_rose;
mod degrees;
mod draw_order;
mod position2d;
#[cfg(feature = "bevy")]
mod propagation_systems;
mod radians;
mod rotation2d;
mod scale2d;
#[cfg(feature = "bevy")]
mod spatialbundle2d;
#[cfg(feature = "bevy")]
mod spatialplugin2d;

pub mod components {
    pub use crate::compass::Compass;
    pub use crate::compass_halfwinds::CompassHalfwinds;
    pub use crate::compass_rose::CompassRose;
    pub use crate::draw_order::DrawOrder;
    pub use crate::position2d::Position2D;
    pub use crate::position2d::PositionPropagation;
    pub use crate::rotation2d::Rotation2D;
    pub use crate::rotation2d::RotationPropagation;
    pub use crate::scale2d::Scale2D;
    pub use crate::scale2d::ScalePropagation;
    #[cfg(feature = "bevy")]
    pub use crate::spatialbundle2d::SpatialBundle2D;
    #[cfg(feature = "bevy")]
    pub use crate::spatialbundle2d::SpatialBundle2DRaw;
}

pub mod math {
    pub use crate::degrees::Degrees;
    pub use crate::radians::Radians;
}

#[cfg(feature = "bevy")]
pub mod plugins {
    pub use crate::spatialplugin2d::SpatialPlugin2D;
}

#[cfg(feature = "bevy")]
pub mod systems {
    pub use crate::propagation_systems::propagate_spatial2d;
    pub use crate::propagation_systems::update_compass_from_rotation2d;
    pub use crate::propagation_systems::update_compass_halfwinds_from_rotation2d;
    pub use crate::propagation_systems::update_compass_rose_from_rotation2d;
    pub use crate::propagation_systems::SpatialSystems2D;
}

pub mod prelude {
    pub use crate::components::*;
    pub use crate::math::*;

    #[cfg(feature = "bevy")]
    pub use crate::plugins::*;
    #[cfg(feature = "bevy")]
    pub use crate::systems::*;
}
