use super::Material;
use crate::{Color, HitRecord, Ray, Texture, Vec3};

pub struct Diffuse {
    texture: Box<dyn Texture>,
}

impl Diffuse {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
        let (u, v) = hit_record.uv;
        let color = self.texture.color(u, v, hit_record.point);

        let mut target = hit_record.normal + Vec3::random_in_unit_sphere();

        if target.near_zero() {
            target = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, target);

        Some((scattered, color))
    }
}
