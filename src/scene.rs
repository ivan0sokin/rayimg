use crate::{math::Ray, hit::{Hit, HitRecord}};

use std::sync::Arc;

/// Scene contains information about hittable objects. It's also hittable.
/// ```
/// use rayimg::{Scene, shapes::Sphere, materials::Lambertian, math::{Vec3, Ray}, RGB, Hit, HitRecord};
///
/// let mut scene = Scene::new();
/// scene.add_object(Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0, Lambertian::new(RGB(1.0, 0.0, 0.0))));
///
/// assert_eq!(scene.object_count(), 1);
/// assert!(scene.hit(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0)), 0.0, f64::MAX).is_none());
/// ```
#[derive(Clone)]
pub struct Scene<'a> {
    objects: Vec<Arc<dyn Hit + 'a + Send + Sync>>
}

impl<'a> Scene<'a> {
    /// Creates empty Scene
    /// ```
    /// # use rayimg::Scene;
    /// let test_scene = Scene::new();
    /// assert_eq!(test_scene.object_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    /// Adds object to scene
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3, materials::Lambertian, RGB};
    /// let mut test_scene = Scene::new();
    /// test_scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Lambertian::new(RGB::default())));
    /// assert_eq!(test_scene.object_count(), 1);
    /// ```
    pub fn add_object(&mut self, object: impl Hit + 'a + Send + Sync) {
        self.objects.push(Arc::new(object));
    }

    /// Returns count of objects
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3, materials::Lambertian, RGB};
    /// let mut test_scene = Scene::new();
    /// let unit_sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Lambertian::new(RGB::default()));
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// assert_eq!(test_scene.object_count(), 3);
    /// ```
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl<'a> Hit for Scene<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(ray, t_min, t_max) {
                if temp_hit_record.t() < t_max && temp_hit_record.t() > t_min {
                    t_max = temp_hit_record.t();
                    hit_record = Some(temp_hit_record);
                }
            }
        }
        
        hit_record
    }
}
