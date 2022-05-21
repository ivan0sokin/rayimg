#[derive(Default, Clone, Eq, PartialEq)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T
}

impl<T: Copy> RGB<T> {
    /// Creates new RGB&lt;T&gt;
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(0.0, 1.0, 0.5);
    /// assert!(light_green.r == 0.0 && light_green.g == 1.0 && light_green.b == 0.5);
    /// ```
    pub fn new(r: T, g: T, b: T) -> Self {
        Self {
            r,
            g,
            b
        }
    }

    /// Creates new RGB&lt;T&gt; from slice of 3 elements
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::from_slice(&[0.0, 1.0, 0.5]);
    /// assert!(light_green.r == 0.0 && light_green.g == 1.0 && light_green.b == 0.5);
    /// ```
    pub fn from_slice(slice: &[T; 3]) -> Self {
        Self {
            r: slice[0],
            g: slice[1],
            b: slice[2]
        }
    }

    /// Creates slice of 3 components
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(0.0, 1.0, 0.5);
    /// assert_eq!(light_green.as_slice(), [0.0, 1.0, 0.5]);
    /// ```
    pub fn as_slice(&self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl RGB<f64> {
    const ALMOST_MAX: f64 = 256.0 - f64::EPSILON;

    /// Return slice of 3 elements converted from 0.0..1.0 interval to 0..255
    /// ```
    /// # use rayimg::RGB;
    /// let light_green = RGB::new(1.0, 1.0, 0.5);
    /// assert_eq!(light_green.as_bytes(), [255, 255, 128]);
    /// ```
    pub fn as_bytes(&self) -> [u8; 3] {
        [(self.r * Self::ALMOST_MAX) as u8, (self.g * Self::ALMOST_MAX) as u8, (self.b * Self::ALMOST_MAX) as u8]
    }
}