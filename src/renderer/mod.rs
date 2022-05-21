use crate::scene::Scene;
use crate::image_write::ImageWrite;
use crate::rgb::RGB;
use crate::camera::Camera;
use crate::math::Ray;

use std::ops::{Add, Sub, Mul, Div};

pub type RayMiss<'a, T> = dyn Fn(&Ray<T>) -> RGB<f64> + 'a;

pub struct Renderer<'a, T> {
    scene: Scene<'a>,
    camera: Camera<T>,
    ray_miss: Box<RayMiss<'a, T>>
}

impl<'a, T> Renderer<'a, T>
    where T: PartialEq + PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    pub fn new(scene: Scene<'a>, camera: Camera<T>, ray_miss: impl Fn(&Ray<T>) -> RGB<f64> + 'a) -> Self {
        Self {
            scene,
            camera,
            ray_miss: Box::new(ray_miss)
        }
    }

    pub fn render(&self, image_writer: &mut impl ImageWrite<RGB<u8>>) {
        image_writer.write_header();

        let bounds = image_writer.bounds();
        let mut buf = vec![RGB::default(); bounds.0 * bounds.1];
        for (i, pixel) in image_writer.pixels().enumerate() {
            let (u, v) = (pixel.0 as f64 / bounds.0 as f64, pixel.1 as f64 / bounds.1 as f64);
            let ray = Ray::new(self.camera.position.clone(), self.camera.lower_left_corner() + self.camera.horizontal() * T::from(u) + self.camera.vertical() * T::from(v) - self.camera.position.clone());
            buf[i] = RGB::from_slice(&(self.ray_miss)(&ray).as_bytes());
        }

        for color in buf {
            image_writer.write_color(&color);
        }
    }
}