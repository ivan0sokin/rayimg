use super::pixels::Pixels;

/// `ImageWrite` defines pixel order starting from upper left corner and image size.
pub trait ImageWrite<C> {
    fn write_header(&mut self);
    fn write_color(&mut self, color: &C);
    fn bounds(&self) -> (usize, usize);
    fn pixels(&self) -> Pixels;
}

impl<C, T> ImageWrite<C> for &mut T where T: ImageWrite<C> {
    fn write_header(&mut self) {
        (**self).write_header()
    }

    fn write_color(&mut self, color: &C) {
        (**self).write_color(color)
    }

    fn bounds(&self) -> (usize, usize) {
        (**self).bounds()
    }

    fn pixels(&self) -> Pixels {
        (**self).pixels()
    }
}