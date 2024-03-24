use crate::RGB;

use super::ImageWrite;

use std::io::Write;

/// Writes bytes (i.e. `[u8; 3]`) to *.ppm files.
pub struct P3ImageWriter<W> {
    bounds: (usize, usize),
    w: W
}

impl<W: Write> P3ImageWriter<W> {
    /// Creates new P3ImageWriter.
    /// ```
    /// # use rayimg::{P3ImageWriter, ImageWrite, RGB};
    /// # use std::io::Write;
    /// let mut buf = Vec::new();
    /// let mut image_writer = P3ImageWriter::new((640, 480), &mut buf);
    /// image_writer.write_image_data(&[[0, 128, 255]]);
    /// assert_eq!(String::from_utf8(buf).unwrap(), String::from("P3\n640 480\n255\n0 128 255\n"));
    /// ```
    pub fn new(bounds: (usize, usize), w: W) -> Self {
        let mut writer = Self {
            bounds,
            w
        };

        writer.write_header();
        writer
    }

    fn write_header(&mut self) {
        write!(self.w, "P3\n{} {}\n255\n", self.bounds.0, self.bounds.1).unwrap();
    }
}

impl<W: Write> ImageWrite for P3ImageWriter<W> {
    fn write_all(&mut self, colors: &[RGB]) {
        for color in colors {
            let color = color.as_bytes();
            write!(self.w, "{} {} {}\n", color[0], color[1], color[2]).unwrap();
        }
    }

    fn bounds(&self) -> (usize, usize) {
        self.bounds
    }
}
