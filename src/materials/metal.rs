use crate::{rgb::RGB, scatter::Scatter, math::{Vec3, Ray}, hit::HitRecord};

pub struct Metal {
    albedo: RGB,
    fuzziness: f64
}

impl Metal {
    pub fn new(albedo: RGB, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.clamp(0.0, 1.0)
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected = ray.direction().normalize().reflect(&hit_record.normal()) + Vec3::random().normalize() * self.fuzziness;
        if reflected.dot(&hit_record.normal()) <= 0.0 {
            None
        } else {
            Some((Ray::new(hit_record.point(), reflected), self.albedo.clone()))
        }
    }
}
