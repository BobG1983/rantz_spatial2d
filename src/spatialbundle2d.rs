use crate::prelude::*;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct SpatialBundle2DRaw {
    pub position: Position2D,
    pub rotation: Rotation2D,
    pub scale: Scale2D,
    pub draw_order: DrawOrder,
    pub r_prop: RotationPropagation,
    pub p_prop: PositionPropagation,
    pub s_prop: ScalePropagation,
}

#[derive(Bundle, Default)]
pub struct SpatialBundle2D {
    pub position: Position2D,
    pub rotation: Rotation2D,
    pub scale: Scale2D,
    pub draw_order: DrawOrder,
    pub spatial: SpatialBundle,
    pub r_prop: RotationPropagation,
    pub p_prop: PositionPropagation,
    pub s_prop: ScalePropagation,
}
