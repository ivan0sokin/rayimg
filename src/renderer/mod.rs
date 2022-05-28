use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::{Vec3, Ray}, hit::Hit};

use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;

pub type RayMiss<'a, T> = dyn Fn(&Ray<T>) -> RGB<f64> + 'a;

pub struct Renderer<'a, T> {
    scene: Scene<'a, T>,
    camera: Camera<T>,
    ray_miss: Box<RayMiss<'a, T>>
}

impl<'a, T> Renderer<'a, T>
    where T: PartialEq + PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + From<f64> + Into<f64> + Default {
    const DEPTH: isize = 125;

    pub fn new(scene: Scene<'a, T>, camera: Camera<T>, ray_miss: impl Fn(&Ray<T>) -> RGB<f64> + 'a) -> Self {
        Self {
            scene,
            camera,
            ray_miss: Box::new(ray_miss)
        }
    }

    pub fn render<IW: ImageWrite<RGB<u8>>>(&self, mut iw: IW) {
        iw.write_header();

        let bounds = iw.bounds();
        let mut buf = vec![RGB::default(); bounds.0 * bounds.1];
        for (i, pixel) in iw.pixels().enumerate() {
            let mut color = RGB::default();
            let samples = 100;
            for s in 0..samples {
                let (r1, r2) = (rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>());
                let (h, v) = ((pixel.0 as f64 + r1) / bounds.0 as f64, (pixel.1 as f64 + r2) / bounds.1 as f64);
                let ray = self.camera.ray_to_viewport(T::from(h), T::from(v));
                color += self.ray_color(&ray, Self::DEPTH);
            }

            color.0 /= samples as f64;
            color.1 /= samples as f64;
            color.2 /= samples as f64;

            color.0 = color.0.clamp(0.0, 1.0);
            color.1 = color.1.clamp(0.0, 1.0);
            color.2 = color.2.clamp(0.0, 1.0);

            buf[i] = RGB::from_slice(&color.as_bytes());
        }

        for color in buf {
            iw.write_color(&color);
        }
    }

    fn ray_color(&self, ray: &Ray<T>, depth: isize) -> RGB<f64> {
        if depth <= 0 {
            return RGB::default();
        }

        if let Some(hit_record) = self.scene.hit(&ray, T::default(), T::from(f64::MAX)) {
            if let Some((scattered_ray, color)) = hit_record.material().scatter(&ray, &hit_record) {
                return color * self.ray_color(&scattered_ray, depth - 1);
            }
            return RGB::default();
        }

        (self.ray_miss)(&ray)
    }
}
