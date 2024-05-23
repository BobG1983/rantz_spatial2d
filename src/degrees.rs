pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Debug, Deref, DerefMut, Reflect)]
pub struct Degrees(f32);

impl Degrees {
    pub fn new(degrees: f32) -> Self {
        Self::from_f32(degrees)
    }

    pub fn to_radians(&self) -> Radians {
        Radians::from_f32(self.0 * std::f32::consts::PI / 180.0)
    }

    pub fn to_f32(self) -> f32 {
        self.0
    }

    pub fn from_f32(value: f32) -> Self {
        Self(value % 360.)
    }

    pub fn from_radians(radians: &Radians) -> Self {
        Self::from_f32(radians.to_f32())
    }

    pub const ZERO: Self = Self(0.);
    pub fn zero() -> Self {
        Self::ZERO
    }

    pub const UP: Self = Self(90.);
    pub fn up() -> Self {
        Self::UP
    }

    pub const DOWN: Self = Self(270.);
    pub fn down() -> Self {
        Self::DOWN
    }

    pub const LEFT: Self = Self(180.);
    pub fn left() -> Self {
        Self::LEFT
    }

    pub const RIGHT: Self = Self(0.);
    pub fn right() -> Self {
        Self::RIGHT
    }
}

mod conversions {
    use super::{Degrees, Radians};
    use bevy::math::{EulerRot, Quat};

    impl From<Radians> for Degrees {
        fn from(radians: Radians) -> Self {
            Self::from_radians(&radians)
        }
    }

    impl From<&Radians> for Degrees {
        fn from(radians: &Radians) -> Self {
            Self::from_radians(radians)
        }
    }

    impl From<f32> for Degrees {
        fn from(value: f32) -> Self {
            Self::from_f32(value)
        }
    }

    impl From<Degrees> for f32 {
        fn from(degrees: Degrees) -> Self {
            degrees.0
        }
    }

    impl From<&Degrees> for f32 {
        fn from(degrees: &Degrees) -> Self {
            degrees.0
        }
    }

    impl From<Degrees> for Quat {
        fn from(value: Degrees) -> Self {
            value.to_radians().into()
        }
    }

    impl From<&Degrees> for Quat {
        fn from(value: &Degrees) -> Self {
            value.to_radians().into()
        }
    }

    impl From<Quat> for Degrees {
        fn from(value: Quat) -> Self {
            Self::from_radians(&value.to_euler(EulerRot::XYZ).2.into())
        }
    }

    impl From<&Quat> for Degrees {
        fn from(value: &Quat) -> Self {
            Self::from_radians(&value.to_euler(EulerRot::XYZ).2.into())
        }
    }
}

mod operators {
    mod add {
        use crate::prelude::{Degrees, Radians};
        use std::ops::{Add, AddAssign};

