use super::vec3::Vec3;

/// Ray is an object which consists of `origin` and `direction`.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    origin: Vec3<f64>,
    direction: Vec3<f64>
}

impl Ray {
    /// Creates new Ray.
    /// ```
    /// # use rayimg::math::{Vec3, Ray};
    /// let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
    /// assert!(ray.origin() == Vec3::new(0.0, 0.0, 0.0) && ray.direction() == Vec3::new(0.0, 0.0, -1.0));
    /// ```
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Self {
            origin,
            direction
        }
    }

    /// Returns origin of ray.
    /// ```
    /// # use rayimg::math::{Vec3, Ray};
    /// let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, -3.0, -1.0));
    /// assert_eq!(ray.origin(), Vec3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn origin(&self) -> Vec3<f64> {
        self.origin
    }

    /// Returns direction of ray.
    /// ```
    /// # use rayimg::math::{Vec3, Ray};
    /// let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, -3.0, -1.0));
    /// assert_eq!(ray.direction(), Vec3::new(2.0, -3.0, -1.0));
    /// ```
    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }

    /// Returns end position of traced ray.
    /// ```
    /// # use rayimg::math::{Vec3, Ray};
    /// let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, -3.0, -1.0));
    /// assert_eq!(ray.trace(3.0), Vec3::new(7.0, -7.0, 0.0));
    /// ```
    pub fn trace(&self, t: f64) -> Vec3<f64> {
        self.origin + self.direction * t
    }
}
