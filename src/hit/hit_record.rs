use crate::{math::{Ray, Vec3}, RGB};

/// Record of ray-object intersection.
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    t: f64,
    point: Vec3<f64>,
    normal: Vec3<f64>,
    front_face: bool,
    scatter: Option<(Ray, RGB)>
}

impl HitRecord {
    /// Creates new `HitRecord`.
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian, RGB};
    /// let hit_record = HitRecord::new(0.0, Vec3::new(1.0, 2.0, 3.0));
    /// assert!(hit_record.t() == 0.0 && hit_record.point() == Vec3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn new(t: f64, point: Vec3<f64>) -> Self {
        Self {
            t,
            point,
            normal: Vec3::default(),
            front_face: bool::default(),
            scatter: None
        }
    }

    /// Returns multiplier `t` of hit record.
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian, RGB};
    /// let hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0));
    /// assert_eq!(hit_record.t(), 5.7);
    /// ```
    pub fn t(&self) -> f64 {
        self.t
    }

    /// Returns point of hit.
    /// ```
    /// # use rayimg::{HitRecord, math::Vec3, materials::Lambertian, RGB};
    /// let hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0));
    /// assert_eq!(hit_record.point(), Vec3::new(-4.0, 2.0, 3.0));
    /// ```
    pub fn point(&self) -> Vec3<f64> {
        self.point.clone()
    }

    /// Sets face normal and determines whether normal points inwards or outwards
    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3<f64>) {
        self.front_face = ray.direction().dot(&normal) < f64::default();
        self.normal = if self.front_face { normal } else { -normal };
    }

    pub fn set_scatter(&mut self, scatter: (Ray, RGB)) {
        self.scatter = Some(scatter);
    }

    /// Get scattering result
    pub fn scatter(&self) -> Option<(Ray, RGB)> {
        self.scatter.clone()
    }

    /// Returns normal of hit surface
    /// ```
    /// # use rayimg::{HitRecord, math::{Vec3, Ray}, materials::Lambertian, RGB};
    /// let mut hit_record = HitRecord::new(5.7, Vec3::new(-4.0, 2.0, 3.0));
    /// hit_record.set_face_normal(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)), Vec3::new(0.0, 0.0, -1.0));
    /// assert_eq!(hit_record.normal(), Vec3::new(0.0, 0.0, -1.0));
    /// ```
    pub fn normal(&self) -> Vec3<f64> {
        self.normal.clone()
    }

    /// Returns true if normal points outwards.
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}
