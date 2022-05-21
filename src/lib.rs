#![forbid(unsafe_code)]

mod scene;
pub mod shapes;
mod object;
pub mod math;
mod material;
mod renderer;
mod image_write;
mod rgb;
mod camera;
mod id;

pub use {camera::Camera,
         image_write::{ImageWrite, Pixels, P3ImageWriter},
         object::{Material, Object},
         renderer::Renderer,
         rgb::RGB,
         scene::Scene,
         id::ID};