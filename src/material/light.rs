use super::Material;
use crate::{Color, HitRecord, Point, Ray, Texture};

pub struct Light {
    texture: Box<dyn Texture>,
}

impl Light {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: Ray, _hit_record: HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emit(&self, u: f32, v: f32, point: Point) -> Color {
        self.texture.color(u, v, point)
    }
}
