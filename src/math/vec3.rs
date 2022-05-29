use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

use rand::distributions::Standard;
use rand::prelude::Distribution;

use crate::rgb::RGB;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    /// Creates new Vec3&lt;T&gt;
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(1.0, 3.0, -2.0);
    /// assert!(some_vector.x == 1.0 && some_vector.y == 3.0 && some_vector.z == -2.0);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl<T> Vec3<T>
    where T: Copy + Add<Output = T> + Mul<Output = T> {
    /// Return squared length of vector
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(1.0, 3.0, -2.0);
    /// assert_eq!(some_vector.squared_magnitude(), 14.0);
    /// ```
    pub fn squared_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns dot product of two vectors
    /// ```
    /// # use rayimg::math::Vec3;
    /// let (v1, v2) = (Vec3::new(0.0, 1.0, 2.0), Vec3::new(3.0, -4.0, 5.0));
    /// assert!(v1.dot(&v2) == 6.0 && v2.dot(&v1) == 6.0);
    /// assert!(v1.dot(&v1) == v1.squared_magnitude() && v2.dot(&v2) == v2.squared_magnitude());
    /// ```
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vec3<T>
    where T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + From<f64> {
    /// Return vector length
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(1.0, 3.0, -2.0);
    /// assert_eq!(some_vector.len(), 14.0f64.sqrt());
    /// ```
    pub fn len(&self) -> T {
        self.squared_magnitude().into().sqrt().into()
    }
    /// Returns normalized vector (i.e. vector with length 1)
    /// ```
    /// # use rayimg::math::Vec3;
    /// let non_unit_vector = Vec3::new(1.0, 2.0, 3.0);
    /// let unit_vector = non_unit_vector.normalize();
    /// let len = 14.0f64.sqrt();
    /// assert_eq!(unit_vector, Vec3::new(1.0 / len, 2.0 / len, 3.0 / len));
    /// ```
    pub fn normalize(&self) -> Vec3<T> {
        self.clone() / self.len()
    }
}

impl<T> Vec3<T> where Standard: Distribution<T> {
    pub fn random() -> Self {
        Self {
            x: rand::random(),
            y: rand::random(),
            z: rand::random()
        }
    }
}

impl<T: Into<f64>> Into<RGB> for Vec3<T> {
    fn into(self) -> RGB {
        RGB(self.x.into(), self.y.into(), self.z.into())
    }
}

impl<T: Copy + Add<Output = T>> Add<T> for Vec3<T> {
    type Output = Self;

    fn add(self, scalar: T) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar
        }
    }
}

impl<T: Copy + Add<Output = T> + Neg<Output = T>> Sub<T> for Vec3<T> {
    type Output = Self;

    fn sub(self, scalar: T) -> Self::Output {
        self + (-scalar)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl<T: Copy + Div<Output = T> + Mul<Output = T> + From<f64>> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        self * (T::from(1.0) / scalar)
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Vec3<T> {
    fn add_assign(&mut self, scalar: T) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}

impl<T: Copy + AddAssign + Neg<Output = T>> SubAssign<T> for Vec3<T> {
    fn sub_assign(&mut self, scalar: T) {
        *self += -scalar;
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T: Copy + MulAssign + From<f64> + Div<Output = T>> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, scalar: T) {
        *self *= T::from(1.0) / scalar;
    }
}

impl<T: AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: AddAssign + Neg<Output = T>> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        *self += -rhs;
    }
}