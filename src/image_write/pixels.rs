/// Pixel iterating object.
pub struct Pixels<'a> {
    next: Box<dyn FnMut() -> Option<(usize, usize)> + 'a>
}

impl<'a> Pixels<'a> {
    /// Creates new Pixels struct.
    /// ```
    /// # use rayimg::Pixels;
    /// let (bounds, mut cur) = ((9, 9), (0, 0));
    /// let mut pixels = Pixels::new(|| {
    ///     if cur == bounds {
    ///         None
    ///     } else {
    ///         let res = cur;
    ///         cur = if cur.0 == bounds.0 { (0, cur.1 + 1) } else { (cur.0 + 1, cur.1 ) };
    ///         Some(res)
    ///     }
    /// });
    ///
    /// assert_eq!(pixels.next(), Some((0, 0)));
    /// assert_eq!(pixels.next(), Some((1, 0)));
    /// ```
    pub fn new(next: impl FnMut() -> Option<(usize, usize)> + 'a) -> Self {
        Self {
            next: Box::new(next)
        }
    }
}

impl<'a> Iterator for Pixels<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        (self.next)()
    }
}