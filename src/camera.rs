use crate::math::{Vec3, Ray};

use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone)]
pub struct Camera<T> {
    position: Vec3<T>,
    viewport: (T, T),
    focal_len: T
}

impl<T: Copy> Camera<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    /// Create new Camera&lt;T&gt;
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (16.0, 9.0), 1.0);
    /// assert!(camera.position() == Vec3::new(0.0, 0.0, 0.0) && camera.viewport() == (16.0, 9.0) && camera.focal_len() == 1.0);
    /// ```
    pub fn new(position: Vec3<T>, viewport: (T, T), focal_len: T) -> Self {
        Self {
            position,
            viewport,
            focal_len
        }
    }

    /// Returns position of camera
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.position(), Vec3::new(-2.0, 1.0, 4.0));
    /// ```
    pub fn position(&self) -> Vec3<T> {
        self.position.clone()
    }

    /// Returns camera viewport
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.viewport(), (16.0, 9.0));
    /// ```
    pub fn viewport(&self) -> (T, T) {
        self.viewport.clone()
    }

    /// Returns focal length of camera
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.focal_len(), 1.0);
    /// ```
    pub fn focal_len(&self) -> T {
        self.focal_len
    }

    /// Returns ray to camera viewport
    /// ```
    /// # use rayimg::{Camera, math::{Vec3, Ray}};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.ray_to_viewport(0.5, 0.5), Ray::new(Vec3::new(-2.0, 1.0, 4.0), Vec3::new(0.0, 0.0, -1.0)));
    /// ```
    pub fn ray_to_viewport(&self, h: T, v: T) -> Ray<T> {
        Ray::new(self.position(), self.lower_left_corner() + self.horizontal() * h + self.vertical() * v - self.position()) 
    }

    fn lower_left_corner(&self) -> Vec3<T> {
        self.position() - self.horizontal().half() - self.vertical().half() - self.frontal()
    }

    fn horizontal(&self) -> Vec3<T> {
        Vec3::new(self.viewport.0, 0.0.into(), 0.0.into())
    }

    fn vertical(&self) -> Vec3<T> {
        Vec3::new(0.0.into(), self.viewport.1, 0.0.into())
    }

    fn frontal(&self) -> Vec3<T> {
        Vec3::new(0.0.into(), 0.0.into(), self.focal_len)
    }
}
