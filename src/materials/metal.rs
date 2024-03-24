use crate::{rgb::RGB, scatter::Scatter, math::{Vec3, Ray}, hit::HitRecord};

/// Material that reflects incident rays.
pub struct Metal {
    albedo: RGB,
    fuzziness: f64
}

impl Metal {
    /// Creates new Metal material.
    pub fn new(albedo: RGB, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.clamp(0.0, 1.0)
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let normal = hit_record.normal();

        let reflected = ray.direction().normalize().reflect(normal);
        if reflected.dot(&normal) <= 0.0 {
            None
        } else {
            Some((Ray::new(hit_record.point(), reflected + Vec3::random_in_unit_sphere() * self.fuzziness), self.albedo))
        }
    }
}
