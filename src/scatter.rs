use crate::{math::Ray, hit::HitRecord, rgb::RGB};

pub trait Scatter<T> {
    fn scatter(&self, ray: &Ray<T>, hit_record: &HitRecord<T>) -> Option<(Ray<T>, RGB<f64>)>;
}