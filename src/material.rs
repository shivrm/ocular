mod diffuse;
mod glass;
mod light;
mod metal;

pub use diffuse::Diffuse;
pub use glass::Glass;
pub use light::Light;
pub use metal::Metal;

use super::{Color, HitRecord, Point, Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<(Ray, Color)>;
    fn emit(&self, _u: f32, _v: f32, _point: Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
