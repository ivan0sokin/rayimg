use crate::{math::Ray, hit::HitRecord, rgb::RGB};

/// Describes material scattering properties.
pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)>;
}