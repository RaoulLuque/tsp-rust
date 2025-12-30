use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub},
};

mod fixed_point_arithmetic;
use fixed_point_arithmetic::FIXED_POINT_FRACTIONAL_BITS;
pub use fixed_point_arithmetic::ScaledDistance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Distance(pub i32);

impl Distance {
    pub const MAX: Distance = Distance(i32::MAX >> (FIXED_POINT_FRACTIONAL_BITS));
    pub const MIN: Distance = Distance(i32::MIN + (1 << FIXED_POINT_FRACTIONAL_BITS));
}

impl Add for Distance {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Distance(self.0 + other.0)
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Distance {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Distance(self.0 - other.0)
    }
}

impl Sum<Distance> for Distance {
    fn sum<I: Iterator<Item = Distance>>(iter: I) -> Self {
        iter.fold(Distance(0), |acc, d| acc + d)
    }
}

impl<'a> Sum<&'a Distance> for Distance {
    fn sum<I: Iterator<Item = &'a Distance>>(iter: I) -> Self {
        iter.fold(Distance(0), |acc, d| acc + *d)
    }
}

impl Mul<Distance> for i32 {
    type Output = Distance;

    fn mul(self, rhs: Distance) -> Self::Output {
        Distance(self * rhs.0)
    }
}
