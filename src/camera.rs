use super::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point,
    screen_bottom_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point, vw: f32, vh: f32, focal_length: f32) -> Self {
        let horizontal = Vec3::new(vw, 0.0, 0.0);
        let vertical = Vec3::new(0.0, vh, 0.0);

        let screen_bottom_left =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            screen_bottom_left,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, frac_x: f32, frac_y: f32) -> Ray {
        let target =
            self.screen_bottom_left + (self.horizontal * frac_x) + (self.vertical * frac_y);

        Ray::new(self.origin, target - self.origin)
    }
}
