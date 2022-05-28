use super::vec3::Vec3;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Ray<T> {
    origin: Vec3<T>,
    direction: Vec3<T>
}

impl<T> Ray<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Vec3<T> {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3<T> {
        self.direction.clone()
    }

    pub fn trace(&self, t: T) -> Vec3<T> {
        self.origin.clone() + self.direction.clone() * t
    }
}
