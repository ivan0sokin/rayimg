mod camera_builder;
mod lens;

use crate::math::{Ray, Vec3};
use camera_builder::CameraBuilder;
use lens::Lens;

/// This structure is a simple Camera which casts ray to lower left corner of its viewport.
#[derive(Clone)]
pub struct Camera {
    pub(super) position: Vec3<f64>,
    pub(super) upper_left_corner: Vec3<f64>,
    pub(super) horizontal: Vec3<f64>,
    pub(super) minus_vertical: Vec3<f64>,
    pub(super) lens: Lens
}

impl Camera {
    /// Returns `CameraBuilder` to setup the camera.
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            position: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            vertical_fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            defocus_angle: 0.0,
            focus_distance: 1.0
        }
    }

    /// Returns ray to camera viewport with some offset in interval `0.0..=1.0`.
    pub fn ray_to_viewport(&self, offset: &(f64, f64)) -> Ray {
        let lens_offset = self.lens.random_offset();
        
        let point = self.upper_left_corner + self.horizontal * offset.0 + self.minus_vertical * offset.1;

        let origin = self.position + lens_offset;
        let direction = point - origin;

        Ray::new(origin, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new().build()
    }
}