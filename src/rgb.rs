use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign};

/// Color structure with components in interval `0.0..=1.0`.
/// ```
/// use rayimg::RGB;
///
/// let light_green = RGB(0.0, 1.0, 0.5);
/// let brown = RGB::new(0.39, 0.26, 0.13);
///
/// assert_eq!(light_green.as_bytes(), [0, 255, 128]);
/// assert_eq!(brown.as_bytes(), [99, 66, 33]);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RGB(pub f64, pub f64, pub f64);

impl RGB {
    const ALMOST_BYTE_MAX: f64 = 256.0 - f64::EPSILON;

    /// Creates new `RGB`.
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(0.0, 1.0, 0.5);
    /// assert!(light_green.r() == 0.0 && light_green.g() == 1.0 && light_green.b() == 0.5);
    /// ```
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGB(r, g, b)
    }

    /// Returns red component of color.
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.r(), 0.39);
    /// ```
    pub fn r(&self) -> f64 {
        self.0
    }

    /// Returns green component of color.
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.g(), 0.26);
    /// ```
    pub fn g(&self) -> f64 {
        self.1
    }

    /// Returns blue component color.
    /// ```
    /// # use rayimg::RGB;
    /// let brown = RGB::new(0.39, 0.26, 0.13);
    /// assert_eq!(brown.b(), 0.13);
    /// ```
    pub fn b(&self) -> f64 {
        self.2
    }

    /// Corrects color based on gamma value, i.e. raises each component to the power camera/gamma.
    /// ```
    /// # use rayimg::RGB;
    /// let some_color = RGB(0.01, 0.25, 0.64);
    /// assert_eq!(some_color.correct_gamma(2.0), RGB(0.1, 0.5, 0.8));
    /// ```
    pub fn correct_gamma(&self, gamma: f64) -> RGB {
        let one_over_gamma = 1.0 / gamma;
        Self(self.0.powf(one_over_gamma), self.1.powf(one_over_gamma), self.2.powf(one_over_gamma))
    }

    /// Return slice of 3 elements converted from `0.0..=1.0` to `0..=255`.
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(1.0, 1.0, 0.5);
    /// assert_eq!(light_green.as_bytes(), [255, 255, 128]);
    /// ```
    pub fn as_bytes(&self) -> [u8; 3] {
        [(self.0 * Self::ALMOST_BYTE_MAX) as u8, (self.1 * Self::ALMOST_BYTE_MAX) as u8, (self.2 * Self::ALMOST_BYTE_MAX) as u8]
    }
}

impl Add<f64> for RGB {
    type Output = Self;

    fn add(self, scalar: f64) -> Self::Output {
        Self(self.0 + scalar, self.1 + scalar, self.2 + scalar)
    }
}

impl Sub<f64> for RGB {
    type Output = Self;

    fn sub(self, scalar: f64) -> Self::Output {
        self + (-scalar)
    }
}

impl Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Div<f64> for RGB {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

impl Add<RGB> for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<RGB> for RGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<RGB> for RGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Neg for RGB {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl AddAssign<RGB> for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign<RGB> for RGB {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl MulAssign<RGB> for RGB {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
