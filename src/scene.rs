use super::{Camera, Color, Image, Pixel, Ray};

pub struct Scene {
    camera: Camera,
}

impl Scene {
    pub const fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn ray_color(&self, ray: Ray) -> Color {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
    }

    pub fn render(&self, width: usize, height: usize) -> Image {
        let mut image = Image::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let frac_x = (x as f32) / (width as f32);
                let frac_y = (y as f32) / (height as f32);

                let ray = self.camera.ray(frac_x, frac_y);
                let color = self.ray_color(ray);
                image.set_pixel(x, y, Pixel::from_color(color));
            }
        }
        image
    }
}
