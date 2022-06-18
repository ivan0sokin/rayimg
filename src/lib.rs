#![forbid(unsafe_code)]

mod scene;
mod hit;
mod scatter;

/// Simple materials which scatter light.
pub mod materials;

/// Math needed for ray tracing.
pub mod math;

mod renderer;
mod image_write;
mod rgb;
mod camera;

/// List of hittable shapes.
pub mod shapes;

mod random;

pub use {camera::Camera,
         image_write::{ImageWrite, P3ImageWriter},
         hit::{Hit, HitRecord},
         scatter::Scatter,
         renderer::Renderer,
         rgb::RGB,
         scene::Scene};
