#![feature(test)]

extern crate test;

use rayimg::{materials::{Dielectric, Lambertian, Metal}, math::Vec3, shapes::Sphere, BVHNode, Camera, P3ImageWriter, Renderer, Scene, RGB};
use test::{Bencher, black_box};

const ITERATIONS: usize = 1_000;

fn random_in_zero_to_one() -> f64 {
    Vec3::<f64>::random_in_unit_segment().x.abs()
}

#[bench]
fn random_in_unit_disk(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(Vec3::<f64>::random_in_unit_disk());
        }
    });
}

#[bench]
fn random_in_unit_sphere(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..ITERATIONS {
            black_box(Vec3::<f64>::random_in_unit_sphere());
        }
    });
}