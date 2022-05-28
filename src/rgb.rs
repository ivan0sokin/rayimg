use crate::{scatter::Scatter, math::Ray, hit::HitRecord};

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct RGB<T>(pub T, pub T, pub T);

impl<T: Copy> RGB<T> {
    /// Creates new RGB&lt;T&gt;
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(0.0, 1.0, 0.5);
    /// assert!(light_green.r() == 0.0 && light_green.g() == 1.0 && light_green.b() == 0.5);
    /// ```
    pub fn new(r: T, g: T, b: T) -> Self {
        RGB(r, g, b)
    }

    /// Returns red component of RGB&lt;T&gt;
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.r(), 0.39);
    /// ```
    pub fn r(&self) -> T {
        self.0
    }

    /// Returns green component of RGB&lt;T&gt;
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.g(), 0.26);
    /// ```
    pub fn g(&self) -> T {
        self.1
    }

    /// Returns blue component of RGB&lt;T&gt;
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.b(), 0.13);
    /// ```
    pub fn b(&self) -> T {
        self.2
    }

    /// Creates new RGB&lt;T&gt; from slice of 3 elements
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::from_slice(&[0.0, 1.0, 0.5]);
    /// assert!(light_green.r() == 0.0 && light_green.g() == 1.0 && light_green.b() == 0.5);
    /// ```
    pub fn from_slice(slice: &[T; 3]) -> Self {
        RGB(slice[0], slice[1], slice[2])
    }

    /// Creates slice of 3 components
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(0.0, 1.0, 0.5);
    /// assert_eq!(light_green.as_slice(), [0.0, 1.0, 0.5]);
    /// ```
    pub fn as_slice(&self) -> [T; 3] {
        [self.0, self.1, self.2]
    }
}

impl<T: Copy + Into<f64>> RGB<T> {
    const ALMOST_MAX: f64 = 256.0 - f64::EPSILON;

    /// Return slice of 3 elements converted from 0.0..1.0 interval to 0..255
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(1.0, 1.0, 0.5);
    /// assert_eq!(light_green.as_bytes(), [255, 255, 128]);
    /// ```
    pub fn as_bytes(&self) -> [u8; 3] {
        [(self.0.into() * Self::ALMOST_MAX) as u8, (self.1.into() * Self::ALMOST_MAX) as u8, (self.2.into() * Self::ALMOST_MAX) as u8]
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for RGB<T> {
    type Output = RGB<T>;

    fn mul(self, rhs: T) -> Self::Output {
        RGB(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Mul<Output = T>> Mul<RGB<T>> for RGB<T> {
    type Output = RGB<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        RGB(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: AddAssign> AddAssign<RGB<T>> for RGB<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}