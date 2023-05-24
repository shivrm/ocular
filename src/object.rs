use super::{Material, Point, Ray, Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub point: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f32,
    pub uv: (f32, f32),
    pub material: &'a dyn Material,
}

pub struct Sphere {
    center: Point,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn normal_at(&self, point: Point) -> Vec3 {
        return (point - self.center) / self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let d = (half_b * half_b) - (a * c);

        if d < 0.0 {
            return None;
        }

        let t = (-half_b - d.sqrt()) / a;
        let t = if t < t_min || t > t_max {
            let t = (-half_b + d.sqrt()) / a;
            if t < t_min || t > t_max {
                return None;
            }
            t
        } else {
            t
        };

        let point = ray.at(t);
        let normal = self.normal_at(point);
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let uv = super::texture::uv_coords(point - self.center);

        Some(HitRecord {
            point,
            normal,
            front_face,
            t,
            uv,
            material: &*self.material,
        })
    }
}
