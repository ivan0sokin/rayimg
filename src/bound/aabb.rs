use crate::{math::{Ray, Vec3}, Hit, HitRecord, Interval};

#[derive(Debug, Clone, Copy, Default)]
pub struct AABB {
    pub axes: [Interval; 3]
}

impl AABB {
    pub fn new() -> Self {
        Self {
            axes: [Interval::new(); 3]
        }
    }

    pub fn with_intervals(axes: [Interval; 3]) -> Self {
        Self {
            axes
        }
    }

    pub fn from_two_points(a: Vec3<f64>, b: Vec3<f64>) -> Self {
        let mut aabb = Self::new();

        for index in 0..3 {
            aabb.axes[index] = Interval::limits(a[index].min(b[index]), a[index].max(b[index]));
        }

        aabb
    }

    pub fn unite(first: AABB, second: AABB) -> Self {
        let mut aabb = Self::new();

        for index in 0..3 {
            aabb.axes[index] = Interval::unite(first.axes[index], second.axes[index]);
        }

        aabb
    }
}

impl Hit for AABB {
    fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        for index in 0..3 {
            let one_over_direction = 1.0 / ray.direction()[index];
            let origin = ray.origin()[index];

            let mut t0 = (self.axes[index].min - origin) * one_over_direction;
            let mut t1 = (self.axes[index].max - origin) * one_over_direction;

            if one_over_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_min {
                t_min = t0;
            }

            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return None;
            }
        }

        Some(HitRecord::new(0.0, Vec3::default()))
    }

    fn bounding(&self) -> AABB {
        *self
    }
}