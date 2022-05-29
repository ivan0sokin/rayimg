use crate::math::{Vec3, Ray};

/// This structure is a simple Camera with position, viewport and focal length, looking in ***-z*** direction.
/// ```
/// use rayimg::{Camera, math::{Vec3, Ray}};
///
/// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (16.0, 9.0), 1.0);
/// assert_eq!(camera.ray_to_viewport(&(0.0, 0.0)), Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(-8.0, -4.5, -1.0)));
/// ```
#[derive(Clone)]
pub struct Camera {
    position: Vec3<f64>,
    viewport: (f64, f64),
    focal_len: f64
}

impl Camera {
    /// Create new Camera
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), (16.0, 9.0), 1.0);
    /// assert!(camera.position() == Vec3::new(0.0, 0.0, 0.0) && camera.viewport() == (16.0, 9.0) && camera.focal_len() == 1.0);
    /// ```
    pub fn new(position: Vec3<f64>, viewport: (f64, f64), focal_len: f64) -> Self {
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
    pub fn position(&self) -> Vec3<f64> {
        self.position.clone()
    }

    /// Returns camera viewport
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.viewport(), (16.0, 9.0));
    /// ```
    pub fn viewport(&self) -> (f64, f64) {
        self.viewport.clone()
    }

    /// Returns focal length of camera
    /// ```
    /// # use rayimg::{Camera, math::Vec3};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.focal_len(), 1.0);
    /// ```
    pub fn focal_len(&self) -> f64 {
        self.focal_len
    }

    /// Returns ray to camera viewport with some offset in interval 0.0..1.0
    /// ```
    /// # use rayimg::{Camera, math::{Vec3, Ray}};
    /// let camera = Camera::new(Vec3::new(-2.0, 1.0, 4.0), (16.0, 9.0), 1.0);
    /// assert_eq!(camera.ray_to_viewport(&(0.5, 0.5)), Ray::new(Vec3::new(-2.0, 1.0, 4.0), Vec3::new(0.0, 0.0, -1.0)));
    /// ```
    pub fn ray_to_viewport(&self, offset: &(f64, f64)) -> Ray {
        Ray::new(self.position(), self.lower_left_corner() + self.horizontal() * offset.0 + self.vertical() * offset.1 - self.position()) 
    }

    fn lower_left_corner(&self) -> Vec3<f64> {
        self.position() - self.horizontal() * 0.5 - self.vertical() * 0.5 - self.frontal()
    }

    fn horizontal(&self) -> Vec3<f64> {
        Vec3::new(self.viewport.0, 0.0, 0.0)
    }

    fn vertical(&self) -> Vec3<f64> {
        Vec3::new(0.0, self.viewport.1, 0.0)
    }

    fn frontal(&self) -> Vec3<f64> {
        Vec3::new(0.0, 0.0, self.focal_len)
    }
}