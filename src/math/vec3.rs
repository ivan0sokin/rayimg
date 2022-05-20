#[derive(Default, Clone, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    /// Creates new Vec3&lt;T&gt;
    /// ```
    /// # use rayimg::Vec3;
    /// let some_vector = Vec3::new(1.0, 3.0, -2.0);
    /// assert!(some_vector.x == 1.0 && some_vector.y == 3.0 && some_vector.z == -2.0);
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}