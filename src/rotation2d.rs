use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Debug, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotation2D {
    rot: Rot2,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RotationPropagation {
    #[default]
    Relative,
    Absolute,
}

impl Rotation2D {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn radians(&self) -> Radians {
        Radians::from_f32(self.rot.as_radians())
    }

    pub fn degrees(&self) -> Degrees {
        Degrees::from_f32(self.rot.as_degrees())
    }

    pub fn from_f32_radians(radians: f32) -> Self {
        Self {
            rot: Rot2::radians(radians),
        }
    }

    pub fn from_f32_degrees(degrees: f32) -> Self {
        Self {
            rot: Rot2::degrees(degrees),
        }
    }
}

mod from {
    use super::{Compass, CompassHalfwinds, CompassRose, Degrees, Radians, Rotation2D};

    use bevy::math::{EulerRot, Quat};

    impl From<Radians> for Rotation2D {
        fn from(radians: Radians) -> Self {
            Self::from_f32_radians(radians.to_f32())
        }
    }

    impl From<&Radians> for Rotation2D {
        fn from(radians: &Radians) -> Self {
            Self::from_f32_radians(radians.to_f32())
        }
    }

    impl From<Degrees> for Rotation2D {
        fn from(degrees: Degrees) -> Self {
            Self::from_f32_degrees(degrees.to_f32())
        }
    }

    impl From<&Degrees> for Rotation2D {
        fn from(degrees: &Degrees) -> Self {
            Self::from_f32_degrees(degrees.to_f32())
        }
    }

    impl From<Quat> for Rotation2D {
        fn from(value: Quat) -> Self {
            Self::from_f32_radians(value.to_euler(EulerRot::XYZ).2)
        }
    }

    impl From<&Quat> for Rotation2D {
        fn from(value: &Quat) -> Self {
            Self::from(*value)
        }
    }

    impl From<Compass> for Rotation2D {
        fn from(compass: Compass) -> Self {
            Self::from(Radians::from(compass))
        }
    }

    impl From<&Compass> for Rotation2D {
        fn from(compass: &Compass) -> Self {
            Self::from(*compass)
        }
    }

    impl From<CompassRose> for Rotation2D {
        fn from(compass_rose: CompassRose) -> Self {
            Self::from(Radians::from(compass_rose))
        }
    }

    impl From<&CompassRose> for Rotation2D {
        fn from(compass_rose: &CompassRose) -> Self {
            Self::from(*compass_rose)
        }
    }

    impl From<CompassHalfwinds> for Rotation2D {
        fn from(compass_halfwinds: CompassHalfwinds) -> Self {
            Self::from(Radians::from(compass_halfwinds))
        }
    }

    impl From<&CompassHalfwinds> for Rotation2D {
        fn from(compass_halfwinds: &CompassHalfwinds) -> Self {
            Self::from(*compass_halfwinds)
        }
    }
}

mod into {
    use super::Rotation2D;
    use bevy::math::Quat;

    impl From<Rotation2D> for Quat {
        fn from(rotation: Rotation2D) -> Self {
            Quat::from_rotation_z(rotation.radians().to_f32())
        }
    }

    impl From<&Rotation2D> for Quat {
        fn from(rotation: &Rotation2D) -> Self {
            Quat::from_rotation_z(rotation.radians().to_f32())
        }
    }
}

mod operators {
    mod add {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Add, AddAssign};

        impl Add<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() + rhs))
            }
        }

        impl Add<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() + rhs))
            }
        }

        impl Add<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() + rhs))
            }
        }

        impl Add<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() + rhs))
            }
        }

        impl Add<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() + rhs))
            }
        }

        impl Add<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() + rhs))
            }
        }

        impl Add<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() + rhs))
            }
        }

        impl Add<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() + rhs))
            }
        }

        impl AddAssign<Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: Degrees) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: &Degrees) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: Radians) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: &Radians) {
                *self = *self + rhs;
            }
        }
    }
    mod sub {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Sub, SubAssign};

        impl Sub<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() - rhs))
            }
        }

        impl Sub<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() - rhs))
            }
        }

        impl Sub<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() - rhs))
            }
        }

        impl Sub<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() - rhs))
            }
        }

        impl Sub<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() - rhs))
            }
        }

        impl Sub<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() - rhs))
            }
        }

        impl Sub<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() - rhs))
            }
        }

        impl Sub<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() - rhs))
            }
        }

        impl SubAssign<Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: Degrees) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Degrees) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: Radians) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Radians) {
                *self = *self - rhs;
            }
        }
    }
    mod mul {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Mul, MulAssign};

        impl Mul<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() * rhs))
            }
        }

        impl Mul<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() * rhs))
            }
        }

        impl Mul<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() * rhs))
            }
        }

        impl Mul<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() * rhs))
            }
        }

        impl Mul<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() * rhs))
            }
        }

        impl Mul<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() * rhs))
            }
        }

        impl Mul<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() * rhs))
            }
        }

        impl Mul<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() * rhs))
            }
        }

        impl MulAssign<Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: Degrees) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Degrees) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: Radians) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Radians) {
                *self = *self * rhs;
            }
        }
    }
    mod div {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Div, DivAssign};

        impl Div<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() / rhs))
            }
        }

        impl Div<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() / rhs))
            }
        }

        impl Div<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() / rhs))
            }
        }

        impl Div<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() / rhs))
            }
        }

        impl Div<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() / rhs))
            }
        }

        impl Div<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() / rhs))
            }
        }

        impl Div<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() / rhs))
            }
        }

        impl Div<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() / rhs))
            }
        }

        impl DivAssign<Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: Degrees) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: &Degrees) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: Radians) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: &Radians) {
                *self = *self / rhs;
            }
        }
    }
    mod rem {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Rem, RemAssign};

        impl Rem<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() % rhs))
            }
        }

        impl Rem<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() % rhs))
            }
        }

        impl Rem<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() % rhs))
            }
        }

        impl Rem<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() % rhs))
            }
        }

        impl Rem<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() % rhs))
            }
        }

        impl Rem<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees() % rhs))
            }
        }

        impl Rem<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() % rhs))
            }
        }

        impl Rem<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians() % rhs))
            }
        }

        impl RemAssign<Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: Degrees) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Degrees) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: Radians) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Radians) {
                *self = *self % rhs;
            }
        }
    }
    mod neg {
        use crate::prelude::Rotation2D;
        use std::ops::Neg;

        impl Neg for Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from(&(-self.radians()))
            }
        }

        impl Neg for &Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from(&(-self.radians()))
            }
        }
    }
}
