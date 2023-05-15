use ocular::{Camera, Point, Scene};

fn main() {
    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), 3.55, 2.0, 1.0);
    let scene = Scene::new(camera);
    let image = scene.render(64, 36);
}
