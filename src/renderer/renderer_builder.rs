use super::Renderer;
use crate::{rgb::RGB, math::Ray, scene::Scene, camera::Camera};

/// `RendererBuilder` builds a renderer with set parameters.
pub struct RendererBuilder<'a> {
    pub(super) scene: Scene<'a>,
    pub(super) camera: Camera,
    pub(super) sample_count: usize,
    pub(super) ray_depth: usize,
    pub(super) ray_miss: Box<dyn Fn(&Ray) -> RGB + 'a + Sync>
}

impl<'a> RendererBuilder<'a> {
    /// Sets count of samples per pixel.
    pub fn sample_count(mut self, sample_count: usize) -> Self {
        self.sample_count = sample_count;
        self
    }

    /// Sets maximum ray refractions/reflections count.
    pub fn ray_depth(mut self, ray_depth: usize) -> Self {
        self.ray_depth = ray_depth;
        self
    }

    /// Sets missing ray color.
    pub fn ray_miss(mut self, ray_miss: impl Fn(&Ray) -> RGB + 'a + Sync) -> Self {
        self.ray_miss = Box::new(ray_miss);
        self
    }

    /// Returns built `Renderer`.
    pub fn build(self) -> Renderer<'a> {
        Renderer {
            scene: self.scene,
            camera: self.camera,
            sample_count: self.sample_count,
            ray_depth: self.ray_depth,
            ray_miss: self.ray_miss
        }
    }
}