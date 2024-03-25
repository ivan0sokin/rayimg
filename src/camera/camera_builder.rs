use {super::{Camera, lens::Lens}, crate::math::Vec3};

/// `CameraBuilder` builds camera with given parameters.
pub struct CameraBuilder {
    pub(super) position: Vec3<f64>,
    pub(super) target: Vec3<f64>,
    pub(super) up: Vec3<f64>,
    pub(super) vertical_fov: f64,
    pub(super) aspect_ratio: f64,
    pub(super) defocus_angle: f64,
    pub(super) focus_distance: f64
}

impl CameraBuilder {
    /// Sets `Camera` position.
    pub fn position(mut self, position: Vec3<f64>) -> Self {
        self.position = position;
        self
    }

    /// Sets `Camera` target.
    pub fn target(mut self, target: Vec3<f64>) -> Self {
        self.target = target;
        self
    }

    /// Sets `Camera` up vector.
    pub fn up(mut self, up: Vec3<f64>) -> Self {
        self.up = up;
        self
    }

    /// Sets `Camera` vertical fov in **degrees**.
    pub fn vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    /// Sets `Camera` viewport aspect_ratio.
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Sets `Camera` defocus angle in **degrees**.
    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    /// Sets `Camera` focus distance;
    pub fn focus_distance(mut self, focus_distance: f64) -> Self {
        self.focus_distance = focus_distance;
        self
    }

    /// Returns built `Camera`.
    pub fn build(self) -> Camera {
        let h = (self.vertical_fov * 0.5).to_radians().tan();
        let height = 2.0 * h * self.focus_distance;
        let width = self.aspect_ratio * height;

        let w = (self.position - self.target).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);
        let (horizontal, vertical) = (u * width, v * height);

        let defocus_radius = (self.defocus_angle * 0.5).to_radians().tan() * self.focus_distance;

        Camera {
            position: self.position,
            upper_left_corner: self.position - horizontal * 0.5 + vertical * 0.5 - w * self.focus_distance,
            horizontal,
            minus_vertical: -vertical,
            lens: Lens::new(u * defocus_radius, v * defocus_radius)
        }
    }
}