        impl Add<f32> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs)
            }
        }

        impl Add<Degrees> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_f32())
            }
        }

        impl Add<&Degrees> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_f32())
            }
        }

        impl Add<Radians> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees().to_f32())
            }
        }

        impl Add<&Radians> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees().to_f32())
            }
        }

        impl Add<f32> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs)
            }
        }

        impl Add<Degrees> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_f32())
            }
        }

        impl Add<&Degrees> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_f32())
            }
        }

        impl Add<Radians> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees().to_f32())
            }
        }

        impl Add<&Radians> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees().to_f32())
            }
        }

        impl AddAssign<f32> for Degrees {
            fn add_assign(&mut self, rhs: f32) {
                *self = (*self + rhs) % 360.0;
            }
        }

        impl AddAssign<Degrees> for Degrees {
            fn add_assign(&mut self, rhs: Degrees) {
                *self = (*self + rhs) % 360.0;
            }
        }

        impl AddAssign<&Degrees> for Degrees {
            fn add_assign(&mut self, rhs: &Degrees) {
                *self = (*self + rhs) % 360.0;
            }
        }

        impl AddAssign<Radians> for Degrees {
            fn add_assign(&mut self, rhs: Radians) {
                *self = (*self + rhs.to_degrees()) % 360.0;
            }
        }

        impl AddAssign<&Radians> for Degrees {
            fn add_assign(&mut self, rhs: &Radians) {
                *self = (*self + rhs.to_degrees()) % 360.0;
            }
        }
    }
    mod sub {
        use crate::prelude::{Degrees, Radians};
        use std::ops::{Sub, SubAssign};

        impl Sub<f32> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs)
            }
        }

        impl Sub<Degrees> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_f32())
            }
        }

        impl Sub<&Degrees> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_f32())
            }
        }

        impl Sub<Radians> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees().to_f32())
            }
        }

        impl Sub<&Radians> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees().to_f32())
            }
        }

        impl Sub<f32> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs)
            }
        }

        impl Sub<Degrees> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_f32())
            }
        }

        impl Sub<&Degrees> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_f32())
            }
        }

        impl Sub<Radians> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees().to_f32())
            }
        }

        impl Sub<&Radians> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees().to_f32())
            }
        }

        impl SubAssign<f32> for Degrees {
            fn sub_assign(&mut self, rhs: f32) {
                *self = (*self - rhs) % 360.0;
            }
        }

        impl SubAssign<Degrees> for Degrees {
            fn sub_assign(&mut self, rhs: Degrees) {
                *self = (*self - rhs) % 360.0;
            }
        }

        impl SubAssign<&Degrees> for Degrees {
            fn sub_assign(&mut self, rhs: &Degrees) {
                *self = (*self - rhs) % 360.0;
            }
        }

        impl SubAssign<Radians> for Degrees {
            fn sub_assign(&mut self, rhs: Radians) {
                *self = (*self - rhs.to_degrees()) % 360.0;
            }
        }

        impl SubAssign<&Radians> for Degrees {
            fn sub_assign(&mut self, rhs: &Radians) {
                *self = (*self - rhs.to_degrees()) % 360.0;
            }
        }
    }
    mod mul {
        use crate::prelude::{Degrees, Radians};
        use std::ops::{Mul, MulAssign};

        impl Mul<f32> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs)
            }
        }

        impl Mul<Degrees> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_f32())
            }
        }

        impl Mul<&Degrees> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_f32())
            }
        }

        impl Mul<Radians> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees().to_f32())
            }
        }

        impl Mul<&Radians> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees().to_f32())
            }
        }

        impl Mul<f32> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs)
            }
        }

        impl Mul<Degrees> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_f32())
            }
        }

        impl Mul<&Degrees> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_f32())
            }
        }

        impl Mul<Radians> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees().to_f32())
            }
        }

        impl Mul<&Radians> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees().to_f32())
            }
        }
        impl MulAssign<f32> for Degrees {
            fn mul_assign(&mut self, rhs: f32) {
                *self = (*self * rhs) % 360.0;
            }
        }

        impl MulAssign<Degrees> for Degrees {
            fn mul_assign(&mut self, rhs: Degrees) {
                *self = (*self * rhs) % 360.0;
            }
        }

        impl MulAssign<&Degrees> for Degrees {
            fn mul_assign(&mut self, rhs: &Degrees) {
                *self = (*self * rhs) % 360.0;
            }
        }

        impl MulAssign<Radians> for Degrees {
            fn mul_assign(&mut self, rhs: Radians) {
                *self = (*self * rhs.to_degrees()) % 360.0;
            }
        }

        impl MulAssign<&Radians> for Degrees {
            fn mul_assign(&mut self, rhs: &Radians) {
                *self = (*self * rhs.to_degrees()) % 360.0;
            }
        }
    }
    mod div {
        use crate::prelude::{Degrees, Radians};
        use std::ops::{Div, DivAssign};

        impl Div<f32> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs)
            }
        }

        impl Div<Degrees> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_f32())
            }
        }

        impl Div<&Degrees> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_f32())
            }
        }

        impl Div<Radians> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees().to_f32())
            }
        }

        impl Div<&Radians> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees().to_f32())
            }
        }

        impl Div<f32> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs)
            }
        }

        impl Div<Degrees> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_f32())
            }
        }

        impl Div<&Degrees> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_f32())
            }
        }

        impl Div<Radians> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees().to_f32())
            }
        }

        impl Div<&Radians> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees().to_f32())
            }
        }

        impl DivAssign<f32> for Degrees {
            fn div_assign(&mut self, rhs: f32) {
                *self = (*self / rhs) % 360.0;
            }
        }

        impl DivAssign<Degrees> for Degrees {
            fn div_assign(&mut self, rhs: Degrees) {
                *self = (*self / rhs) % 360.0;
            }
        }

        impl DivAssign<&Degrees> for Degrees {
            fn div_assign(&mut self, rhs: &Degrees) {
                *self = (*self / rhs) % 360.0;
            }
        }

        impl DivAssign<Radians> for Degrees {
            fn div_assign(&mut self, rhs: Radians) {
                *self = (*self / rhs.to_degrees()) % 360.0;
            }
        }

        impl DivAssign<&Radians> for Degrees {
            fn div_assign(&mut self, rhs: &Radians) {
                *self = (*self / rhs.to_degrees()) % 360.0;
            }
        }
    }
    mod rem {
        use crate::prelude::{Degrees, Radians};
        use std::ops::{Rem, RemAssign};

        impl Rem<f32> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs)
            }
        }

        impl Rem<Degrees> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_f32())
            }
        }

        impl Rem<&Degrees> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_f32())
            }
        }

        impl Rem<Radians> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees().to_f32())
            }
        }

        impl Rem<&Radians> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees().to_f32())
            }
        }

        impl Rem<f32> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: f32) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs)
            }
        }

        impl Rem<Degrees> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_f32())
            }
        }

        impl Rem<&Degrees> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_f32())
            }
        }

        impl Rem<Radians> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees().to_f32())
            }
        }

        impl Rem<&Radians> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees().to_f32())
            }
        }

        impl RemAssign<f32> for Degrees {
            fn rem_assign(&mut self, rhs: f32) {
                *self = (*self % rhs) % 360.0;
            }
        }

        impl RemAssign<Degrees> for Degrees {
            fn rem_assign(&mut self, rhs: Degrees) {
                *self = (*self % rhs) % 360.0;
            }
        }

        impl RemAssign<&Degrees> for Degrees {
            fn rem_assign(&mut self, rhs: &Degrees) {
                *self = (*self % rhs) % 360.0;
            }
        }

        impl RemAssign<Radians> for Degrees {
            fn rem_assign(&mut self, rhs: Radians) {
                *self = (*self % rhs) % 360.0;
            }
        }

        impl RemAssign<&Radians> for Degrees {
            fn rem_assign(&mut self, rhs: &Radians) {
                *self = (*self % rhs) % 360.0;
            }
        }
    }
    mod neg {
        use crate::prelude::Degrees;
        use std::ops::Neg;

        impl Neg for Degrees {
            type Output = Degrees;
            fn neg(self) -> Self::Output {
                Degrees::from_f32(-self.0)
            }
        }

        impl Neg for &Degrees {
            type Output = Degrees;
            fn neg(self) -> Self::Output {
                Degrees::from_f32(-self.0)
            }
        }
    }
    mod f32 {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
        };

        use crate::prelude::Degrees;

        impl Add<Degrees> for f32 {
            type Output = f32;
            fn add(self, rhs: Degrees) -> Self::Output {
                self + rhs.0
            }
        }

        impl Add<&Degrees> for f32 {
            type Output = f32;
            fn add(self, rhs: &Degrees) -> Self::Output {
                self + rhs.0
            }
        }

        impl AddAssign<Degrees> for f32 {
            fn add_assign(&mut self, rhs: Degrees) {
                *self += rhs.0
            }
        }

        impl AddAssign<&Degrees> for f32 {
            fn add_assign(&mut self, rhs: &Degrees) {
                *self += rhs.0
            }
        }

        impl Sub<Degrees> for f32 {
            type Output = f32;
            fn sub(self, rhs: Degrees) -> Self::Output {
                self - rhs.0
            }
        }

        impl Sub<&Degrees> for f32 {
            type Output = f32;
            fn sub(self, rhs: &Degrees) -> Self::Output {
                self - rhs.0
            }
        }

        impl SubAssign<Degrees> for f32 {
            fn sub_assign(&mut self, rhs: Degrees) {
                *self -= rhs.0
            }
        }

        impl SubAssign<&Degrees> for f32 {
            fn sub_assign(&mut self, rhs: &Degrees) {
                *self -= rhs.0
            }
        }

        impl Mul<Degrees> for f32 {
            type Output = f32;
            fn mul(self, rhs: Degrees) -> Self::Output {
                self * rhs.0
            }
        }

        impl Mul<&Degrees> for f32 {
            type Output = f32;
            fn mul(self, rhs: &Degrees) -> Self::Output {
                self * rhs.0
            }
        }

        impl MulAssign<Degrees> for f32 {
            fn mul_assign(&mut self, rhs: Degrees) {
                *self *= rhs.0
            }
        }

        impl MulAssign<&Degrees> for f32 {
            fn mul_assign(&mut self, rhs: &Degrees) {
                *self *= rhs.0
            }
        }

        impl Div<Degrees> for f32 {
            type Output = f32;
            fn div(self, rhs: Degrees) -> Self::Output {
                self / rhs.0
            }
        }

        impl Div<&Degrees> for f32 {
            type Output = f32;
            fn div(self, rhs: &Degrees) -> Self::Output {
                self / rhs.0
            }
        }

        impl DivAssign<Degrees> for f32 {
            fn div_assign(&mut self, rhs: Degrees) {
                *self /= rhs.0
            }
        }

        impl DivAssign<&Degrees> for f32 {
            fn div_assign(&mut self, rhs: &Degrees) {
                *self /= rhs.0
            }
        }

        impl Rem<Degrees> for f32 {
            type Output = f32;
            fn rem(self, rhs: Degrees) -> Self::Output {
                self % rhs.0
            }
        }

        impl Rem<&Degrees> for f32 {
            type Output = f32;
            fn rem(self, rhs: &Degrees) -> Self::Output {
                self % rhs.0
            }
        }

        impl RemAssign<Degrees> for f32 {
            fn rem_assign(&mut self, rhs: Degrees) {
                *self %= rhs.0
            }
        }

        impl RemAssign<&Degrees> for f32 {
            fn rem_assign(&mut self, rhs: &Degrees) {
                *self %= rhs.0
            }
        }
    }
}
