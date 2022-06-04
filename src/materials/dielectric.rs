use crate::{rgb::RGB, scatter::Scatter, math::Ray, hit::HitRecord, random::random_in_range};

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let unit_direction = ray.direction().normalize();
        let refraction_ratio = if hit_record.front_face() { 1.0 / self.refraction_index } else { self.refraction_index };

        let cos_theta = (-unit_direction.clone()).dot(&hit_record.normal());
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_in_range(0.0..1.0) {
            unit_direction.reflect(&hit_record.normal())
        } else {
            unit_direction.refract(&hit_record.normal(), refraction_ratio)
        };

        Some((Ray::new(hit_record.point(), direction), RGB(1.0, 1.0, 1.0)))
    }
}