use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct Rotation2D {
    degrees: Degrees,
    radians: Radians,
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Reflect)]
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

    pub fn from_radians(radians: &Radians) -> Self {
        Self::from_f32_radians(radians.to_f32())
    }

    pub fn from_degrees(degrees: &Degrees) -> Self {
        Self::from_f32_degrees(degrees.to_f32())
    }

    pub fn from_f32_radians(radians: f32) -> Self {
        let radians = Radians::from_f32(radians);
        let degrees = Degrees::from(radians);
        Self { degrees, radians }
    }

    pub fn from_f32_degrees(degrees: f32) -> Self {
        let degrees = Degrees::from_f32(degrees);
        let radians = Radians::from(degrees);
        Self { degrees, radians }
    }
}

mod conversions {
    use super::{Degrees, Radians, Rotation2D};
    use bevy::math::{EulerRot, Quat};

    impl From<Radians> for Rotation2D {
        fn from(radians: Radians) -> Self {
            Self {
                degrees: Degrees::from(radians),
                radians,
            }
        }
    }

    impl From<&Radians> for Rotation2D {
        fn from(radians: &Radians) -> Self {
            Self::from_radians(&radians)
        }
    }

    impl From<Degrees> for Rotation2D {
        fn from(degrees: Degrees) -> Self {
            Self {
                degrees,
                radians: Radians::from_degrees(&degrees),
            }
        }
    }

    impl From<&Degrees> for Rotation2D {
        fn from(degrees: &Degrees) -> Self {
            Self {
                degrees: degrees.clone(),
                radians: Radians::from_degrees(degrees),
            }
        }
    }

    impl From<Quat> for Rotation2D {
        fn from(value: Quat) -> Self {
            Self::from_f32_radians(value.to_euler(EulerRot::XYZ).2)
        }
    }

    impl From<&Quat> for Rotation2D {
        fn from(value: &Quat) -> Self {
            Self::from_f32_radians(value.to_euler(EulerRot::XYZ).2)
        }
    }

    impl From<Rotation2D> for Quat {
        fn from(rotation: Rotation2D) -> Self {
            Quat::from_rotation_z(rotation.radians().into())
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
                Rotation2D::from_degrees(&(self.degrees + rhs))
            }
        }

        impl Add<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees + rhs))
            }
        }

        impl Add<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians + rhs))
            }
        }

        impl Add<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians + rhs))
            }
        }

        impl Add<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees + rhs))
            }
        }

        impl Add<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees + rhs))
            }
        }

        impl Add<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians + rhs))
            }
        }

        impl Add<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn add(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians + rhs))
            }
        }

        impl AddAssign<Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees + rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl AddAssign<&Degrees> for Rotation2D {
            fn add_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees + rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl AddAssign<Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: Radians) {
                self.radians = self.radians + rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }

        impl AddAssign<&Radians> for Rotation2D {
            fn add_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians + rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }
    }
    mod sub {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Sub, SubAssign};

        impl Sub<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees - rhs))
            }
        }

        impl Sub<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees - rhs))
            }
        }

        impl Sub<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians - rhs))
            }
        }

        impl Sub<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians - rhs))
            }
        }

        impl Sub<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees - rhs))
            }
        }

        impl Sub<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees - rhs))
            }
        }

        impl Sub<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians - rhs))
            }
        }

        impl Sub<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians - rhs))
            }
        }

        impl SubAssign<Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees - rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl SubAssign<&Degrees> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees - rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl SubAssign<Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: Radians) {
                self.radians = self.radians - rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }

        impl SubAssign<&Radians> for Rotation2D {
            fn sub_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians - rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }
    }
    mod mul {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Mul, MulAssign};

        impl Mul<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees * rhs))
            }
        }

        impl Mul<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees * rhs))
            }
        }

        impl Mul<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians * rhs))
            }
        }

        impl Mul<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians * rhs))
            }
        }

        impl Mul<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees * rhs))
            }
        }

        impl Mul<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees * rhs))
            }
        }

        impl Mul<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians * rhs))
            }
        }

        impl Mul<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians * rhs))
            }
        }

        impl MulAssign<Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees * rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl MulAssign<&Degrees> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees * rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl MulAssign<Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: Radians) {
                self.radians = self.radians * rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }

        impl MulAssign<&Radians> for Rotation2D {
            fn mul_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians * rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }
    }
    mod div {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Div, DivAssign};

        impl Div<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees / rhs))
            }
        }

        impl Div<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees / rhs))
            }
        }

        impl Div<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians / rhs))
            }
        }

        impl Div<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians / rhs))
            }
        }

        impl Div<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees / rhs))
            }
        }

        impl Div<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees / rhs))
            }
        }

        impl Div<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians / rhs))
            }
        }

        impl Div<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn div(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians / rhs))
            }
        }

        impl DivAssign<Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees / rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl DivAssign<&Degrees> for Rotation2D {
            fn div_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees / rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl DivAssign<Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: Radians) {
                self.radians = self.radians / rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }

        impl DivAssign<&Radians> for Rotation2D {
            fn div_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians / rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }
    }
    mod rem {
        use crate::prelude::{Degrees, Radians, Rotation2D};
        use std::ops::{Rem, RemAssign};

        impl Rem<Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees % rhs))
            }
        }

        impl Rem<&Degrees> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees % rhs))
            }
        }

        impl Rem<Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians % rhs))
            }
        }

        impl Rem<&Radians> for Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians % rhs))
            }
        }

        impl Rem<Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees % rhs))
            }
        }

        impl Rem<&Degrees> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Rotation2D::from_degrees(&(self.degrees % rhs))
            }
        }

        impl Rem<Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians % rhs))
            }
        }

        impl Rem<&Radians> for &Rotation2D {
            type Output = Rotation2D;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Rotation2D::from_radians(&(self.radians % rhs))
            }
        }

        impl RemAssign<Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: Degrees) {
                self.degrees = self.degrees % rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl RemAssign<&Degrees> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Degrees) {
                self.degrees = self.degrees % rhs;
                self.radians = Radians::from_degrees(&self.degrees);
            }
        }

        impl RemAssign<Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: Radians) {
                self.radians = self.radians % rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }

        impl RemAssign<&Radians> for Rotation2D {
            fn rem_assign(&mut self, rhs: &Radians) {
                self.radians = self.radians % rhs;
                self.degrees = Degrees::from_radians(&self.radians);
            }
        }
    }
    mod neg {
        use crate::prelude::Rotation2D;
        use std::ops::Neg;

        impl Neg for Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from_radians(&(-self.radians))
            }
        }

        impl Neg for &Rotation2D {
            type Output = Rotation2D;
            fn neg(self) -> Self::Output {
                Rotation2D::from_radians(&(-self.radians))
            }
        }
    }
}
