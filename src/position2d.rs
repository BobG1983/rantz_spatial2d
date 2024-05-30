use crate::prelude::{Degrees, Radians};

#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PositionPropogation {
    #[default]
    Relative,
    Absolute,
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self::from_f32(x, y)
    }

    pub fn from_f32(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub fn zero() -> Self {
        Self::ZERO
    }

    pub const UP: Self = Self { x: 0., y: 1. };
    pub fn up() -> Self {
        Self::UP
    }

    pub const DOWN: Self = Self { x: 0., y: -1. };
    pub fn down() -> Self {
        Self::DOWN
    }

    pub const LEFT: Self = Self { x: -1., y: 0. };
    pub fn left() -> Self {
        Self::LEFT
    }

    pub const RIGHT: Self = Self { x: 0., y: 0. };
    pub fn right() -> Self {
        Self::RIGHT
    }

    pub fn rotate_degrees(self, angle: Degrees) -> Self {
        let x = self.x * angle.to_radians_f32().cos() - self.y * angle.to_radians_f32().sin();
        let y = self.x * angle.to_radians_f32().sin() + self.y * angle.to_radians_f32().cos();
        Self::from_f32(x, y)
    }

    pub fn rotate_radians(self, angle: Radians) -> Self {
        let x = self.x * angle.to_f32().cos() - self.y * angle.to_f32().sin();
        let y = self.x * angle.to_f32().sin() + self.y * angle.to_f32().cos();
        Self::from_f32(x, y)
    }
}

#[cfg(feature = "bevy")]
mod from {
    use super::Position2D;
    #[cfg(feature = "bevy")]
    use bevy::math::Vec2;

    #[cfg(feature = "bevy")]
    impl From<Vec2> for Position2D {
        fn from(value: Vec2) -> Self {
            Self::from_f32(value.x, value.y)
        }
    }

    #[cfg(feature = "bevy")]
    impl From<&Vec2> for Position2D {
        fn from(value: &Vec2) -> Self {
            Self::from_f32(value.x, value.y)
        }
    }
}

#[cfg(feature = "bevy")]
mod into {
    use super::Position2D;
    #[cfg(feature = "bevy")]
    use bevy::math::Vec2;

    #[cfg(feature = "bevy")]
    impl From<Position2D> for Vec2 {
        fn from(value: Position2D) -> Self {
            Self::new(value.x, value.y)
        }
    }

    #[cfg(feature = "bevy")]
    impl From<&Position2D> for Vec2 {
        fn from(value: &Position2D) -> Self {
            Self::new(value.x, value.y)
        }
    }
}

mod operations {
    mod add {
        use crate::prelude::Position2D;
        #[cfg(feature = "bevy")]
        use bevy::math::Vec2;
        use std::ops::{Add, AddAssign};

