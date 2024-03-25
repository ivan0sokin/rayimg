use std::{cmp::Ordering, sync::Arc};

use crate::{math::Ray, random::random_in_range, Hit, HitRecord, Scene, AABB};

pub struct BVHNode<'a> {
    left: Arc<dyn Hit + 'a + Send + Sync>,
    right: Arc<dyn Hit + 'a + Send + Sync>,
    aabb: AABB
}

impl<'a> BVHNode<'a> {
    pub fn from_scene(scene: Scene<'a>) -> Self {
        BVHNode::from_objects(&mut scene.objects())
    }

    fn from_objects(objects: &mut [Arc<dyn Hit + 'a + Send + Sync>]) -> Self {
        let axis = random_in_range(0..=2);

        let (left, right);
        if objects.len() == 1 {
            (left, right) = (objects[0].clone(), objects[0].clone()); 
        } else if objects.len() == 2 {
            if Self::compare_axis(&objects[0], &objects[1], axis).is_lt() {
                (left, right) = (objects[0].clone(), objects[1].clone())
            } else {
                (left, right) = (objects[1].clone(), objects[0].clone())
            }
        } else {
            objects.sort_by(|a, b| Self::compare_axis(a, b, axis));

            let mid = objects.len() / 2;
            left = Arc::new(BVHNode::from_objects(&mut objects[..mid]));
            right = Arc::new(BVHNode::from_objects(&mut objects[mid..]))
        }

        Self {
            left: left.clone(),
            right: right.clone(),
            aabb: AABB::unite(left.bounding(), right.bounding())
        }
    }
}

impl<'a> BVHNode<'a> {
    fn compare_axis(a: &Arc<dyn Hit + 'a + Send + Sync>, b: &Arc<dyn Hit + 'a + Send + Sync>, index: usize) -> Ordering {
        a.bounding().axes[index].min.total_cmp(&b.bounding().axes[index].min)
    }
}

impl<'a> Hit for BVHNode<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if self.aabb.hit(ray, t_min, t_max).is_none() {
            return None;
        }

        let mut hit_record_option = None;
        if let Some(temp_hit_record) = self.left.hit(ray, t_min, t_max) {
            hit_record_option = Some(temp_hit_record);
            t_max = temp_hit_record.t();
        }

        if let Some(temp_hit_record) = self.right.hit(ray, t_min, t_max) {
            hit_record_option = Some(temp_hit_record);
        }

        hit_record_option
    }

    fn bounding(&self) -> AABB {
        self.aabb
    }
}