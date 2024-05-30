#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrawOrder(f32);

impl DrawOrder {
    pub fn new(order: f32) -> Self {
        Self(order)
    }

    pub fn from_f32(order: f32) -> Self {
        Self::new(order)
    }
}

// TODO: Split and genericize
mod conversions {
    use super::DrawOrder;

    impl From<f32> for DrawOrder {
        fn from(order: f32) -> Self {
            Self::from_f32(order)
        }
    }

    impl From<DrawOrder> for f32 {
        fn from(order: DrawOrder) -> Self {
            order.0
        }
    }

    impl From<&DrawOrder> for f32 {
        fn from(order: &DrawOrder) -> Self {
            order.0
        }
    }
}

mod operators {
    mod add {
        use crate::prelude::DrawOrder;
        use std::ops::{Add, AddAssign};

        impl Add<f32> for DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs)
            }
        }

        impl Add<DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<&DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<f32> for &DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs)
            }
        }

        impl Add<DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs.0)
            }
        }

        impl Add<&DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn add(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 + rhs.0)
            }
        }

        impl AddAssign<f32> for DrawOrder {
            fn add_assign(&mut self, rhs: f32) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<DrawOrder> for DrawOrder {
            fn add_assign(&mut self, rhs: DrawOrder) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&DrawOrder> for DrawOrder {
            fn add_assign(&mut self, rhs: &DrawOrder) {
                *self = *self + rhs;
            }
        }
    }
    mod sub {
        use crate::prelude::DrawOrder;
        use std::ops::{Sub, SubAssign};

        impl Sub<f32> for DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs)
            }
        }

        impl Sub<DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<&DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<f32> for &DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs)
            }
        }

        impl Sub<DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs.0)
            }
        }

        impl Sub<&DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn sub(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 - rhs.0)
            }
        }

        impl SubAssign<f32> for DrawOrder {
            fn sub_assign(&mut self, rhs: f32) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<DrawOrder> for DrawOrder {
            fn sub_assign(&mut self, rhs: DrawOrder) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&DrawOrder> for DrawOrder {
            fn sub_assign(&mut self, rhs: &DrawOrder) {
                *self = *self - rhs;
            }
        }
    }
    mod mul {
        use crate::prelude::DrawOrder;
        use std::ops::{Mul, MulAssign};

        impl Mul<f32> for DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs)
            }
        }

        impl Mul<DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<&DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<f32> for &DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs)
            }
        }

        impl Mul<DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs.0)
            }
        }

        impl Mul<&DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn mul(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 * rhs.0)
            }
        }

        impl MulAssign<f32> for DrawOrder {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<DrawOrder> for DrawOrder {
            fn mul_assign(&mut self, rhs: DrawOrder) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&DrawOrder> for DrawOrder {
            fn mul_assign(&mut self, rhs: &DrawOrder) {
                *self = *self * rhs;
            }
        }
    }
    mod div {
        use crate::prelude::DrawOrder;
        use std::ops::{Div, DivAssign};

        impl Div<f32> for DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs)
            }
        }

        impl Div<DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<&DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<f32> for &DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs)
            }
        }

        impl Div<DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs.0)
            }
        }

        impl Div<&DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn div(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 / rhs.0)
            }
        }

        impl DivAssign<f32> for DrawOrder {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<DrawOrder> for DrawOrder {
            fn div_assign(&mut self, rhs: DrawOrder) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&DrawOrder> for DrawOrder {
            fn div_assign(&mut self, rhs: &DrawOrder) {
                *self = *self / rhs;
            }
        }
    }
    mod rem {
        use crate::prelude::DrawOrder;
        use std::ops::{Rem, RemAssign};

        impl Rem<f32> for DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs)
            }
        }

        impl Rem<DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<&DrawOrder> for DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<f32> for &DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: f32) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs)
            }
        }

        impl Rem<DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs.0)
            }
        }

        impl Rem<&DrawOrder> for &DrawOrder {
            type Output = DrawOrder;
            fn rem(self, rhs: &DrawOrder) -> Self::Output {
                DrawOrder::from_f32(self.0 % rhs.0)
            }
        }

        impl RemAssign<f32> for DrawOrder {
            fn rem_assign(&mut self, rhs: f32) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<DrawOrder> for DrawOrder {
            fn rem_assign(&mut self, rhs: DrawOrder) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&DrawOrder> for DrawOrder {
            fn rem_assign(&mut self, rhs: &DrawOrder) {
                *self = *self % rhs;
            }
        }
    }
    mod neg {
        use crate::prelude::DrawOrder;
        use std::ops::Neg;

        impl Neg for DrawOrder {
            type Output = DrawOrder;
            fn neg(self) -> Self::Output {
                DrawOrder::from_f32(-self.0)
            }
        }

        impl Neg for &DrawOrder {
            type Output = DrawOrder;
            fn neg(self) -> Self::Output {
                DrawOrder::from_f32(-self.0)
            }
        }
    }
    mod f32 {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
        };

        use crate::prelude::DrawOrder;

        impl Add<DrawOrder> for f32 {
            type Output = f32;
            fn add(self, rhs: DrawOrder) -> Self::Output {
                self + rhs.0
            }
        }

        impl Add<&DrawOrder> for f32 {
            type Output = f32;
            fn add(self, rhs: &DrawOrder) -> Self::Output {
                self + rhs.0
            }
        }

        impl AddAssign<DrawOrder> for f32 {
            fn add_assign(&mut self, rhs: DrawOrder) {
                *self += rhs.0
            }
        }

        impl AddAssign<&DrawOrder> for f32 {
            fn add_assign(&mut self, rhs: &DrawOrder) {
                *self += rhs.0
            }
        }

        impl Sub<DrawOrder> for f32 {
            type Output = f32;
            fn sub(self, rhs: DrawOrder) -> Self::Output {
                self - rhs.0
            }
        }

        impl Sub<&DrawOrder> for f32 {
            type Output = f32;
            fn sub(self, rhs: &DrawOrder) -> Self::Output {
                self - rhs.0
            }
        }

        impl SubAssign<DrawOrder> for f32 {
            fn sub_assign(&mut self, rhs: DrawOrder) {
                *self -= rhs.0
            }
        }

        impl SubAssign<&DrawOrder> for f32 {
            fn sub_assign(&mut self, rhs: &DrawOrder) {
                *self -= rhs.0
            }
        }

        impl Mul<DrawOrder> for f32 {
            type Output = f32;
            fn mul(self, rhs: DrawOrder) -> Self::Output {
                self * rhs.0
            }
        }

        impl Mul<&DrawOrder> for f32 {
            type Output = f32;
            fn mul(self, rhs: &DrawOrder) -> Self::Output {
                self * rhs.0
            }
        }

        impl MulAssign<DrawOrder> for f32 {
            fn mul_assign(&mut self, rhs: DrawOrder) {
                *self *= rhs.0
            }
        }

        impl MulAssign<&DrawOrder> for f32 {
            fn mul_assign(&mut self, rhs: &DrawOrder) {
                *self *= rhs.0
            }
        }

        impl Div<DrawOrder> for f32 {
            type Output = f32;
            fn div(self, rhs: DrawOrder) -> Self::Output {
                self / rhs.0
            }
        }

        impl Div<&DrawOrder> for f32 {
            type Output = f32;
            fn div(self, rhs: &DrawOrder) -> Self::Output {
                self / rhs.0
            }
        }

        impl DivAssign<DrawOrder> for f32 {
            fn div_assign(&mut self, rhs: DrawOrder) {
                *self /= rhs.0
            }
        }

        impl DivAssign<&DrawOrder> for f32 {
            fn div_assign(&mut self, rhs: &DrawOrder) {
                *self /= rhs.0
            }
        }

        impl Rem<DrawOrder> for f32 {
            type Output = f32;
            fn rem(self, rhs: DrawOrder) -> Self::Output {
                self % rhs.0
            }
        }

        impl Rem<&DrawOrder> for f32 {
            type Output = f32;
            fn rem(self, rhs: &DrawOrder) -> Self::Output {
                self % rhs.0
            }
        }

        impl RemAssign<DrawOrder> for f32 {
            fn rem_assign(&mut self, rhs: DrawOrder) {
                *self %= rhs.0
            }
        }

        impl RemAssign<&DrawOrder> for f32 {
            fn rem_assign(&mut self, rhs: &DrawOrder) {
                *self %= rhs.0
            }
        }
    }
}
