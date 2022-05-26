use std::rc::Rc;

use crate::{math::Ray, hit::{Hit, HitRecord}};

#[derive(Clone)]
pub struct Scene<'a, T> {
    objects: Vec<Rc<dyn Hit<T> + 'a>>
}

impl<'a, T> Scene<'a, T> {
    /// Creates empty Scene
    /// ```
    /// # use rayimg::Scene;
    /// let test_scene = Scene::<f64>::new();
    /// assert_eq!(test_scene.object_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    /// Adds object to scene
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3};
    /// let mut test_scene = Scene::new();
    /// test_scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0));
    /// assert_eq!(test_scene.object_count(), 1);
    /// ```
    pub fn add_object(&mut self, object: impl Hit<T> + 'a) {
        self.objects.push(Rc::new(object));
    }

    /// Returns count of objects
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3};
    /// let mut test_scene = Scene::new();
    /// let unit_sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// assert_eq!(test_scene.object_count(), 3);
    /// ```
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl<'a, T: Copy + PartialOrd> Hit<T> for Scene<'a, T> {
    fn hit(&self, ray: &Ray<T>, t_min: T, mut t_max: T) -> Option<HitRecord<T>> {
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
