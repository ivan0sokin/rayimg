use crate::math::Vec3;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone)]
pub struct Camera<T> {
    pub position: Vec3<T>,
    pub viewport: (T, T),
    pub focal_length: T
}

impl<T: Copy> Camera<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    /// Create new Camera&lt;T&gt;
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (16.0, 9.0), 1.0);
    /// assert!(camera.position == Vec3::new(0.0, 0.0, 0.0) && camera.viewport == (16.0, 9.0) && camera.focal_length == 1.0);
    /// ```
    pub fn new(position: Vec3<T>, viewport: (T, T), focal_length: T) -> Self {
        Self {
            position,
            viewport,
            focal_length
        }
    }

    /// Returns position of lower left corner of viewport
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0, 4.0), 6.0);
    /// assert_eq!(camera.lower_left_corner(), Vec3::new(-1.0, -2.0, -6.0));
    /// ```
    pub fn lower_left_corner(&self) -> Vec3<T> {
        self.position.clone() - self.horizontal().half() - self.vertical().half() - self.frontal()
    }

    /// Returns horizontal vector
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0, 4.0), 6.0);
    /// assert_eq!(camera.horizontal(), Vec3::new(2.0, 0.0, 0.0));
    /// ```
    pub fn horizontal(&self) -> Vec3<T> {
        Vec3::new(self.viewport.0, 0.0.into(), 0.0.into())
    }

    /// Returns vertical vector
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0, 4.0), 6.0);
    /// assert_eq!(camera.vertical(), Vec3::new(0.0, 4.0, 0.0));
    /// ```
    pub fn vertical(&self) -> Vec3<T> {
        Vec3::new(0.0.into(), self.viewport.1, 0.0.into())
    }

    /// Returns frontal vector
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (2.0, 4.0), 6.0);
    /// assert_eq!(camera.frontal(), Vec3::new(0.0, 0.0, 6.0));
    /// ```
    pub fn frontal(&self) -> Vec3<T> {
        Vec3::new(0.0.into(), 0.0.into(), self.focal_length)
    }
}