use crate::{hit::HitRecord, rgb::RGB, scatter::Scatter, math::{Vec3, Ray}};

/// A simple diffuse material.\
/// When rays intersects object it bounces from the surface in random direction.
pub struct Lambertian {
    albedo: RGB
}

impl Lambertian {
    /// Creates new Lambertian material.
    pub fn new(albedo: RGB) -> Self {
        Self {
            albedo
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_in_unit_sphere().normalize();
        if scatter_direction.near_epsilon(1e-8) {
            scatter_direction = hit_record.normal();
        }

        let scattered_ray = Ray::new(hit_record.point(), scatter_direction);
        Some((scattered_ray, self.albedo.clone()))
    }
}
