use crate::{math::Ray, hit::{Hit, HitRecord}};

use std::{rc::Rc, ops::{Add, Sub, Mul, Div, Neg}};

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
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let mut test_scene = Scene::new();
    /// test_scene.add_object(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Rc::new(Lambertian::default())));
    /// assert_eq!(test_scene.object_count(), 1);
    /// ```
    pub fn add_object(&mut self, object: impl Hit<T> + 'a) {
        self.objects.push(Rc::new(object));
    }

    /// Returns count of objects
    /// ```
    /// # use rayimg::{Scene, shapes::Sphere, math::Vec3, materials::Lambertian};
    /// # use std::rc::Rc;
    /// let mut test_scene = Scene::new();
    /// let unit_sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Rc::new(Lambertian::default()));
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// test_scene.add_object(unit_sphere.clone());
    /// assert_eq!(test_scene.object_count(), 3);
    /// ```
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl<'a, T> Hit<T> for Scene<'a, T>
    where T: Copy + PartialOrd + Default + Neg + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + From<f64> + Into<f64> {
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
