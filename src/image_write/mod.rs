mod p3_image_writer;
mod pixels;

pub use p3_image_writer::P3ImageWriter;
pub use pixels::Pixels;

pub trait ImageWrite<C> {
    fn write_header(&mut self);
    fn write_color(&mut self, color: &C);
    fn bounds(&self) -> (usize, usize);
    fn pixels(&self) -> Pixels;
}