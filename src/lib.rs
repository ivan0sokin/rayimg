#![forbid(unsafe_code)]

mod scene;
pub mod shapes;
mod object;
mod math;
mod material;

pub use scene::{Scene, ID};
pub use object::{Object, Material};
pub use math::Vec3;