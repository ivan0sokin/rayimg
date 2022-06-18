use crate::{math::{Vec3, Ray}, Hit, HitRecord, Scatter};

use std::rc::Rc;

/// 2D Triangle with determined as plane between three points.
pub struct Triangle {
    vertices: [Vec3<f64>; 3],
    material: Rc<dyn Scatter>
}

impl Triangle {
    /// Creates new `Triangle`.
    pub fn new(vertices: [Vec3<f64>; 3], material: Rc<dyn Scatter>) -> Self {
        Self {
            vertices,
            material
        }
    }
}

impl Hit for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edges = (self.vertices[1].clone() - self.vertices[0].clone(), self.vertices[2].clone() - self.vertices[0].clone());
        let h = ray.direction().cross(&edges.1);
        let a = edges.0.dot(&h);
        if a > -f64::EPSILON && a < f64::EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertices[0].clone();
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let q = s.cross(&edges.0);
        let v = f * ray.direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edges.1.dot(&q);
        if t < t_min || t_max < t {
            return None;
        }

        let mut hit_record = HitRecord::new(t, ray.trace(t), self.material.clone());
        hit_record.set_face_normal(&ray, edges.0.cross(&edges.1).normalize());
        Some(hit_record)
    }
}
