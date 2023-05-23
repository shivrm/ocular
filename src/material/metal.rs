use super::Material;
use crate::{Color, HitRecord, Ray, Texture, Vec3};

pub struct Metal {
    texture: Box<dyn Texture>,
    roughness: f32,
}

impl Metal {
    pub fn new(texture: Box<dyn Texture>, roughness: f32) -> Self {
        Self { texture, roughness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> (Ray, Color) {
        let (u, v) = hit_record.uv;
        let color = self.texture.color(u, v, hit_record.point);

        let target = ray.direction.reflect(hit_record.normal);
        let random = self.roughness * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, target + random);

        (scattered, color)
    }
}
