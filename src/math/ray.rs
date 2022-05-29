use super::vec3::Vec3;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3<f64>,
    direction: Vec3<f64>
}

impl Ray {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Vec3<f64> {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.direction.clone()
    }

    pub fn trace(&self, t: f64) -> Vec3<f64> {
        self.origin.clone() + self.direction.clone() * t
    }
}