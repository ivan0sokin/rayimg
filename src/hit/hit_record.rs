use crate::math::{Vec3};

pub struct HitRecord<T> {
    point: Vec3<T>,
    t: T
}

impl<T: Copy> HitRecord<T> {
    pub fn new(point: Vec3<T>, t: T) -> Self {
        Self {
            point,
            t
        }
    }

    pub fn point(&self) -> Vec3<T> {
        self.point.clone()
    }

    pub fn t(&self) -> T {
        self.t
    }
}
