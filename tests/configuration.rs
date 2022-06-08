use rayimg::Camera;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 400;
pub const HEIGHT: usize = (400 as f64 / ASPECT_RATIO) as usize;
pub const BOUNDS: (usize, usize) = (WIDTH, HEIGHT);

pub fn get_camera() -> Camera {
    Camera::new().build()
}