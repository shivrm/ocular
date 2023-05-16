use super::{random, Camera, Color, HitRecord, Hittable, Image, Pixel, Point, Ray, Texture};

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub bounces: usize,
    pub clip_start: f32,
    pub clip_end: f32,
}

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

    pub fn ray_color(&self, ray: Ray, t_min: f32, t_max: f32, bounces: usize) -> Color {
        if bounces == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let record = self.hit(ray, t_min, t_max);

        if record.is_some() {
            let record = record.unwrap();
            let (scattered, color1) = record.material.scatter(ray, record);
            let color2 = self.ray_color(scattered, t_min, t_max, bounces - 1);

            Color::new(
                color1.x * color2.x,
                color1.y * color2.y,
                color1.z * color2.z,
            )
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

    pub fn render(&self, options: RenderOptions) -> Image {
        let mut image = Image::new(options.width, options.height);

        for y in 0..options.height {
            for x in 0..options.width {
                let mut color_sum = Color::new(0.0, 0.0, 0.0);
                for _ in 0..options.samples {
                    let frac_x = (x as f32 + random()) / (options.width as f32);
                    let frac_y = (y as f32 + random()) / (options.height as f32);

                    let ray = self.camera.ray(frac_x, frac_y);
                    let color =
                        self.ray_color(ray, options.clip_start, options.clip_end, options.bounces);
                    color_sum = color_sum + color;
                }
                color_sum = color_sum / (options.samples as f32);
                image.set_pixel(x, y, Pixel::from_color(color_sum));
            }
        }
        image
    }
}
