use super::{HitRecord, Hittable, Material, Point, Ray, Vec3};
use obj::Obj;

pub struct TrigHitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f32,
    pub uv: (f32, f32),
}

pub struct Trig {
    v1: Point,
    v2: Point,
    v3: Point,
}

impl Trig {
    pub fn new(v1: Point, v2: Point, v3: Point) -> Self {
        Self { v1, v2, v3 }
    }

    pub fn hit(&self, ray: Ray) -> Option<TrigHitRecord> {
        let edge1 = self.v2 - self.v1;
        let edge2 = self.v3 - self.v1;

        let (p, d) = (ray.origin, ray.direction);
        let h = d.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = p - self.v1;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * d.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        let point = p + (t * d);
        let normal = edge1.cross(&edge2);
        let front_face = d.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        let hit_record = TrigHitRecord {
            point,
            normal,
            front_face,
            t,
            uv: (u, v),
        };
        Some(hit_record)
    }
}

pub struct Mesh {
    vertices: Vec<Point>,
    faces: Vec<[u32; 3]>,
    center: Point,
    material: Box<dyn Material>,
    aabb: (Vec3, Vec3),
}

impl Mesh {
    pub fn new(
        vertices: Vec<Point>,
        faces: Vec<[u32; 3]>,
        center: Point,
        material: Box<dyn Material>,
    ) -> Self {
        // Calculate AABB

        let (mut x_min, mut y_min, mut z_min) = (f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let (mut x_max, mut y_max, mut z_max) =
            (f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for v in vertices.iter() {
            x_min = x_min.min(v.x);
            y_min = y_min.min(v.y);
            z_min = z_min.min(v.z);
            x_max = x_max.max(v.x);
            y_max = y_max.max(v.y);
            z_max = z_max.max(v.z);
        }

        Self {
            vertices,
            faces,
            center,
            material,
            aabb: (
                Vec3::new(x_min, y_min, z_min),
                Vec3::new(x_max, y_max, z_max),
            ),
        }
    }

    pub fn from_obj(
        obj: Obj<obj::TexturedVertex, u32>,
        center: Point,
        material: Box<dyn Material>,
    ) -> Self {
        let mut vertices: Vec<Point> = Vec::new();

        for v in obj.vertices {
            let point = Point::new(v.position[0], v.position[1], v.position[2]);
            vertices.push(point);
        }

        let mut faces: Vec<[u32; 3]> = Vec::new();

        for indices in obj.indices.chunks_exact(3) {
            faces.push([indices[0], indices[1], indices[2]]);
        }

        Self::new(vertices, faces, center, material)
    }

    pub fn hit_aabb(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        let (a, b) = self.aabb;

        let rmin = a - ray.origin;
        let rmax = b - ray.origin;
        let d = ray.direction;

        let t0 = f32::min(rmin.x / d.x, rmax.x / d.x);
        let t1 = f32::max(rmin.x / d.x, rmax.x / d.x);
        if f32::max(t0, t_min) >= f32::min(t1, t_max) {
            return false;
        }

        let t0 = f32::min(rmin.y / d.y, rmax.y / d.y);
        let t1 = f32::max(rmin.y / d.y, rmax.y / d.y);
        if f32::max(t0, t_min) >= f32::min(t1, t_max) {
            return false;
        }

        let t0 = f32::min(rmin.z / d.z, rmax.z / d.z);
        let t1 = f32::max(rmin.z / d.z, rmax.z / d.z);
        if f32::max(t0, t_min) >= f32::min(t1, t_max) {
            return false;
        }

        return true;
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<TrigHitRecord> = None;
        let mut hit_t = t_max;

        let transformed_ray = Ray::new(ray.origin - self.center, ray.direction);

        if !self.hit_aabb(transformed_ray, t_min, t_max) {
            return None;
        }

        for face in self.faces.iter() {
            let trig = Trig::new(
                self.vertices[face[0] as usize],
                self.vertices[face[1] as usize],
                self.vertices[face[2] as usize],
            );

            match trig.hit(transformed_ray) {
                Some(record) => {
                    if record.t < hit_t && record.t > t_min {
                        hit_t = record.t;
                        hit_record = Some(record);
                    }
                }
                None => (),
            }
        }

        if let Some(r) = hit_record {
            Some(HitRecord {
                point: r.point + self.center,
                normal: r.normal,
                front_face: r.front_face,
                t: r.t,
                uv: r.uv,
                material: &*self.material,
            })
        } else {
            None
        }
    }
}
