use crate::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotation2D {
    degrees: Degrees,
    radians: Radians,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RotationPropogation {
    #[default]
    Relative,
    Absolute,
}

impl Rotation2D {
    pub fn new() -> Self {
        Default::default()
    }

    pub const ZERO: Self = Self {
        degrees: Degrees::ZERO,
        radians: Radians::ZERO,
    };
    pub const UP: Self = Self {
        degrees: Degrees::UP,
        radians: Radians::UP,
    };
    pub const DOWN: Self = Self {
        degrees: Degrees::DOWN,
        radians: Radians::DOWN,
    };
    pub const LEFT: Self = Self {
        degrees: Degrees::LEFT,
        radians: Radians::LEFT,
    };
    pub const RIGHT: Self = Self {
        degrees: Degrees::RIGHT,
        radians: Radians::RIGHT,
    };

    pub fn radians(&self) -> Radians {
        self.radians
    }

    pub fn degrees(&self) -> Degrees {
        self.degrees
    }

    pub fn from_f32_radians(radians: f32) -> Self {
        let radians = Radians::from_f32(radians);
        Self {
            degrees: radians.into(),
            radians,
        }
    }

    pub fn from_f32_degrees(degrees: f32) -> Self {
        let degrees = Degrees::from_f32(degrees);
        Self {
            degrees,
            radians: degrees.into(),
        }
    }
}

mod from {
    use super::{Compass, CompassHalfwinds, CompassRose, Degrees, Radians, Rotation2D};
    #[cfg(feature = "bevy")]
    use bevy::math::{EulerRot, Quat};

    impl From<Radians> for Rotation2D {
        fn from(radians: Radians) -> Self {
            Self {
                degrees: radians.into(),
                radians,
            }
        }
    }

    impl From<&Radians> for Rotation2D {
        fn from(radians: &Radians) -> Self {
            Self::from(radians.clone())
        }
    }

    impl From<Degrees> for Rotation2D {
        fn from(degrees: Degrees) -> Self {
            Self {
                degrees,
                radians: degrees.into(),
            }
        }
    }

    impl From<&Degrees> for Rotation2D {
        fn from(degrees: &Degrees) -> Self {
            Self::from(degrees.clone())
        }
    }

    #[cfg(feature = "bevy")]
    impl From<Quat> for Rotation2D {
        fn from(value: Quat) -> Self {
            Self::from_f32_radians(value.to_euler(EulerRot::XYZ).2)
        }
    }

    #[cfg(feature = "bevy")]
    impl From<&Quat> for Rotation2D {
        fn from(value: &Quat) -> Self {
            Self::from(value.clone())
        }
    }

    impl From<Compass> for Rotation2D {
        fn from(compass: Compass) -> Self {
            Self::from(Radians::from(compass))
        }
    }

    impl From<&Compass> for Rotation2D {
        fn from(compass: &Compass) -> Self {
            Self::from(compass.clone())
        }
    }

    impl From<CompassRose> for Rotation2D {
        fn from(compass_rose: CompassRose) -> Self {
            Self::from(Radians::from(compass_rose))
        }
    }

    impl From<&CompassRose> for Rotation2D {
        fn from(compass_rose: &CompassRose) -> Self {
            Self::from(compass_rose.clone())
        }
    }

    impl From<CompassHalfwinds> for Rotation2D {
        fn from(compass_halfwinds: CompassHalfwinds) -> Self {
            Self::from(Radians::from(compass_halfwinds))
        }
    }

    impl From<&CompassHalfwinds> for Rotation2D {
        fn from(compass_halfwinds: &CompassHalfwinds) -> Self {
            Self::from(compass_halfwinds.clone())
        }
    }
}

#[cfg(feature = "bevy")]
mod into {
    use super::Rotation2D;
    #[cfg(feature = "bevy")]
    use bevy::math::Quat;

    #[cfg(feature = "bevy")]
    impl From<Rotation2D> for Quat {
        fn from(rotation: Rotation2D) -> Self {
            Quat::from_rotation_z(rotation.radians().to_f32())
        }
    }

    #[cfg(feature = "bevy")]
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
                Rotation2D::from(&(self.degrees + rhs))
            }
        }

        impl Add<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees + rhs))
            }
        }

        impl Add<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians + rhs))
            }
        }

        impl Add<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians + rhs))
            }
        }

        impl Add<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees + rhs))
            }
        }

        impl Add<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees + rhs))
            }
        }

        impl Add<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians + rhs))
            }
        }

        impl Add<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians + rhs))
            }
        }

        impl AddAssign<Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees + rhs;
                self.radians = self.degrees.into();
            }
        }

        impl AddAssign<&Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees + rhs;
                self.radians = self.degrees.into();
            }
        }

        impl AddAssign<Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: Radians) {
                self.radians = self.radians + rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }

        impl AddAssign<&Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians + rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }
    }
    mod sub {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Sub, SubAssign};

        impl Sub<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees - rhs))
            }
        }

        impl Sub<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees - rhs))
            }
        }

        impl Sub<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians - rhs))
            }
        }

        impl Sub<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians - rhs))
            }
        }

        impl Sub<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees - rhs))
            }
        }

        impl Sub<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees - rhs))
            }
        }

        impl Sub<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians - rhs))
            }
        }

        impl Sub<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians - rhs))
            }
        }

        impl SubAssign<Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees - rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl SubAssign<&Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees - rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl SubAssign<Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: Radians) {
                self.radians = self.radians - rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }

        impl SubAssign<&Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians - rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }
    }
    mod mul {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Mul, MulAssign};

        impl Mul<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees * rhs))
            }
        }

        impl Mul<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees * rhs))
            }
        }

        impl Mul<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians * rhs))
            }
        }

        impl Mul<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians * rhs))
            }
        }

        impl Mul<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees * rhs))
            }
        }

        impl Mul<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees * rhs))
            }
        }

        impl Mul<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians * rhs))
            }
        }

        impl Mul<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians * rhs))
            }
        }

        impl MulAssign<Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees * rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl MulAssign<&Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees * rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl MulAssign<Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: Radians) {
                self.radians = self.radians * rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }

        impl MulAssign<&Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians * rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }
    }
    mod div {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Div, DivAssign};

        impl Div<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees / rhs))
            }
        }

        impl Div<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees / rhs))
            }
        }

        impl Div<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians / rhs))
            }
        }

        impl Div<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians / rhs))
            }
        }

        impl Div<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees / rhs))
            }
        }

        impl Div<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees / rhs))
            }
        }

        impl Div<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians / rhs))
            }
        }

        impl Div<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians / rhs))
            }
        }

        impl DivAssign<Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees / rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl DivAssign<&Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees / rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl DivAssign<Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: Radians) {
                self.radians = self.radians / rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }

        impl DivAssign<&Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians / rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }
    }
    mod rem {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Rem, RemAssign};

        impl Rem<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees % rhs))
            }
        }

        impl Rem<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees % rhs))
            }
        }

        impl Rem<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians % rhs))
            }
        }

        impl Rem<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians % rhs))
            }
        }

        impl Rem<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees % rhs))
            }
        }

        impl Rem<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from(&(self.degrees % rhs))
            }
        }

        impl Rem<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from(&(self.radians % rhs))
            }
        }

        impl Rem<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from(&(self.radians % rhs))
            }
        }

        impl RemAssign<Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees % rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl RemAssign<&Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees % rhs;
                self.radians = Radians::from(&self.degrees);
            }
        }

        impl RemAssign<Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: Radians) {
                self.radians = self.radians % rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }

        impl RemAssign<&Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians % rhs;
                self.degrees = Degrees::from(&self.radians);
            }
        }
    }
    mod neg {
        use crate::prelude::Rotation2D;
        use std::ops::Neg;

        impl Neg for Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from(&(-self.radians))
            }
        }

        impl Neg for &Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from(&(-self.radians))
            }
        }
    }
}
