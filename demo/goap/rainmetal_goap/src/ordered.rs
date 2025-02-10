use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use bevy::prelude::{Vec2, Vec3};
use ordered_float::OrderedFloat;
use crate::distance::Distance;

#[derive(Copy, Clone, Debug, Default)]
pub struct OrderedVec3(pub Vec3);

#[derive(Copy, Clone, Debug, Default)]
pub struct OrderedVec2(pub Vec2);

impl OrderedVec3 {
    pub const ZERO : OrderedVec3 = OrderedVec3(Vec3::ZERO);
}

impl OrderedVec2 {
    pub const ZERO : OrderedVec2 = OrderedVec2(Vec2::ZERO);
}

impl Hash for OrderedVec2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat::from(self.0.x).hash(state);
        OrderedFloat::from(self.0.y).hash(state);
    }
}

impl Hash for OrderedVec3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat::from(self.0.x).hash(state);
        OrderedFloat::from(self.0.y).hash(state);
        OrderedFloat::from(self.0.z).hash(state);
    }
}

impl PartialEq for OrderedVec2 {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat::from(self.0.x).eq(&OrderedFloat::from(other.0.x)) &&
        OrderedFloat::from(self.0.y).eq(&OrderedFloat::from(other.0.y))
    }
}

impl PartialEq for OrderedVec3 {
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat::from(self.0.x).eq(&OrderedFloat::from(other.0.x)) &&
        OrderedFloat::from(self.0.y).eq(&OrderedFloat::from(other.0.y)) &&
        OrderedFloat::from(self.0.z).eq(&OrderedFloat::from(other.0.z))
    }
}

impl Eq for OrderedVec2 {}
impl Eq for OrderedVec3 {}

impl Distance for OrderedVec2 {
    fn distance(&self, other: &Self) -> usize {
        Vec2::distance(self.0, other.0) as usize
    }
}

impl Distance for OrderedVec3 {
    fn distance(&self, other: &Self) -> usize {
        Vec3::distance(self.0, other.0) as usize
    }
}

impl Add for OrderedVec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        OrderedVec3(self.0 + other.0)
    }
}

impl Add<Vec3> for OrderedVec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        OrderedVec3(self.0 + other)
    }
}

impl Add for OrderedVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        OrderedVec2(self.0 + other.0)
    }
}

impl Add<Vec2> for OrderedVec2 {
    type Output = Self;

    fn add(self, other: Vec2) -> Self {
        OrderedVec2(self.0 + other)
    }
}

impl AddAssign for OrderedVec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}


impl AddAssign<Vec3> for OrderedVec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other;
    }
}

impl AddAssign for OrderedVec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}


impl AddAssign<Vec2> for OrderedVec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.0 += other;
    }
}

impl Mul for OrderedVec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        OrderedVec3(self.0 * rhs.0)
    }
}

impl Mul<Vec3> for OrderedVec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        OrderedVec3(self.0 * rhs)
    }
}

impl Mul for OrderedVec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        OrderedVec2(self.0 * rhs.0)
    }
}

impl Mul<Vec2> for OrderedVec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        OrderedVec2(self.0 * rhs)
    }
}

impl MulAssign for OrderedVec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<Vec3> for OrderedVec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs;
    }
}

impl MulAssign for OrderedVec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl MulAssign<Vec2> for OrderedVec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.0 *= rhs;
    }
}