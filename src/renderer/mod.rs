mod renderer_builder;

use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::Ray, hit::Hit, random::random_in_range};
use renderer_builder::RendererBuilder;

/// Renders scene to some image (or buffer).
pub struct Renderer<'a> {
    pub(super) scene: Scene<'a>,
    pub(super) camera: Camera,
    pub(super) sample_count: usize,
    pub(super) ray_depth: usize,
    pub(super) ray_miss: Box<dyn Fn(&Ray) -> RGB + 'a>
}

impl<'a> Renderer<'a> {
    /// Returns new `RendererBuilder`.
    pub fn new(scene: Scene<'a>, camera: Camera) -> RendererBuilder {
        RendererBuilder {
            scene,
            camera,
            sample_count: 100,
            ray_depth: 50,
            ray_miss: Box::new(|_| RGB::default())
        }
    }

    /// Renders image to `ImageWrite<[u8; 3]>` buffer.
    pub fn render<IW: ImageWrite<Color = [u8; 3]>>(&self, mut iw: IW) {
        let scale = 1.0 / self.sample_count as f64;
        let bounds = iw.bounds();
        let mut buf = vec![[0, 0, 0]; bounds.0 * bounds.1];
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                let mut color = RGB::default();
                for _ in 0..self.sample_count {
                    let offset = ((x as f64 + random_in_range(0.0..1.0)) / (bounds.0 as f64 - 1.0), (y as f64 + random_in_range(0.0..1.0)) / (bounds.1 as f64 - 1.0));
                    let ray = self.camera.ray_to_viewport(&offset);
                    color += self.ray_color(&ray, self.ray_depth);
                }

                buf[(bounds.1 - y - 1) * bounds.0 + x] = (color * scale).correct_gamma(2.0).as_bytes();
            }
        }

        iw.write_image_data(&buf);
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