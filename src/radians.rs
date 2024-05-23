pub use crate::prelude::Degrees;
use bevy::prelude::*;
use std::f32::consts::TAU;

#[derive(Default, Clone, Copy, PartialEq, Debug, Deref, DerefMut, Reflect)]
pub struct Radians(f32);

impl Radians {
    pub fn new(radians: f32) -> Self {
        Self::from_f32(radians)
    }

    pub fn to_degrees(self) -> Degrees {
        Degrees::from_f32(self.0.to_degrees())
    }

    pub fn to_f32(self) -> f32 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(value % TAU)
    }

    pub fn from_degrees(degrees: &Degrees) -> Self {
        Self::from_f32(degrees.to_f32())
    }

    pub const ZERO: Self = Self(0.);
    pub const UP: Self = Self(std::f32::consts::FRAC_PI_2);
    pub const DOWN: Self = Self(-std::f32::consts::FRAC_PI_2);
    pub const LEFT: Self = Self(std::f32::consts::PI);
    pub const RIGHT: Self = Self::ZERO;
}

mod conversions {
    use super::{Degrees, Radians};
    use bevy::prelude::Quat;

    impl From<Degrees> for Radians {
        fn from(degrees: Degrees) -> Self {
            Self::from_degrees(&degrees)
        }
    }

    impl From<&Degrees> for Radians {
        fn from(degrees: &Degrees) -> Self {
            Self::from_degrees(degrees)
        }
    }

    impl From<f32> for Radians {
        fn from(value: f32) -> Self {
            Self::from_f32(value)
        }
    }

    impl From<Radians> for f32 {
        fn from(radians: Radians) -> Self {
            radians.0
        }
    }

    impl From<Radians> for Quat {
        fn from(radians: Radians) -> Self {
            Quat::from_rotation_z(radians.0)
        }
    }

    impl From<&Radians> for Quat {
        fn from(radians: &Radians) -> Self {
            Quat::from_rotation_z(radians.0)
        }
    }
}

mod operators {
    mod add {
        use std::{
            f32::consts::TAU,
            ops::{Add, AddAssign},
        };

        use crate::prelude::{Degrees, Radians};

        impl Add<Radians> for Radians {
            type Output = Radians;
            fn add(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<&Radians> for Radians {
            type Output = Radians;
            fn add(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<f32> for Radians {
            type Output = Radians;
            fn add(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 + rhs)
            }
        }

        impl Add<Degrees> for Radians {
            type Output = Radians;
            fn add(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 + rhs.to_radians().to_f32())
            }
        }

        impl Add<&Degrees> for Radians {
            type Output = Radians;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 + rhs.to_radians().to_f32())
            }
        }

        impl Add<Radians> for &Radians {
            type Output = Radians;
            fn add(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<&Radians> for &Radians {
            type Output = Radians;
            fn add(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<f32> for &Radians {
            type Output = Radians;
            fn add(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 + rhs)
            }
        }

        impl Add<Degrees> for &Radians {
            type Output = Radians;
            fn add(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 + rhs.to_radians().to_f32())
            }
        }

        impl Add<&Degrees> for &Radians {
            type Output = Radians;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 + rhs.to_radians().to_f32())
            }
        }

        impl AddAssign<Radians> for Radians {
            fn add_assign(&mut self, rhs: Radians) {
                *self = (*self + rhs) % TAU;
            }
        }

        impl AddAssign<&Radians> for Radians {
            fn add_assign(&mut self, rhs: &Radians) {
                *self = (*self + rhs) % TAU;
            }
        }

        impl AddAssign<f32> for Radians {
            fn add_assign(&mut self, rhs: f32) {
                *self = (*self + rhs) % TAU;
            }
        }

        impl AddAssign<Degrees> for Radians {
            fn add_assign(&mut self, rhs: Degrees) {
                *self = (*self + rhs.to_radians()) % TAU;
            }
        }

