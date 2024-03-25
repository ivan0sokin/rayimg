use crate::math::Vec3;

#[derive(Clone)]
pub struct Lens {
    u: Vec3<f64>,
    v: Vec3<f64>
}

impl Lens {
    pub fn new(u: Vec3<f64>, v: Vec3<f64>) -> Self {
        Self {
            u,
            v
        }
    }

    pub fn random_offset(&self) -> Vec3<f64> {
        let p = Vec3::random_in_unit_disk();
        self.u * p.x + self.v * p.y
    }
}