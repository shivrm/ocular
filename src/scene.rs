use super::{Camera, Color, HitRecord, Hittable, Image, Pixel, Point, Ray, Texture};

pub struct Scene {
    camera: Camera,
    texture: Box<dyn Texture>,
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub const fn new(
        camera: Camera,
        texture: Box<dyn Texture>,
        objects: Vec<Box<dyn Hittable>>,
    ) -> Self {
        Self {
            camera,
            texture,
            objects,
        }
    }

    pub fn ray_color(&self, ray: Ray) -> Color {
        let record = self.hit(ray, 0.001, f32::INFINITY);

        if record.is_some() {
            let record = record.unwrap();
            let (_, color) = record.material.scatter(ray, record);
            color
        } else {
            let (u, v) = super::texture::uv_coords(ray.direction);
            let p = Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            self.texture.color(u, v, p)
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut hit_t = t_max;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, t_max) {
                Some(record) => {
                    if record.t < hit_t && record.t > t_min {
                        hit_t = record.t;
                        hit_record = Some(record);
                    }
                }
                None => (),
            }
        }

        hit_record
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
