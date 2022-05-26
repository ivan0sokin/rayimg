use crate::math::Ray;
use super::hit_record::HitRecord;

pub trait Hit<T> {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}
