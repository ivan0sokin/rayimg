use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
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

    /// Return squared length of vector
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(1.0, 3.0, -2.0);
    /// assert_eq!(some_vector.squared_magnitude(), 14.0);
    /// ```
    pub fn squared_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

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

    /// Returns half of vector
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(2.0, 4.0, -6.0);
    /// let half_vector = some_vector.half();
    /// assert_eq!(half_vector, Vec3::new(1.0, 2.0, -3.0));
    /// ```
    pub fn half(&self) -> Vec3<T> {
        Vec3::new((self.x.into() * 0.5).into(), (self.y.into() * 0.5).into(), (self.z.into() * 0.5).into())
    }

    /// Creates slice of 3 components
    /// ```
    /// # use rayimg::math::Vec3;
    /// let some_vector = Vec3::new(0.0, 1.0, 0.5);
    /// assert_eq!(some_vector.as_slice(), [0.0, 1.0, 0.5]);
    /// ```
    pub fn as_slice(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> Add<T> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn add(self, scalar: T) -> Self::Output {
        Vec3::new(self.x + scalar, self.y + scalar, self.z + scalar)
    }
}

impl<T> Sub<T> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn sub(self, scalar: T) -> Self::Output {
        Vec3::new(self.x - scalar, self.y - scalar, self.z - scalar)
    }
}

impl<T> Mul<T> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl<T> Div<T> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn div(self, scalar: T) -> Self::Output {
        let one_over_scalar = T::from(1.0) / scalar;
        Vec3::new(self.x * one_over_scalar, self.y * one_over_scalar, self.z * one_over_scalar)
    }
}

impl<T> Add<Vec3<T>> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub<Vec3<T>> for Vec3<T>
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64> + Into<f64> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}