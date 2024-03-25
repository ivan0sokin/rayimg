use crate::{hit::{Hit, HitRecord}, math::{Vec3, Ray}, scatter::Scatter};

use std::sync::Arc;

/// Geometric shape, set of points that are all at the same distance called `radius` from the `center`.
/// Sphere is `Hit`table.
#[derive(Clone)]
pub struct Sphere<'a> {
    center: Vec3<f64>,
    radius: f64,
    radius_squared: f64,
    material: Arc<dyn Scatter + 'a + Send + Sync>
}

impl<'a> Sphere<'a> {
    /// Creates new Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian, RGB};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(1.0, 3.0, 2.0), 1.0, Lambertian::new(RGB::default()));
    /// assert!(unit_sphere.center() == Vec3::new(1.0, 3.0, 2.0) && unit_sphere.radius() == 1.0);
    /// ```
    pub fn new(center: Vec3<f64>, radius: f64, material: impl Scatter + 'a + Send + Sync) -> Self {
        Self {
            center,
            radius,
            radius_squared: radius * radius,
            material: Arc::new(material)
        }
    }

    /// Returns center of Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian, RGB};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(4.0, -3.0, 1.0), 1.0, Lambertian::new(RGB::default()));
    /// assert_eq!(unit_sphere.center(), Vec3::new(4.0, -3.0, 1.0));
    /// ```
    pub fn center(&self) -> Vec3<f64> {
        self.center.clone()
    }

    /// Returns radius of Sphere.
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3, materials::Lambertian, RGB};
    /// # use std::rc::Rc;
    /// let unit_sphere = Sphere::new(Vec3::new(4.0, -3.0, 1.0), 1.0, Lambertian::new(RGB::default()));
    /// assert_eq!(unit_sphere.radius(), 1.0);
    /// ```
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;
        let ray_direction = ray.direction();
        
        let (a, k, c) = (ray_direction.squared_magnitude(), center_to_origin.dot(&ray_direction), center_to_origin.squared_magnitude() - self.radius_squared);
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
        let mut hit_record = HitRecord::new(t, point);

        let normal = (point - self.center) / self.radius;
        hit_record.set_face_normal(ray, normal);
        
        if let Some(scatter) = self.material.scatter(ray, &hit_record) {
            hit_record.set_scatter(scatter);
        }

        Some(hit_record)
    }
}
