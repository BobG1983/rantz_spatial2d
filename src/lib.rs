mod degrees;
mod draw_order;
mod position2d;
mod propogation_systems;
mod radians;
mod rotation2d;
mod scale2d;
mod spatialbundle2d;
mod spatialplugin2d;

pub mod components {
    pub use crate::draw_order::DrawOrder;
    pub use crate::position2d::Position2D;
    pub use crate::position2d::PositionPropogation;
    pub use crate::rotation2d::Rotation2D;
    pub use crate::rotation2d::RotationPropogation;
    pub use crate::scale2d::Scale2D;
    pub use crate::scale2d::ScalePropogation;
    pub use crate::spatialbundle2d::SpatialBundle2D;
    pub use crate::spatialbundle2d::SpatialBundle2DRaw;
}

pub mod math {
    pub use crate::degrees::Degrees;
    pub use crate::radians::Radians;
}

pub mod plugins {
    pub use crate::spatialplugin2d::SpatialPlugin2D;
}

pub mod systems {
    pub use crate::propogation_systems::propogate_spatial2d;
    pub use crate::propogation_systems::SpatialSystems2D;
}

pub mod prelude {
    pub use crate::components::*;
    pub use crate::math::*;
    pub use crate::plugins::*;
    pub use crate::systems::*;
}
