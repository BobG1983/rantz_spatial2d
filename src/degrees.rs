pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Debug, Deref, DerefMut, Reflect)]
pub struct Degrees(f32);

impl Degrees {
    pub fn new(degrees: f32) -> Self {
        Self::from_f32(degrees)
    }

    pub fn to_f32(self) -> f32 {
        self.0
    }

    pub fn from_f32(angle: f32) -> Self {
        Self(angle)
    }

    pub const ZERO: Self = Self(0.);
    pub const UP: Self = Self(90.);
    pub const DOWN: Self = Self(-90.);
    pub const LEFT: Self = Self(180.);
    pub const RIGHT: Self = Self(0.);
}

mod from {
    use super::{Compass, CompassHalfwinds, CompassRose, Degrees, Radians, Rotation2D};

    impl From<Radians> for Degrees {
        fn from(radians: Radians) -> Self {
            Self::from_f32(radians.to_degrees())
        }
    }

    impl From<&Radians> for Degrees {
        fn from(radians: &Radians) -> Self {
            Self::from(radians.clone())
        }
    }

    impl From<f32> for Degrees {
        fn from(value: f32) -> Self {
            Self::from_f32(value)
        }
    }

    impl From<&f32> for Degrees {
        fn from(value: &f32) -> Self {
            Self::from_f32(*value)
        }
    }

    impl From<Rotation2D> for Degrees {
        fn from(rotation: Rotation2D) -> Self {
            rotation.degrees()
        }
    }

    impl From<&Rotation2D> for Degrees {
        fn from(rotation: &Rotation2D) -> Self {
            rotation.degrees()
        }
    }

    impl From<Compass> for Degrees {
        fn from(compass_cardinal: Compass) -> Self {
            match compass_cardinal {
                Compass::N => Self::UP,
                Compass::S => Self::DOWN,
                Compass::E => Self::RIGHT,
                Compass::W => Self::LEFT,
            }
        }
    }

    impl From<&Compass> for Degrees {
        fn from(compass: &Compass) -> Self {
            Self::from(compass.clone())
        }
    }

    impl From<CompassRose> for Degrees {
        fn from(compass_rose: CompassRose) -> Self {
            match compass_rose {
                CompassRose::N => Self::UP,
                CompassRose::S => Self::DOWN,
                CompassRose::E => Self::RIGHT,
                CompassRose::W => Self::LEFT,
                CompassRose::NE => Self::from_f32(45.),
                CompassRose::SE => Self::from_f32(135.),
                CompassRose::SW => Self::from_f32(225.),
                CompassRose::NW => Self::from_f32(315.),
            }
        }
    }

    impl From<&CompassRose> for Degrees {
        fn from(compass_rose: &CompassRose) -> Self {
            Self::from(compass_rose.clone())
        }
    }

    impl From<CompassHalfwinds> for Degrees {
        fn from(compass_halfwinds: CompassHalfwinds) -> Self {
            match compass_halfwinds {
                CompassHalfwinds::N => Self::UP,
                CompassHalfwinds::S => Self::DOWN,
                CompassHalfwinds::E => Self::RIGHT,
                CompassHalfwinds::W => Self::LEFT,
                CompassHalfwinds::NE => Self::from_f32(45.),
                CompassHalfwinds::SE => Self::from_f32(135.),
                CompassHalfwinds::SW => Self::from_f32(225.),
                CompassHalfwinds::NW => Self::from_f32(315.),
                CompassHalfwinds::NNE => Self::from_f32(22.5),
                CompassHalfwinds::ENE => Self::from_f32(67.5),
                CompassHalfwinds::ESE => Self::from_f32(112.5),
                CompassHalfwinds::SSE => Self::from_f32(157.5),
                CompassHalfwinds::SSW => Self::from_f32(202.5),
                CompassHalfwinds::WSW => Self::from_f32(247.5),
                CompassHalfwinds::WNW => Self::from_f32(292.5),
                CompassHalfwinds::NNW => Self::from_f32(337.5),
            }
        }
    }

    impl From<&CompassHalfwinds> for Degrees {
        fn from(compass_halfwinds: &CompassHalfwinds) -> Self {
            Self::from(compass_halfwinds.clone())
        }
    }
}

mod into {
    use super::{Degrees, Radians};
    use bevy::math::{Quat, Vec2};

    impl From<Degrees> for Quat {
        fn from(value: Degrees) -> Self {
            Quat::from_rotation_z(value.to_radians().into())
        }
    }

    impl From<&Degrees> for Quat {
        fn from(value: &Degrees) -> Self {
            Quat::from_rotation_z(value.to_radians().into())
        }
    }

    impl From<Degrees> for f32 {
        fn from(value: Degrees) -> Self {
            value.to_f32()
        }
    }

    impl From<&Degrees> for f32 {
        fn from(value: &Degrees) -> Self {
            value.to_f32()
        }
    }

    impl From<Degrees> for Vec2 {
        fn from(value: Degrees) -> Self {
            Vec2::from_angle(Radians::from(value).into())
        }
    }

    impl From<&Degrees> for Vec2 {
        fn from(value: &Degrees) -> Self {
            Vec2::from_angle(Radians::from(value).into())
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
                Degrees::from_f32(self.to_f32() + rhs.to_degrees())
            }
        }

        impl Add<&Radians> for Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() + rhs.to_degrees())
            }
        }

        impl Add<&Radians> for &Degrees {
            type Output = Degrees;
            fn add(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() + rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() - rhs.to_degrees())
            }
        }

        impl Sub<&Radians> for Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() - rhs.to_degrees())
            }
        }

        impl Sub<&Radians> for &Degrees {
            type Output = Degrees;
            fn sub(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() - rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() * rhs.to_degrees())
            }
        }

        impl Mul<&Radians> for Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() * rhs.to_degrees())
            }
        }

        impl Mul<&Radians> for &Degrees {
            type Output = Degrees;
            fn mul(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() * rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() / rhs.to_degrees())
            }
        }

        impl Div<&Radians> for Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() / rhs.to_degrees())
            }
        }

        impl Div<&Radians> for &Degrees {
            type Output = Degrees;
            fn div(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() / rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() % rhs.to_degrees())
            }
        }

        impl Rem<&Radians> for Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees())
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
                Degrees::from_f32(self.to_f32() % rhs.to_degrees())
            }
        }

        impl Rem<&Radians> for &Degrees {
            type Output = Degrees;
            fn rem(self, rhs: &Radians) -> Self::Output {
                Degrees::from_f32(self.to_f32() % rhs.to_degrees())
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
