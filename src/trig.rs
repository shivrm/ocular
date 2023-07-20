use super::{HitRecord, Hittable, Material, Point, Ray, Vec3};

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
    trigs: Vec<Trig>,
    material: Box<dyn Material>,
}

impl Mesh {
    pub fn new(trigs: Vec<Trig>, material: Box<dyn Material>) -> Self {
        Self { trigs, material }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<TrigHitRecord> = None;
        let mut hit_t = t_max;

        for trig in self.trigs.iter() {
            match trig.hit(ray) {
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
                point: r.point,
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
