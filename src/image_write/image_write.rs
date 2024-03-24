use crate::RGB;

/// `ImageWrite` defines pixel order starting from upper left corner and image size.
pub trait ImageWrite {
    /// Partial or full data write starting from upper left corner.
    fn write_all(&mut self, colors: &[RGB]);
    /// Image width and height.
    fn bounds(&self) -> (usize, usize);
}

impl<T> ImageWrite for &mut T where T: ImageWrite {
    fn write_all(&mut self, colors: &[RGB]) {
        (**self).write_all(colors)
    }

    fn bounds(&self) -> (usize, usize) {
        (**self).bounds()
    }
}