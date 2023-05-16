mod vec3;
pub use vec3::Vec3;

mod ray;
pub use ray::Ray;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::{RenderOptions, Scene};

mod image;
pub use image::{Image, Pixel};

pub mod texture;
pub use texture::Texture;

pub mod object;
pub use object::{HitRecord, Hittable};

pub mod material;
pub use material::Material;

// Aliases
pub type Point = Vec3;
pub type Color = Vec3;

fn random() -> f32 {
    use rand::Rng;
    rand::thread_rng().gen_range(0.0..0.1)
}
