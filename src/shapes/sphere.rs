use crate::{hit::{Hit, HitRecord}, math::{Vec3, Ray}, scatter::Scatter};

use std::rc::Rc;

/// Geometric shape, set of points that are all at the same distance called `radius` from the `center`.
/// Sphere is `Hit`table.
#[derive(Clone)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    material: Rc<dyn Scatter>
}

impl Sphere {
    /// Creates new Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(1.0, 3.0, 2.0), 1.0, Rc::new(Lambertian::default()));
    /// assert!(unit_sphere.center() == Vec3::new(1.0, 3.0, 2.0) && unit_sphere.radius() == 1.0);
    /// ```
    pub fn new(center: Vec3<f64>, radius: f64, material: Rc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            material
        }
    }

    /// Returns center of Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(4.0, -3.0, 1.0), 1.0, Rc::new(Lambertian::default()));
    /// assert_eq!(unit_sphere.center(), Vec3::new(4.0, -3.0, 1.0));
    /// ```
    pub fn center(&self) -> Vec3<f64> {
        self.center.clone()
    }

    /// Returns radius of Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(4.0, -3.0, 1.0), 1.0, Rc::new(Lambertian::default()));
    /// assert_eq!(unit_sphere.radius(), 1.0);
    /// ```
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center();
        let (a, k, c) = (ray.direction().squared_magnitude(), center_to_origin.dot(&ray.direction()), center_to_origin.squared_magnitude() - self.radius * self.radius);
        let discriminant = k * k - a * c;

        if discriminant < f64::default() {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let roots = ((-k - sqrt_d) / a, (-k + sqrt_d) / a);
        let mut t = roots.0;
        if t < t_min || t_max < t {
            t = roots.1;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.trace(t);
        let mut hit_record = HitRecord::new(t, point.clone(),self.material.clone());
        let normal = (point - self.center()) / self.radius;
        hit_record.set_face_normal(ray, normal);
        Some(hit_record)
    }
}
