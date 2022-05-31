use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::Ray, hit::Hit, random::random_in_range};

pub type RayMiss<'a> = dyn Fn(&Ray) -> RGB + 'a;

pub struct Renderer<'a> {
    scene: Scene<'a>,
    camera: Camera,
    sample_count: usize,
    ray_depth: usize,
    ray_miss: Box<RayMiss<'a>>
}

impl<'a> Renderer<'a> {
    const DEFAULT_SAMPLE_COUNT: usize = 100;
    const DEFAULT_RAY_DEPTH: usize = 50;

    pub fn new(scene: Scene<'a>, camera: Camera, ray_miss: impl Fn(&Ray) -> RGB + 'a) -> Self {
        Self {
            scene,
            camera,
            sample_count: Self::DEFAULT_SAMPLE_COUNT,
            ray_depth: Self::DEFAULT_RAY_DEPTH,
            ray_miss: Box::new(ray_miss)
        }
    }

    pub fn set_sample_count(&mut self, sample_count: usize) {
        self.sample_count = sample_count.clamp(1, usize::MAX);
    }
    
    pub fn set_ray_depth(&mut self, ray_depth: usize) {
        self.ray_depth = ray_depth.clamp(1, usize::MAX);
    }

    pub fn render<IW: ImageWrite<[u8; 3]>>(&self, mut iw: IW) {
        iw.write_header();

        let scale = 1.0 / self.sample_count as f64;
        let bounds = iw.bounds();
        let mut buf = vec![[0, 0, 0]; bounds.0 * bounds.1];
        for (i, pixel) in iw.pixels().enumerate() {
            let mut color = RGB::default();
            for _ in 0..self.sample_count {
                let (r1, r2) = (random_in_range(0.0..1.0), random_in_range(0.0..1.0));
                let offset = ((pixel.0 as f64 + r1) / bounds.0 as f64, (pixel.1 as f64 + r2) / bounds.1 as f64);
                let ray = self.camera.ray_to_viewport(&offset);
                color += self.ray_color(&ray, self.ray_depth);
            }
            
            buf[i] = (color * scale).correct_gamma(2.0).as_bytes();
        }

        for color in buf {
            iw.write_color(&color);
        }
    }

    fn ray_color(&self, ray: &Ray, depth: usize) -> RGB {
        if depth == 0 {
            return RGB::default();
        }

        if let Some(hit_record) = self.scene.hit(&ray, 0.001, f64::MAX) {
            if let Some((scattered_ray, color)) = hit_record.material().scatter(&ray, &hit_record) {
                return color * self.ray_color(&scattered_ray, depth - 1);
            }
            return RGB::default();
        }

        (self.ray_miss)(&ray)
    }
}