        impl Add<Position2D> for Position2D {
            type Output = Position2D;
            fn add(self, rhs: Self) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Position2D> for Position2D {
            type Output = Position2D;
            fn add(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<Position2D> for &Position2D {
            type Output = Position2D;
            fn add(self, rhs: Position2D) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Position2D> for &Position2D {
            type Output = Position2D;
            fn add(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<f32> for Position2D {
            type Output = Position2D;
            fn add(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x + rhs, self.y + rhs)
            }
        }

        impl Add<f32> for &Position2D {
            type Output = Position2D;
            fn add(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x + rhs, self.y + rhs)
            }
        }

        #[cfg(feature = "bevy")]
        impl Add<Vec2> for Position2D {
            type Output = Position2D;
            fn add(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Add<&Vec2> for Position2D {
            type Output = Position2D;
            fn add(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Add<Vec2> for &Position2D {
            type Output = Position2D;
            fn add(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Add<&Vec2> for &Position2D {
            type Output = Position2D;
            fn add(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x + rhs.x, self.y + rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl AddAssign<Vec2> for Position2D {
            fn add_assign(&mut self, rhs: Vec2) {
                *self = *self + rhs;
            }
        }

        #[cfg(feature = "bevy")]
        impl AddAssign<&Vec2> for Position2D {
            fn add_assign(&mut self, rhs: &Vec2) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<Position2D> for Position2D {
            fn add_assign(&mut self, rhs: Position2D) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Position2D> for Position2D {
            fn add_assign(&mut self, rhs: &Position2D) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<f32> for Position2D {
            fn add_assign(&mut self, rhs: f32) {
                *self = *self + rhs;
            }
        }
    }
    mod sub {
        use crate::prelude::Position2D;
        #[cfg(feature = "bevy")]
        use bevy::math::Vec2;
        use std::ops::{Sub, SubAssign};

        impl Sub<Position2D> for Position2D {
            type Output = Position2D;
            fn sub(self, rhs: Self) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Position2D> for Position2D {
            type Output = Position2D;
            fn sub(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<Position2D> for &Position2D {
            type Output = Position2D;
            fn sub(self, rhs: Position2D) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Position2D> for &Position2D {
            type Output = Position2D;
            fn sub(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<f32> for Position2D {
            type Output = Position2D;
            fn sub(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x - rhs, self.y - rhs)
            }
        }

        impl Sub<f32> for &Position2D {
            type Output = Position2D;
            fn sub(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x - rhs, self.y - rhs)
            }
        }

        #[cfg(feature = "bevy")]
        impl Sub<Vec2> for Position2D {
            type Output = Position2D;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Sub<&Vec2> for Position2D {
            type Output = Position2D;
            fn sub(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Sub<Vec2> for &Position2D {
            #[cfg(feature = "bevy")]
            type Output = Position2D;
            fn sub(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Sub<&Vec2> for &Position2D {
            type Output = Position2D;
            fn sub(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x - rhs.x, self.y - rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl SubAssign<Vec2> for Position2D {
            fn sub_assign(&mut self, rhs: Vec2) {
                *self = *self - rhs;
            }
        }

        #[cfg(feature = "bevy")]
        impl SubAssign<&Vec2> for Position2D {
            fn sub_assign(&mut self, rhs: &Vec2) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<Position2D> for Position2D {
            fn sub_assign(&mut self, rhs: Position2D) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Position2D> for Position2D {
            fn sub_assign(&mut self, rhs: &Position2D) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<f32> for Position2D {
            fn sub_assign(&mut self, rhs: f32) {
                *self = *self - rhs;
            }
        }
    }
    mod mul {
        use crate::prelude::{Position2D, Scale2D};
        #[cfg(feature = "bevy")]
        use bevy::math::Vec2;
        use std::ops::{Mul, MulAssign};

        impl Mul<Position2D> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Self) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Position2D> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Position2D> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Position2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Position2D> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<f32> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        impl Mul<f32> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x * rhs, self.y * rhs)
            }
        }

        #[cfg(feature = "bevy")]
        impl Mul<Vec2> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Mul<&Vec2> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Mul<Vec2> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Mul<&Vec2> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Scale2D> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Scale2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Scale2D> for Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Scale2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Scale2D> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: Scale2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Scale2D> for &Position2D {
            type Output = Position2D;
            fn mul(self, rhs: &Scale2D) -> Self::Output {
                Position2D::from_f32(self.x * rhs.x, self.y * rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl MulAssign<Vec2> for Position2D {
            fn mul_assign(&mut self, rhs: Vec2) {
                *self = *self * rhs;
            }
        }

        #[cfg(feature = "bevy")]
        impl MulAssign<&Vec2> for Position2D {
            fn mul_assign(&mut self, rhs: &Vec2) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<Position2D> for Position2D {
            fn mul_assign(&mut self, rhs: Position2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Position2D> for Position2D {
            fn mul_assign(&mut self, rhs: &Position2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<f32> for Position2D {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<Scale2D> for Position2D {
            fn mul_assign(&mut self, rhs: Scale2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Scale2D> for Position2D {
            fn mul_assign(&mut self, rhs: &Scale2D) {
                *self = *self * rhs;
            }
        }
    }
    mod div {
        use crate::prelude::Position2D;
        #[cfg(feature = "bevy")]
        use bevy::math::Vec2;
        use std::ops::{Div, DivAssign};

        impl Div<Position2D> for Position2D {
            type Output = Position2D;
            fn div(self, rhs: Self) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Position2D> for Position2D {
            type Output = Position2D;
            fn div(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<Position2D> for &Position2D {
            type Output = Position2D;
            fn div(self, rhs: Position2D) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Position2D> for &Position2D {
            type Output = Position2D;
            fn div(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<f32> for Position2D {
            type Output = Position2D;
            fn div(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x / rhs, self.y / rhs)
            }
        }

        impl Div<f32> for &Position2D {
            type Output = Position2D;
            fn div(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x / rhs, self.y / rhs)
            }
        }

        #[cfg(feature = "bevy")]
        impl Div<Vec2> for Position2D {
            type Output = Position2D;
            fn div(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Div<&Vec2> for Position2D {
            type Output = Position2D;
            fn div(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Div<Vec2> for &Position2D {
            type Output = Position2D;
            fn div(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Div<&Vec2> for &Position2D {
            type Output = Position2D;
            fn div(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x / rhs.x, self.y / rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl DivAssign<Vec2> for Position2D {
            fn div_assign(&mut self, rhs: Vec2) {
                *self = *self / rhs;
            }
        }

        #[cfg(feature = "bevy")]
        impl DivAssign<&Vec2> for Position2D {
            fn div_assign(&mut self, rhs: &Vec2) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<Position2D> for Position2D {
            fn div_assign(&mut self, rhs: Position2D) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Position2D> for Position2D {
            fn div_assign(&mut self, rhs: &Position2D) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<f32> for Position2D {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    }
    mod rem {
        use crate::prelude::Position2D;
        #[cfg(feature = "bevy")]
        use bevy::math::Vec2;
        use std::ops::{Rem, RemAssign};

        impl Rem<Position2D> for Position2D {
            type Output = Position2D;
            fn rem(self, rhs: Self) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl Rem<&Position2D> for Position2D {
            type Output = Position2D;
            fn rem(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl Rem<Position2D> for &Position2D {
            type Output = Position2D;
            fn rem(self, rhs: Position2D) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl Rem<&Position2D> for &Position2D {
            type Output = Position2D;
            fn rem(self, rhs: &Position2D) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl Rem<f32> for Position2D {
            type Output = Position2D;
            fn rem(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x % rhs, self.y % rhs)
            }
        }

        impl Rem<f32> for &Position2D {
            type Output = Position2D;
            fn rem(self, rhs: f32) -> Self::Output {
                Position2D::from_f32(self.x % rhs, self.y % rhs)
            }
        }

        #[cfg(feature = "bevy")]
        impl Rem<Vec2> for Position2D {
            type Output = Position2D;
            fn rem(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Rem<&Vec2> for Position2D {
            type Output = Position2D;
            fn rem(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Rem<Vec2> for &Position2D {
            type Output = Position2D;
            fn rem(self, rhs: Vec2) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl Rem<&Vec2> for &Position2D {
            type Output = Position2D;
            fn rem(self, rhs: &Vec2) -> Self::Output {
                Position2D::from_f32(self.x % rhs.x, self.y % rhs.y)
            }
        }

        #[cfg(feature = "bevy")]
        impl RemAssign<Vec2> for Position2D {
            fn rem_assign(&mut self, rhs: Vec2) {
                *self = *self % rhs;
            }
        }

        #[cfg(feature = "bevy")]
        impl RemAssign<&Vec2> for Position2D {
            fn rem_assign(&mut self, rhs: &Vec2) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<Position2D> for Position2D {
            fn rem_assign(&mut self, rhs: Position2D) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Position2D> for Position2D {
            fn rem_assign(&mut self, rhs: &Position2D) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<f32> for Position2D {
            fn rem_assign(&mut self, rhs: f32) {
                *self = *self % rhs;
            }
        }
    }
    mod neg {
        use std::ops::Neg;

        use crate::prelude::Position2D;

        impl Neg for Position2D {
            type Output = Position2D;
            fn neg(self) -> Self::Output {
                Position2D::from_f32(-self.x, -self.y)
            }
        }

        impl Neg for &Position2D {
            type Output = Position2D;
            fn neg(self) -> Self::Output {
                Position2D::from_f32(-self.x, -self.y)
            }
        }
    }
    #[cfg(feature = "bevy")]
    mod vec2 {
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
        };

        use crate::prelude::Position2D;
        use bevy::math::Vec2;

        impl Add<Position2D> for Vec2 {
            type Output = Vec2;
            fn add(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Position2D> for Vec2 {
            type Output = Vec2;
            fn add(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<Position2D> for &Vec2 {
            type Output = Vec2;
            fn add(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<&Position2D> for &Vec2 {
            type Output = Vec2;
            fn add(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl AddAssign<Position2D> for Vec2 {
            fn add_assign(&mut self, rhs: Position2D) {
                *self = *self + rhs;
            }
        }

        impl AddAssign<&Position2D> for Vec2 {
            fn add_assign(&mut self, rhs: &Position2D) {
                *self = *self + rhs;
            }
        }

        impl Sub<Position2D> for Vec2 {
            type Output = Vec2;
            fn sub(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Position2D> for Vec2 {
            type Output = Vec2;
            fn sub(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<Position2D> for &Vec2 {
            type Output = Vec2;
            fn sub(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl Sub<&Position2D> for &Vec2 {
            type Output = Vec2;
            fn sub(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x - rhs.x, self.y - rhs.y)
            }
        }

        impl SubAssign<Position2D> for Vec2 {
            fn sub_assign(&mut self, rhs: Position2D) {
                *self = *self - rhs;
            }
        }

        impl SubAssign<&Position2D> for Vec2 {
            fn sub_assign(&mut self, rhs: &Position2D) {
                *self = *self - rhs;
            }
        }

        impl Mul<Position2D> for Vec2 {
            type Output = Vec2;
            fn mul(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Position2D> for Vec2 {
            type Output = Vec2;
            fn mul(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<Position2D> for &Vec2 {
            type Output = Vec2;
            fn mul(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl Mul<&Position2D> for &Vec2 {
            type Output = Vec2;
            fn mul(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x * rhs.x, self.y * rhs.y)
            }
        }

        impl MulAssign<Position2D> for Vec2 {
            fn mul_assign(&mut self, rhs: Position2D) {
                *self = *self * rhs;
            }
        }

        impl MulAssign<&Position2D> for Vec2 {
            fn mul_assign(&mut self, rhs: &Position2D) {
                *self = *self * rhs;
            }
        }

        impl Div<Position2D> for Vec2 {
            type Output = Vec2;
            fn div(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Position2D> for Vec2 {
            type Output = Vec2;
            fn div(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<Position2D> for &Vec2 {
            type Output = Vec2;
            fn div(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl Div<&Position2D> for &Vec2 {
            type Output = Vec2;
            fn div(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x / rhs.x, self.y / rhs.y)
            }
        }

        impl DivAssign<Position2D> for Vec2 {
            fn div_assign(&mut self, rhs: Position2D) {
                *self = *self / rhs;
            }
        }

        impl DivAssign<&Position2D> for Vec2 {
            fn div_assign(&mut self, rhs: &Position2D) {
                *self = *self / rhs;
            }
        }

        impl Rem<Position2D> for Vec2 {
            type Output = Vec2;
            fn rem(self, rhs: Position2D) -> Self::Output {
                Vec2::new(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl Rem<&Position2D> for Vec2 {
            type Output = Vec2;
            fn rem(self, rhs: &Position2D) -> Self::Output {
                Vec2::new(self.x % rhs.x, self.y % rhs.y)
            }
        }

        impl RemAssign<Position2D> for Vec2 {
            fn rem_assign(&mut self, rhs: Position2D) {
                *self = *self % rhs;
            }
        }

        impl RemAssign<&Position2D> for Vec2 {
            fn rem_assign(&mut self, rhs: &Position2D) {
                *self = *self % rhs;
            }
        }
    }
}
