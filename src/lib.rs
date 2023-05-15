mod vec3;
pub use vec3::Vec3;

mod ray;
pub use ray::Ray;

mod camera;
pub use camera::Camera;

mod scene;
pub use scene::Scene;

mod image;
pub use image::{Image, Pixel};

// Aliases
pub type Point = Vec3;
pub type Color = Vec3;
