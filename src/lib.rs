#![forbid(unsafe_code)]

mod scene;
pub mod shapes;
mod hit;
pub mod math;
mod material;
mod renderer;
mod image_write;
mod rgb;
mod camera;

pub use {camera::Camera,
         image_write::{ImageWrite, Pixels, P3ImageWriter},
         hit::{Hit, HitRecord},
         renderer::Renderer,
         rgb::RGB,
         scene::Scene};
