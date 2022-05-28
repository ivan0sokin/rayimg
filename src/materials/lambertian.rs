use crate::{hit::HitRecord, rgb::RGB, scatter::Scatter, math::{Vec3, Ray}};

use std::ops::{Add, Sub, Mul, Div, Neg};

use rand::random;

#[derive(Default)]
pub struct Lambertian {
    albedo: RGB<f64>
}

impl Lambertian {
    pub fn new(albedo: RGB<f64>) -> Self {
        Self {
            albedo
        }
    }
}

impl<T> Scatter<T> for Lambertian
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + From<f64> + Into<f64> + Default + PartialOrd {
    fn scatter(&self, ray: &Ray<T>, hit_record: &HitRecord<T>) -> Option<(Ray<T>, RGB<f64>)> {
        let random_unit_vector = Vec3::random().normalize();
        let mut scatter_direction = hit_record.normal() + random_unit_vector;

        let s = 1e-8.into();
        if scatter_direction.x < s && scatter_direction.y < s && scatter_direction.z < s {
            scatter_direction = hit_record.normal();
        }

        let scattered_ray = Ray::new(hit_record.point(), scatter_direction);
        Some((scattered_ray, self.albedo.clone()))
    }
}