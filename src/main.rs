use ocular::{Camera, Point, Scene};

const WIDTH: usize = 64;
const HEIGHT: usize = 36;

fn main() {
    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), 3.55, 2.0, 1.0);
    let scene = Scene::new(camera);
    let image = scene.render(WIDTH, HEIGHT);

    let mut bitmap = bmp::Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ocular::Pixel { r, g, b } = image.get_pixel(x, y);
            bitmap.set_pixel(x as u32, y as u32, bmp::Pixel { r, g, b });
        }
    }

    bitmap.save("result.bmp").unwrap();
}
