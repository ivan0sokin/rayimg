use crate::{math::{Ray, Vec3}, Hit, HitRecord, Scatter, AABB};

use std::sync::Arc;

/// 2D Triangle with determined as plane between three points.
pub struct Triangle<'a> {
    vertices: [Vec3<f64>; 3],
    edges: [Vec3<f64>; 2],
    material: Arc<dyn Scatter + 'a + Send + Sync>
}

impl<'a> Triangle<'a> {
    /// Creates new `Triangle`.
    pub fn new(vertices: [Vec3<f64>; 3], material: impl Scatter + 'a + Send + Sync) -> Self {
        Self {
            vertices,
            edges: [vertices[1] - vertices[0], vertices[2] - vertices[0]],
            material: Arc::new(material)
        }
    }
}

impl<'a> Hit for Triangle<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray_direction = ray.direction();

        let h = ray_direction.cross(&self.edges[1]);
        let a = self.edges[0].dot(&h);
        if a > -f64::EPSILON && a < f64::EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertices[0];
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let q = s.cross(&self.edges[0]);
        let v = f * ray_direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * self.edges[1].dot(&q);
        if t < t_min || t_max < t {
            return None;
        }

        let mut hit_record = HitRecord::new(t, ray.trace(t));
        
        hit_record.set_face_normal(&ray, self.edges[0].cross(&self.edges[1]).normalize());
        
        if let Some(scatter) = self.material.scatter(ray, &hit_record) {
            hit_record.set_scatter(scatter);
        }
        
        Some(hit_record)
    }

    fn bounding(&self) -> AABB {
        todo!()
    }
}
