use crate::math::Ray;
use super::hit_record::HitRecord;

/// An object that ray can `Hit`.
pub trait Hit {
    /// Returns `HitRecord` if ray hits object that implements `Hit` trait or None if ray does not intersects with it.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}