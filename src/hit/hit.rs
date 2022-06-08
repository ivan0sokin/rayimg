use crate::math::Ray;
use super::hit_record::HitRecord;

/// An object that ray can `Hit`.
pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
