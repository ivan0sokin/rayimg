use crate::{math::{Vec3, Ray}, scatter::Scatter};

use std::{rc::Rc, ops::{Add, Sub, Mul, Div, Neg}};

pub struct HitRecord<T> {
    t: T,
    point: Vec3<T>,
    normal: Vec3<T>,
    material: Rc<dyn Scatter<T>>
}

impl<T> HitRecord<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + From<f64> + Into<f64> + Default + PartialOrd {
    /// Creates new HitRecord&lt;T&gt;
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let hit_record = HitRecord::new(0.0, Vec3::new(1.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// assert!(hit_record.t() == 0.0 && hit_record.point() == Vec3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn new(t: T, point: Vec3<T>, material: Rc<dyn Scatter<T>>) -> Self {
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
    pub fn t(&self) -> T {
        self.t
    }

    /// Returns point of hit
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0), Rc::new(Lambertian::default()));
    /// assert_eq!(hit_record.point(), Vec3::new(-4.0, 2.0, 3.0));
    /// ```
    pub fn point(&self) -> Vec3<T> {
        self.point.clone()
    }

    pub fn set_face_normal(&mut self, ray: &Ray<T>, normal: Vec3<T>) {
        let front_face = ray.direction().dot(&normal) < T::default();
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
    pub fn normal(&self) -> Vec3<T> {
        self.normal.clone()
    }

    pub fn material(&self) -> Rc<dyn Scatter<T>> {
        self.material.clone()
    }
}
