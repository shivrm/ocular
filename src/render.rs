use super::{Camera, Hittable, Scene, Texture};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub crop_region: ((usize, usize), (usize, usize)),
    pub samples: usize,
    pub bounces: usize,
    pub clip_start: f32,
    pub clip_end: f32,
}

pub struct SharedRenderer {
    camera: Arc<Camera>,
    texture: Arc<Box<dyn Texture>>,
    objects: Arc<Vec<Box<dyn Hittable>>>,
}

impl SharedRenderer {
    fn new(
        camera: Arc<Camera>,
        texture: Arc<Box<dyn Texture>>,
        objects: Arc<Vec<Box<dyn Hittable>>>,
    ) -> Self {
        Self {
            camera,
            texture,
            objects,
        }
    }
}
