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

pub mod mesh;
pub use mesh::{Mesh, Trig};

// Aliases
pub type Point = Vec3;
pub type Color = Vec3;

pub fn random(min: f32, max: f32) -> f32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..max)
}
