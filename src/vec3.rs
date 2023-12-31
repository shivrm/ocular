use super::random;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - (2.0 * normal * self.dot(&normal))
    }

    pub fn refract(&self, normal: Vec3, ri: f32) -> Vec3 {
        let cos_theta = f32::min(-self.dot(&normal), 1.0);
        let r_perp = (*self + (normal * cos_theta)) * ri;
        let r_para = normal * -((1.0 - r_perp.dot(&r_perp)).abs().sqrt());
        r_perp + r_para
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-7;
        (self.x.abs() < epsilon) && (self.y.abs() < epsilon) && (self.z.abs() < epsilon)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::new(random(-1.0, 1.0), random(-1.0, 1.0), random(-1.0, 1.0));
            if v.len() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let v = Vec3::new(random(-1.0, 1.0), random(-1.0, 1.0), 0.0);
            if v.len() < 1.0 {
                return v;
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}
