use crate::{rgb::RGB, scatter::Scatter, math::Ray, hit::HitRecord};

pub struct Metal {
    albedo: RGB
}

impl Metal {
    pub fn new(albedo: RGB) -> Self {
        Self {
            albedo
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected = ray.direction().normalize().reflect(&hit_record.normal());
        if reflected.dot(&hit_record.normal()) <= 0.0 {
            None
        } else {
            Some((Ray::new(hit_record.point(), reflected), self.albedo.clone()))
        }
    }
}
