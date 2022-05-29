use crate::{math::{Vec3, Ray}, scatter::Scatter};

use std::rc::Rc;

pub struct HitRecord {
    t: f64,
    point: Vec3<f64>,
    normal: Vec3<f64>,
    material: Rc<dyn Scatter>
}

impl HitRecord {
    /// Creates new HitRecord&lt;f64&gt;
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let hit_record = HitRecord::new(0.0, Vec3::new(1.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// assert!(hit_record.t() == 0.0 && hit_record.point() == Vec3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn new(t: f64, point: Vec3<f64>, material: Rc<dyn Scatter>) -> Self {
        Self {
            t,
            point,
            normal: Vec3::default(),
            material
        }
    }

    /// Returns multiplier ***t*** of hit record
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// assert_eq!(hit_record.t(), 5.7);
    /// ```
    pub fn t(&self) -> f64 {
        self.t
    }

    /// Returns point of hit
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// assert_eq!(hit_record.point(), Vec3::new(-4.0, 2.0, 3.0));
    /// ```
    pub fn point(&self) -> Vec3<f64> {
        self.point.clone()
    }

    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3<f64>) {
        let front_face = ray.direction().dot(&normal) < f64::default();
        self.normal = if front_face { normal } else { -normal };
    }

    /// Returns normal of hit surface
    /// ```
    /// # use rayimg::{HitRecord, math::{Vec3, Ray}, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let mut hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// hit_record.set_face_normal(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)), Vec3::new(0.0, 0.0, -1.0));
    /// assert_eq!(hit_record.normal(), Vec3::new(0.0, 0.0, -1.0));
    /// ```
    pub fn normal(&self) -> Vec3<f64> {
        self.normal.clone()
    }

    pub fn material(&self) -> Rc<dyn Scatter> {
        self.material.clone()
    }
}
