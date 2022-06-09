pub use rayimg::{Camera, Renderer, math::*, RGB, materials::*, shapes::*, HitRecord, Scene, P3ImageWriter};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 400;
pub const HEIGHT: usize = (400 as f64 / ASPECT_RATIO) as usize;
pub const BOUNDS: (usize, usize) = (WIDTH, HEIGHT);

pub fn sky_color() -> Box<dyn Fn(&Ray) -> RGB> {
    Box::new(|r| {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t).into()
    })
}