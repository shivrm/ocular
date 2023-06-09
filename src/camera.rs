use super::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point,
    screen_top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        let screen_top_left = origin - (horizontal / 2.0) + (vertical / 2.0) - w;

        Self {
            origin,
            screen_top_left,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, frac_x: f32, frac_y: f32) -> Ray {
        let target = self.screen_top_left + (self.horizontal * frac_x) - (self.vertical * frac_y);

        Ray::new(self.origin, target - self.origin)
    }
}
