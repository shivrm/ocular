use super::{Color, HitRecord, Ray, Texture, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> (Ray, Color);
}

pub struct Diffuse {
    texture: Box<dyn Texture>,
}

impl Diffuse {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit_record: HitRecord) -> (Ray, Color) {
        let (u, v) = hit_record.uv;
        let color = self.texture.color(u, v, hit_record.point);

        let target = hit_record.normal + Vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, target);

        (scattered, color)
    }
}

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

pub struct Glass {
    refractive_index: f32,
}

impl Glass {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Material for Glass {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> (Ray, Color) {
        let color = Color::new(1.0, 1.0, 1.0);

        let ri_inverse = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction.unit();
        let target = unit_direction.refract(hit_record.normal, ri_inverse);
        let scattered = Ray::new(hit_record.point, target);
        (scattered, color)
    }
}
