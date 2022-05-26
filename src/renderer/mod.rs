use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::Ray, hit::Hit};

use std::ops::{Add, Sub, Mul, Div};

pub type RayMiss<'a, T> = dyn Fn(&Ray<T>) -> RGB<f64> + 'a;

pub struct Renderer<'a, T> {
    scene: Scene<'a, T>,
    camera: Camera<T>,
    ray_miss: Box<RayMiss<'a, T>>
}

impl<'a, T> Renderer<'a, T>
    where T: PartialEq + PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> + Default {
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
            let (h, v) = (pixel.0 as f64 / bounds.0 as f64, pixel.1 as f64 / bounds.1 as f64);
            let ray = self.camera.ray_to_viewport(T::from(h), T::from(v));
            if let Some(hit_record) = self.scene.hit(&ray, T::default(), T::from(f64::MAX)) {
                todo!();
            } else {                
                buf[i] = RGB::from_slice(&(self.ray_miss)(&ray).as_bytes());
            }
        }

        for color in buf {
            iw.write_color(&color);
        }
    }
}
