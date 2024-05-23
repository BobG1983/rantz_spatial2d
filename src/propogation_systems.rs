use crate::prelude::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SpatialSystems2D {
    Propogate,
}

pub fn propogate_spatial2d(
    mut query: Query<(
        &mut Transform,
        &Position2D,
        &Rotation2D,
        &Scale2D,
        &DrawOrder,
        &RotationPropogation,
        &PositionPropogation,
        &ScalePropogation,
        Option<&Parent>,
    )>,
    all_parent_rotations: Query<&Rotation2D, With<Children>>,
    all_parent_positions: Query<&Position2D, With<Children>>,
    all_parent_scales: Query<&Scale2D, With<Children>>,
) {
    query.par_iter_mut().for_each(
        |(mut transform, position, rotation, scale, draw_order, r_prop, p_prop, s_prop, parent)| {
            let mut new_rot = Quat::from_rotation_z(rotation.degrees().into());
            let mut new_pos = Vec3::new(position.x, position.y, draw_order.into());
            let mut new_scale = Vec3::new(scale.x, scale.y, 1.0);

            if let Some(parent) = parent {
                let parent_rotation = all_parent_rotations.get(parent.get()).unwrap();
                let parent_position = all_parent_positions.get(parent.get()).unwrap();
                let parent_scale = all_parent_scales.get(parent.get()).unwrap();

                if r_prop == &RotationPropogation::Absolute {
                    // Counteract parent's rotation effects on position
                    let temp = position.rotate_radians(-parent_rotation.radians());
                    new_pos.x = temp.x;
                    new_pos.y = temp.y;
                    // Counteract parent's rotation
                    new_rot = Quat::from(-parent_rotation);
                }

                if p_prop == &PositionPropogation::Absolute {
                    // Counteract parent's position
                    new_pos.x = -parent_position.x;
                    new_pos.y = -parent_position.y;
                }

                if s_prop == &ScalePropogation::Absolute {
                    // Counteract parent's scale
                    new_scale.x = 1.0 / parent_scale.x;
                    new_scale.y = 1.0 / parent_scale.y;
                }
            }

            transform.rotation = new_rot;
            transform.translation = new_pos;
            transform.scale = new_scale;
        },
    )
}
