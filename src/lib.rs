#![forbid(unsafe_code)]

mod scene;
mod hit;
mod scatter;
pub mod materials;
pub mod math;
mod renderer;
mod image_write;
mod rgb;
mod camera;
pub mod shapes;
mod random;

pub use {camera::Camera,
         image_write::{ImageWrite, P3ImageWriter},
         hit::{Hit, HitRecord},
         scatter::Scatter,
         renderer::Renderer,
         rgb::RGB,
         scene::Scene};
