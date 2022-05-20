use crate::object::{Object, Material};
use crate::math::Vec3;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    // material: Box<dyn Material + 'a>
}

impl Sphere {
    /// Creates new Sphere with given center, radius and material
    /// ```
    /// # use rayimg::{shapes::Sphere, Vec3};
    /// let unit_sphere = Sphere::new(Vec3::new(1.0, 3.0, 2.0), 1.0);
    /// assert!(unit_sphere.center == Vec3::new(1.0, 3.0, 2.0) && unit_sphere.radius == 1.0);
    /// ```
    pub fn new(center: Vec3<f64>, radius: f64) -> Self {
        Self {
            center,
            radius,
            // material: Box::new(PhantomData::default())
        }
    }
}

impl Object for Sphere {
    /*fn get_material(&self) -> &dyn Material {
        *self.material
    }*/
}