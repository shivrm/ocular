use super::{Camera, Color, Image, Pixel, Point, Ray, Texture};

pub struct Scene {
    camera: Camera,
    texture: Box<dyn Texture>,
}

impl Scene {
    pub const fn new(camera: Camera, texture: Box<dyn Texture>) -> Self {
        Self { camera, texture }
    }

    pub fn ray_color(&self, ray: Ray) -> Color {
        let (u, v) = super::texture::uv_coords(ray.direction);
        let p = Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        self.texture.color(u, v, p)
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