        impl AddAssign<&Degrees> for Radians {
            fn add_assign(&mut self, rhs: &Degrees) {
                *self = (*self + rhs.to_radians()) % TAU;
            }
        }
    }
    mod sub {
        use crate::prelude::{Degrees, Radians};
        use std::{
            f32::consts::TAU,
            ops::{Sub, SubAssign},
        };

        impl Sub<Radians> for Radians {
            type Output = Radians;
            fn sub(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<&Radians> for Radians {
            type Output = Radians;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<f32> for Radians {
            type Output = Radians;
            fn sub(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 - rhs)
            }
        }

        impl Sub<Degrees> for Radians {
            type Output = Radians;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 - rhs.to_radians().to_f32())
            }
        }

        impl Sub<&Degrees> for Radians {
            type Output = Radians;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 - rhs.to_radians().to_f32())
            }
        }

        impl Sub<Radians> for &Radians {
            type Output = Radians;
            fn sub(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<&Radians> for &Radians {
            type Output = Radians;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<f32> for &Radians {
            type Output = Radians;
            fn sub(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 - rhs)
            }
        }

        impl Sub<Degrees> for &Radians {
            type Output = Radians;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 - rhs.to_radians().to_f32())
            }
        }

        impl Sub<&Degrees> for &Radians {
            type Output = Radians;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 - rhs.to_radians().to_f32())
            }
        }

        impl SubAssign<Radians> for Radians {
            fn sub_assign(&mut self, rhs: Radians) {
                *self = (*self - rhs) % TAU;
            }
        }

        impl SubAssign<&Radians> for Radians {
            fn sub_assign(&mut self, rhs: &Radians) {
                *self = (*self - rhs) % TAU;
            }
        }

        impl SubAssign<f32> for Radians {
            fn sub_assign(&mut self, rhs: f32) {
                *self = (*self - rhs) % TAU;
            }
        }

        impl SubAssign<Degrees> for Radians {
            fn sub_assign(&mut self, rhs: Degrees) {
                *self = (*self - rhs.to_radians()) % TAU;
            }
        }

        impl SubAssign<&Degrees> for Radians {
            fn sub_assign(&mut self, rhs: &Degrees) {
                *self = (*self - rhs.to_radians()) % TAU;
            }
        }
    }
    mod mul {
        use std::{
            f32::consts::TAU,
            ops::{Mul, MulAssign},
        };

        use crate::prelude::{Degrees, Radians};

        impl Mul<Radians> for Radians {
            type Output = Radians;
            fn mul(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<&Radians> for Radians {
            type Output = Radians;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<f32> for Radians {
            type Output = Radians;
            fn mul(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 * rhs)
            }
        }

        impl Mul<Degrees> for Radians {
            type Output = Radians;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 * rhs.to_radians().to_f32())
            }
        }

        impl Mul<&Degrees> for Radians {
            type Output = Radians;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 * rhs.to_radians().to_f32())
            }
        }

        impl Mul<Radians> for &Radians {
            type Output = Radians;
            fn mul(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<&Radians> for &Radians {
            type Output = Radians;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<f32> for &Radians {
            type Output = Radians;
            fn mul(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 * rhs)
            }
        }

        impl Mul<Degrees> for &Radians {
            type Output = Radians;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 * rhs.to_radians().to_f32())
            }
        }

        impl Mul<&Degrees> for &Radians {
            type Output = Radians;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 * rhs.to_radians().to_f32())
            }
        }

        impl MulAssign<Radians> for Radians {
            fn mul_assign(&mut self, rhs: Radians) {
                *self = (*self * rhs) % TAU;
            }
        }

        impl MulAssign<&Radians> for Radians {
            fn mul_assign(&mut self, rhs: &Radians) {
                *self = (*self * rhs) % TAU;
            }
        }

        impl MulAssign<f32> for Radians {
            fn mul_assign(&mut self, rhs: f32) {
                *self = (*self * rhs) % TAU;
            }
        }

        impl MulAssign<Degrees> for Radians {
            fn mul_assign(&mut self, rhs: Degrees) {
                *self = (*self * rhs.to_radians()) % TAU;
            }
        }

        impl MulAssign<&Degrees> for Radians {
            fn mul_assign(&mut self, rhs: &Degrees) {
                *self = (*self * rhs.to_radians()) % TAU;
            }
        }
    }
    mod div {
        use std::{
            f32::consts::TAU,
            ops::{Div, DivAssign},
        };

        use crate::prelude::{Degrees, Radians};

        impl Div<Radians> for Radians {
            type Output = Radians;
            fn div(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<&Radians> for Radians {
            type Output = Radians;
            fn div(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<f32> for Radians {
            type Output = Radians;
            fn div(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 / rhs)
            }
        }

        impl Div<Degrees> for Radians {
            type Output = Radians;
            fn div(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 / rhs.to_radians().to_f32())
            }
        }

        impl Div<&Degrees> for Radians {
            type Output = Radians;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 / rhs.to_radians().to_f32())
            }
        }

        impl Div<Radians> for &Radians {
            type Output = Radians;
            fn div(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<&Radians> for &Radians {
            type Output = Radians;
            fn div(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<f32> for &Radians {
            type Output = Radians;
            fn div(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 / rhs)
            }
        }

        impl Div<Degrees> for &Radians {
            type Output = Radians;
            fn div(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 / rhs.to_radians().to_f32())
            }
        }

        impl Div<&Degrees> for &Radians {
            type Output = Radians;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 / rhs.to_radians().to_f32())
            }
        }

        impl DivAssign<Radians> for Radians {
            fn div_assign(&mut self, rhs: Radians) {
                *self = (*self / rhs) % TAU;
            }
        }

        impl DivAssign<&Radians> for Radians {
            fn div_assign(&mut self, rhs: &Radians) {
                *self = (*self / rhs) % TAU;
            }
        }

        impl DivAssign<f32> for Radians {
            fn div_assign(&mut self, rhs: f32) {
                *self = (*self / rhs) % TAU;
            }
        }

        impl DivAssign<Degrees> for Radians {
            fn div_assign(&mut self, rhs: Degrees) {
                *self = (*self / rhs.to_radians()) % TAU;
            }
        }

        impl DivAssign<&Degrees> for Radians {
            fn div_assign(&mut self, rhs: &Degrees) {
                *self = (*self / rhs.to_radians()) % TAU;
            }
        }
    }
    mod rem {
        use std::{
            f32::consts::TAU,
            ops::{Rem, RemAssign},
        };

        use crate::prelude::{Degrees, Radians};

        impl Rem<Radians> for Radians {
            type Output = Radians;
            fn rem(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<&Radians> for Radians {
            type Output = Radians;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<f32> for Radians {
            type Output = Radians;
            fn rem(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 % rhs)
            }
        }

        impl Rem<Degrees> for Radians {
            type Output = Radians;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 % rhs.to_radians().to_f32())
            }
        }

        impl Rem<&Degrees> for Radians {
            type Output = Radians;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 % rhs.to_radians().to_f32())
            }
        }

        impl Rem<Radians> for &Radians {
            type Output = Radians;
            fn rem(self, rhs: Radians) -> Self::Output {
                Radians::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<&Radians> for &Radians {
            type Output = Radians;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Radians::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<f32> for &Radians {
            type Output = Radians;
            fn rem(self, rhs: f32) -> Self::Output {
                Radians::from_f32(self.0 % rhs)
            }
        }

        impl Rem<Degrees> for &Radians {
            type Output = Radians;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Radians::from_f32(self.0 % rhs.to_radians().to_f32())
            }
        }

        impl Rem<&Degrees> for &Radians {
            type Output = Radians;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Radians::from_f32(self.0 % rhs.to_radians().to_f32())
            }
        }

        impl RemAssign<Radians> for Radians {
            fn rem_assign(&mut self, rhs: Radians) {
                *self = (*self % rhs) % TAU;
            }
        }

        impl RemAssign<&Radians> for Radians {
            fn rem_assign(&mut self, rhs: &Radians) {
                *self = (*self % rhs) % TAU;
            }
        }

        impl RemAssign<f32> for Radians {
            fn rem_assign(&mut self, rhs: f32) {
                *self = (*self % rhs) % TAU;
            }
        }

        impl RemAssign<Degrees> for Radians {
            fn rem_assign(&mut self, rhs: Degrees) {
                *self = (*self % rhs.to_radians()) % TAU;
            }
        }

        impl RemAssign<&Degrees> for Radians {
            fn rem_assign(&mut self, rhs: &Degrees) {
                *self = (*self % rhs.to_radians()) % TAU;
            }
        }
    }
    mod neg {
        use crate::prelude::Radians;
        use std::ops::Neg;

        impl Neg for Radians {
            type Output = Radians;
            fn neg(self) -> Self::Output {
                Radians::from_f32(-self.0)
            }
        }

        impl Neg for &Radians {
            type Output = Radians;
            fn neg(self) -> Self::Output {
                Radians::from_f32(-self.0)
            }
        }
    }
    mod f32 {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
        };

        use crate::prelude::Radians;

        impl Add<Radians> for f32 {
            type Output = f32;
            fn add(self, rhs: Radians) -> Self::Output {
                self + rhs.0
            }
        }

        impl Add<&Radians> for f32 {
            type Output = f32;
            fn add(self, rhs: &Radians) -> Self::Output {
                self + rhs.0
            }
        }

        impl AddAssign<Radians> for f32 {
            fn add_assign(&mut self, rhs: Radians) {
                *self += rhs.0
            }
        }

        impl AddAssign<&Radians> for f32 {
            fn add_assign(&mut self, rhs: &Radians) {
                *self += rhs.0
            }
        }

        impl Sub<Radians> for f32 {
            type Output = f32;
            fn sub(self, rhs: Radians) -> Self::Output {
                self - rhs.0
            }
        }

        impl Sub<&Radians> for f32 {
            type Output = f32;
            fn sub(self, rhs: &Radians) -> Self::Output {
                self - rhs.0
            }
        }

        impl SubAssign<Radians> for f32 {
            fn sub_assign(&mut self, rhs: Radians) {
                *self -= rhs.0
            }
        }

        impl SubAssign<&Radians> for f32 {
            fn sub_assign(&mut self, rhs: &Radians) {
                *self -= rhs.0
            }
        }

        impl Mul<Radians> for f32 {
            type Output = f32;
            fn mul(self, rhs: Radians) -> Self::Output {
                self * rhs.0
            }
        }

        impl Mul<&Radians> for f32 {
            type Output = f32;
            fn mul(self, rhs: &Radians) -> Self::Output {
                self * rhs.0
            }
        }

        impl MulAssign<Radians> for f32 {
            fn mul_assign(&mut self, rhs: Radians) {
                *self *= rhs.0
            }
        }

        impl MulAssign<&Radians> for f32 {
            fn mul_assign(&mut self, rhs: &Radians) {
                *self *= rhs.0
            }
        }

        impl Div<Radians> for f32 {
            type Output = f32;
            fn div(self, rhs: Radians) -> Self::Output {
                self / rhs.0
            }
        }

        impl Div<&Radians> for f32 {
            type Output = f32;
            fn div(self, rhs: &Radians) -> Self::Output {
                self / rhs.0
            }
        }

        impl DivAssign<Radians> for f32 {
            fn div_assign(&mut self, rhs: Radians) {
                *self /= rhs.0
            }
        }

        impl DivAssign<&Radians> for f32 {
            fn div_assign(&mut self, rhs: &Radians) {
                *self /= rhs.0
            }
        }

        impl Rem<Radians> for f32 {
            type Output = f32;
            fn rem(self, rhs: Radians) -> Self::Output {
                self % rhs.0
            }
        }

        impl Rem<&Radians> for f32 {
            type Output = f32;
            fn rem(self, rhs: &Radians) -> Self::Output {
                self % rhs.0
            }
        }

        impl RemAssign<Radians> for f32 {
            fn rem_assign(&mut self, rhs: Radians) {
                *self %= rhs.0
            }
        }

        impl RemAssign<&Radians> for f32 {
            fn rem_assign(&mut self, rhs: &Radians) {
                *self %= rhs.0
            }
        }
    }
}
