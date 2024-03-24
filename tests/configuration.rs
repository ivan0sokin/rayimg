pub use rayimg::{Camera, Renderer, math::*, RGB, materials::*, shapes::*, HitRecord, Scene, P3ImageWriter};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 400;
pub const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
pub const BOUNDS: (usize, usize) = (WIDTH, HEIGHT);