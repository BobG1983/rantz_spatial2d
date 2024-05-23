use crate::prelude::*;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct SpatialBundle2DRaw {
    pub position: Position2D,
    pub rotation: Rotation2D,
    pub scale: Scale2D,
    pub draw_order: DrawOrder,
    pub r_prop: RotationPropogation,
    pub p_prop: PositionPropogation,
    pub s_prop: ScalePropogation,
}

#[derive(Bundle, Default)]
pub struct SpatialBundle2D {
    pub position: Position2D,
    pub rotation: Rotation2D,
    pub scale: Scale2D,
    pub draw_order: DrawOrder,
    pub spatial: SpatialBundle,
    pub r_prop: RotationPropogation,
    pub p_prop: PositionPropogation,
    pub s_prop: ScalePropogation,
}
