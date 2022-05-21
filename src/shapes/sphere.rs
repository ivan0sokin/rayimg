use crate::object::{Object, Material};
use crate::math::Vec3;

#[derive(Clone)]
pub struct Sphere<T> {
    pub center: Vec3<T>,
    pub radius: T,
    // material: Box<dyn Material + 'a>
}

impl<T> Sphere<T> {
    /// Creates new Sphere with given center, radius and material
    /// ```
    /// # use rayimg::{shapes::Sphere, math::Vec3};
    /// let unit_sphere = Sphere::new(Vec3::new(1.0, 3.0, 2.0), 1.0);
    /// assert!(unit_sphere.center == Vec3::new(1.0, 3.0, 2.0) && unit_sphere.radius == 1.0);
    /// ```
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self {
            center,
            radius,
            // material: Box::new(PhantomData::default())
        }
    }
}

impl<T> Object for Sphere<T> {
    /*fn get_material(&self) -> &dyn Material {
        *self.material
    }*/
}