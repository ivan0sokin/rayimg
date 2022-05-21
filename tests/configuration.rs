use rayimg::{Camera, math::Vec3};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 400;
pub const HEIGHT: usize = (400 as f64 / ASPECT_RATIO) as usize;
pub const BOUNDS: (usize, usize) = (WIDTH, HEIGHT);
const VIEWPORT: (f64, f64) = (ASPECT_RATIO * 2.0, 2.0);
const FOCAL_LENGTH: f64 = 1.0;

pub fn get_position() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 0.0)
}

pub fn get_camera() -> Camera<f64> {
    Camera::new(get_position(), VIEWPORT, FOCAL_LENGTH)
}