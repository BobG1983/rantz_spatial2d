use crate::prelude::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub struct SpatialPlugin2D;

impl Plugin for SpatialPlugin2D {
    fn build(&self, app: &mut App) {
        app.register_type::<DrawOrder>()
            .register_type::<RotationPropagation>()
            .register_type::<PositionPropagation>()
            .register_type::<ScalePropagation>()
            .register_type::<Position2D>()
            .register_type::<Rotation2D>()
            .register_type::<Scale2D>()
            .register_type::<Degrees>()
            .register_type::<Radians>()
            .register_type::<Compass>()
            .register_type::<CompassHalfwinds>()
            .register_type::<CompassRose>()
            .add_systems(
                PostUpdate,
                (
                    propagate_spatial2d,
                    update_compass_from_rotation2d,
                    update_compass_halfwinds_from_rotation2d,
                    update_compass_rose_from_rotation2d,
                )
                    .in_set(SpatialSystems2D::Propagate)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                PostStartup,
                (
                    propagate_spatial2d,
                    update_compass_from_rotation2d,
                    update_compass_halfwinds_from_rotation2d,
                    update_compass_rose_from_rotation2d,
                )
                    .in_set(SpatialSystems2D::Propagate)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
