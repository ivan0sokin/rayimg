use crate::{hit::{Hit, HitRecord}, math::{Vec3, Ray}};

#[derive(Clone)]
pub struct Sphere<T> {
    center: Vec3<T>,
    radius: T,
    // material: Box<dyn Material + 'a>
}

impl<T: Copy> Sphere<T> {
    /// Creates new Sphere with given center, radius and material
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3};
    /// let unit_sphere = Sphere::new(Vec3::new(1.0, 3.0, 2.0), 1.0);
    /// assert!(unit_sphere.center() == Vec3::new(1.0, 3.0, 2.0) && unit_sphere.radius() == 1.0);
    /// ```
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self {
            center,
            radius
        }
    }

    pub fn center(&self) -> Vec3<T> {
        self.center.clone()
    }

    pub fn radius(&self) -> T {
        self.radius
    }
}

impl<T> Hit<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        todo!()
    }
}
