use super::ImageWrite;
use crate::rgb::RGB;

use std::io::Write;
use crate::image_write::Pixels;

pub struct P3ImageWriter<W> {
    bounds: (usize, usize),
    w: W
}

impl<W: Write> P3ImageWriter<W> {
    /// Creates new P3ImageWriter
    /// ```
    /// # use rayimg::{P3ImageWriter, ImageWrite, RGB};
    /// # use std::io::Write;
    /// let mut buf = Vec::new();
    /// let mut image_writer = P3ImageWriter::new((640, 480), &mut buf);
    /// image_writer.write_header();
    /// image_writer.write_color(&RGB::new(0, 128, 255));
    /// assert_eq!(String::from_utf8(buf).unwrap(), String::from("P3\n640 480\n255\n0 128 255\n"));
    /// ```
    pub fn new(bounds: (usize, usize), w: W) -> Self {
        Self {
            bounds,
            w
        }
    }
}

impl<W: Write> ImageWrite<RGB<u8>> for P3ImageWriter<W> {
    fn write_header(&mut self) {
        write!(self.w, "P3\n{} {}\n255\n", self.bounds.0, self.bounds.1).unwrap();
    }

    fn write_color(&mut self, color: &RGB<u8>) {
        write!(self.w, "{} {} {}\n", color.r, color.g, color.b).unwrap();
    }

    fn bounds(&self) -> (usize, usize) {
        self.bounds
    }

    fn pixels(&self) -> Pixels {
        let mut current = (1, self.bounds.1);
        Pixels::new(move || {
            if current.1 == 0 {
                None
            } else {
                let next = (current.0 - 1, current.1 - 1);
                current = if current.0 == self.bounds.0 { (1, current.1 - 1) } else { (current.0 + 1, current.1) };
                Some(next)
            }
        })
    }
}
