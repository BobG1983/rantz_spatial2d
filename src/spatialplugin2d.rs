use crate::prelude::*;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub struct SpatialPlugin2D;

impl Plugin for SpatialPlugin2D {
    fn build(&self, app: &mut App) {
        app.register_type::<DrawOrder>()
            .register_type::<RotationPropogation>()
            .register_type::<PositionPropogation>()
            .register_type::<ScalePropogation>()
            .register_type::<Position2D>()
            .register_type::<Rotation2D>()
            .register_type::<Scale2D>()
            .register_type::<Degrees>()
            .register_type::<Radians>()
            .add_systems(
                PostUpdate,
                (propogate_spatial2d)
                    .in_set(Spatial2dSystems::Propogate)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                PostStartup,
                (propogate_spatial2d)
                    .in_set(Spatial2dSystems::Propogate)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
