mod diffuse;
mod glass;
mod metal;

pub use diffuse::Diffuse;
pub use glass::Glass;
pub use metal::Metal;

use super::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> (Ray, Color);
}
