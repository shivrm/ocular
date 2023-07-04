use super::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point,
    screen_top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let screen_top_left = origin - (horizontal / 2.0) + (vertical / 2.0) - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            screen_top_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray(&self, frac_x: f32, frac_y: f32) -> Ray {
        let blur = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * blur.x + self.v * blur.y;
        let target = self.screen_top_left + (self.horizontal * frac_x) - (self.vertical * frac_y);

        Ray::new(self.origin + offset, target - self.origin - offset)
    }
}
