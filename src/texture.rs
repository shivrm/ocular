use super::{Color, Point};

pub trait Texture: Send + Sync {
    fn color(&self, u: f32, v: f32, p: Point) -> Color;
}

pub struct Sky;

impl Texture for Sky {
    fn color(&self, _u: f32, v: f32, _p: Point) -> Color {
        ((1.0 - v) * Color::new(1.0, 1.0, 1.0)) + (v * Color::new(0.5, 0.7, 1.0))
    }
}

pub fn uv_coords(p: Point) -> (f32, f32) {
    let unit = p.unit();

    let theta = (-unit.y).acos();
    let phi = f32::atan2(-unit.z, unit.x) + std::f32::consts::PI;

    let u = phi / std::f32::consts::TAU;
    let v = theta / std::f32::consts::PI;
    (u, v)
}

pub struct Solid {
    color: Color,
}

impl Solid {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for Solid {
    fn color(&self, _u: f32, _v: f32, _p: Point) -> Color {
        self.color
    }
}

pub struct Checkered {
    color1: Color,
    color2: Color,
    size: f32,
}

impl Checkered {
    pub const fn new(color1: Color, color2: Color, size: f32) -> Self {
        Self {
            color1,
            color2,
            size,
        }
    }
}

impl Texture for Checkered {
    fn color(&self, u: f32, v: f32, p: Point) -> Color {
        if (p.x * self.size).sin() * (p.y * self.size).sin() * (p.z * self.size).sin() < 0.0 {
            self.color1
        } else {
            self.color2
        }
    }
}
