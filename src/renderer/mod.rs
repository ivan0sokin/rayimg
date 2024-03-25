mod renderer_builder;

use crate::{scene::Scene, image_write::ImageWrite, rgb::RGB, camera::Camera, math::Ray, hit::Hit, random::random_in_range};
use renderer_builder::RendererBuilder;

/// Renders scene to some image (or buffer).
pub struct Renderer<'a> {
    pub(super) scene: Scene<'a>,
    pub(super) camera: Camera,
    pub(super) sample_count: usize,
    pub(super) ray_depth: usize,
    pub(super) ray_miss: Box<dyn Fn(&Ray) -> RGB + 'a + Sync>
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

    /// Renders image to `ImageWrite` buffer.
    pub fn render<IW: ImageWrite>(&self, mut iw: IW) {
        let scale = 1.0 / self.sample_count as f64;
        let bounds = iw.bounds();
        let mut buf = vec![RGB::default(); bounds.0 * bounds.1];
        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                let mut color = RGB::default();
                for _ in 0..self.sample_count {
                    let offset = ((x as f64 + random_in_range(0.0..1.0)) / (bounds.0 as f64 - 1.0), (y as f64 + random_in_range(0.0..1.0)) / (bounds.1 as f64 - 1.0));
                    let ray = self.camera.ray_to_viewport(&offset);
                    color += self.ray_color(&ray, self.ray_depth);
                }

                buf[(bounds.1 - y - 1) * bounds.0 + x] = (color * scale).correct_gamma(2.0);
            }
        }

        iw.write_all(&buf);
    }

    /// Renders image to `ImageWrite` buffer multithreaded using all threads.
    pub fn render_multithreaded<IW: ImageWrite>(&self, mut iw: IW) {
        let scale = 1.0 / self.sample_count as f64;
        let bounds = iw.bounds();
        let mut buf = vec![RGB::default(); bounds.0 * bounds.1];

        let thread_count = std::thread::available_parallelism().unwrap().get();
        let pixels_per_thread = buf.len() / (thread_count + 1);
        let chunks = buf.chunks_mut(pixels_per_thread).collect::<Vec<&mut [RGB]>>();

        std::thread::scope(|scope| {
            for (chunk_index, chunk) in chunks.into_iter().enumerate() {
                let offset = chunk_index * pixels_per_thread;
                scope.spawn(move || {
                    let mut pixel = [offset % bounds.0, offset / bounds.0];
                    for index in 0..chunk.len() {
                        if pixel[0] >= bounds.0 {
                            pixel[0] = 0;
                            pixel[1] += 1;
                        }

                        let mut color = RGB::default();
                        for _ in 0..self.sample_count {
                            let offset_x = (pixel[0] as f64 + random_in_range(0.0..1.0) - 0.5) / (bounds.0 as f64 - 1.0);
                            let offset_y = (pixel[1] as f64 + random_in_range(0.0..1.0) - 0.5) / (bounds.1 as f64 - 1.0);
                            let ray = self.camera.ray_to_viewport(&(offset_x, offset_y));
                            color += self.ray_color(&ray, self.ray_depth);
                        }

                        chunk[index] = (color * scale).correct_gamma(2.0);

                        pixel[0] += 1;
                    }
                });
            }
        });

        iw.write_all(&buf);
    }

    fn ray_color(&self, ray: &Ray, depth: usize) -> RGB {
        if depth == 0 {
            return RGB::default();
        }

        if let Some(hit_record) = self.scene.hit(&ray, 0.001, f64::MAX) {
            if let Some((scattered_ray, color)) = hit_record.scatter() {
                return color * self.ray_color(&scattered_ray, depth - 1);
            }
            return RGB::default();
        }

        (self.ray_miss)(&ray)
    }
}