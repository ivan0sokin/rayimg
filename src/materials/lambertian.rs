use crate::{hit::HitRecord, rgb::RGB, scatter::Scatter, math::{Vec3, Ray}};

#[derive(Default)]
pub struct Lambertian {
    albedo: RGB
}

impl Lambertian {
    pub fn new(albedo: RGB) -> Self {
        Self {
            albedo
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let random_unit_vector = Vec3::random().normalize();
        let mut scatter_direction = hit_record.normal() + random_unit_vector;

        let s = 1e-8;
        if scatter_direction.x < s && scatter_direction.y < s && scatter_direction.z < s {
            scatter_direction = hit_record.normal();
        }

        let scattered_ray = Ray::new(hit_record.point(), scatter_direction);
        Some((scattered_ray, self.albedo.clone()))
    }
}