use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::Ray, hit::Hit, random::random_in_range};

pub type RayMiss<'a> = dyn Fn(&Ray) -> RGB + 'a;

pub struct Renderer<'a> {
    scene: Scene<'a>,
    camera: Camera,
    ray_miss: Box<RayMiss<'a>>
}

impl<'a> Renderer<'a> {
    const DEPTH: usize = 125;

    pub fn new(scene: Scene<'a>, camera: Camera, ray_miss: impl Fn(&Ray) -> RGB + 'a) -> Self {
        Self {
            scene,
            camera,
            ray_miss: Box::new(ray_miss)
        }
    }

    pub fn render<IW: ImageWrite<[u8; 3]>>(&self, mut iw: IW) {
        iw.write_header();

        let bounds = iw.bounds();
        let mut buf = vec![[0, 0, 0]; bounds.0 * bounds.1];
        for (i, pixel) in iw.pixels().enumerate() {
            let mut color = RGB::default();
            let samples = 100;
            for _ in 0..samples {
                let (r1, r2) = (random_in_range(-1.0..1.0), random_in_range(-1.0..1.0));
                let offset = ((pixel.0 as f64 + r1) / bounds.0 as f64, (pixel.1 as f64 + r2) / bounds.1 as f64);
                let ray = self.camera.ray_to_viewport(&offset);
                color += self.ray_color(&ray, Self::DEPTH);
            }

            color.0 /= samples as f64;
            color.1 /= samples as f64;
            color.2 /= samples as f64;

            color.0 = color.0.clamp(0.0, 1.0);
            color.1 = color.1.clamp(0.0, 1.0);
            color.2 = color.2.clamp(0.0, 1.0);

            buf[i] = color.as_bytes();
        }

        for color in buf {
            iw.write_color(&color);
        }
    }

    fn ray_color(&self, ray: &Ray, depth: usize) -> RGB {
        if depth == 0 {
            return RGB::default();
        }

        if let Some(hit_record) = self.scene.hit(&ray, f64::default(), f64::MAX) {
            if let Some((scattered_ray, color)) = hit_record.material().scatter(&ray, &hit_record) {
                return color * self.ray_color(&scattered_ray, depth - 1);
            }
            return RGB::default();
        }

        (self.ray_miss)(&ray)
    }
}
