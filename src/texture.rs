use super::{Color, Point};

pub trait Texture {
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
    let u = (f32::atan2(unit.x, unit.z) / std::f32::consts::TAU) + 0.5;
    let v = (unit.y * 0.5) + 0.5;
    (u, v)
}
