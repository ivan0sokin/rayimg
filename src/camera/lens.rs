use crate::math::Vec3;

#[derive(Clone)]
pub struct Lens {
    radius: f64,
    u: Vec3<f64>,
    v: Vec3<f64>
}

impl Lens {
    pub fn new(radius: f64, u: Vec3<f64>, v: Vec3<f64>) -> Self {
        Self {
            radius,
            u,
            v
        }
    }

    pub fn random_offset(&self) -> Vec3<f64> {
        let p = self.random_point();
        self.u.clone() * p.x + self.v.clone() * p.y
    }

    fn random_point(&self) -> Vec3<f64> {
        Vec3::random_in_unit_disk() * self.radius
    }
}