mod camera_builder;
mod lens;

use {crate::math::{Vec3, Ray}};
use camera_builder::CameraBuilder;
use lens::Lens;

/// This structure is a simple Camera which casts ray to lower left corner of its viewport.
#[derive(Clone)]
pub struct Camera {
    pub(super) position: Vec3<f64>,
    pub(super) lower_left_corner: Vec3<f64>,
    pub(super) horizontal: Vec3<f64>,
    pub(super) vertical: Vec3<f64>,
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
            viewport_height: 2.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_distance: 1.0
        }
    }

    /// Returns ray to camera viewport with some offset in interval `0.0..=1.0`.
    pub fn ray_to_viewport(&self, offset: &(f64, f64)) -> Ray {
        let lens_offset = self.lens.random_offset();
        Ray::new(self.position.clone() + lens_offset.clone(), self.lower_left_corner.clone() + self.horizontal.clone() * offset.0 + self.vertical.clone() * offset.1 - self.position.clone() - lens_offset)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new().build()
    }
}