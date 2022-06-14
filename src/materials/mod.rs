/// Simple materials which scatter light.

mod lambertian;
mod metal;
mod dielectric;

pub use {lambertian::Lambertian, metal::Metal, dielectric::Dielectric};
