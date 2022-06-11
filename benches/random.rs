#![feature(test)]

extern crate test;

use rayimg::math::Vec3;
use test::{Bencher, black_box};

const ITERATIONS: usize = 1_000;

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