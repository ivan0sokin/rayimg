
/// `ImageWrite` defines pixel order starting from upper left corner and image size.
pub trait ImageWrite {
    /// User-defined color type.
    type Color;
    /// Partial or full data write starting from upper left corner.
    fn write_image_data(&mut self, data: &[Self::Color]);
    /// Image width and height.
    fn bounds(&self) -> (usize, usize);
}

impl<T, C> ImageWrite for &mut T where T: ImageWrite<Color = C> {
    type Color = C;

    fn write_image_data(&mut self, data: &[Self::Color]) {
        (**self).write_image_data(data)
    }

    fn bounds(&self) -> (usize, usize) {
        (**self).bounds()
    }
}