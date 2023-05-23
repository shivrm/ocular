use ocular::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const SAMPLES_PER_PIXEL: usize = 16;
const BOUNCES: usize = 16;
const CLIP_START: f32 = 0.01;
const CLIP_END: f32 = f32::INFINITY;

fn main() {
    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), 3.55, 2.0, 1.0);
    let sky = texture::Sky;

    let front = {
        let texture = texture::Solid::new(Color::new(0.7, 0.3, 0.3));
        let material = material::Diffuse::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, Box::new(material));
        Box::new(sphere)
    };

    let ground = {
        let texture = texture::Solid::new(Color::new(0.8, 0.8, 0.0));
        let material = material::Diffuse::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, Box::new(material));
        Box::new(sphere)
    };

    let left = {
        let texture = texture::Solid::new(Color::new(0.8, 0.8, 0.8));
        let material = material::Metal::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, Box::new(material));
        Box::new(sphere)
    };

    let objects: Vec<Box<dyn Hittable>> = vec![front, ground, left];

    let options = RenderOptions {
        width: WIDTH,
        height: HEIGHT,
        samples: SAMPLES_PER_PIXEL,
        bounces: BOUNCES,
        clip_start: CLIP_START,
        clip_end: CLIP_END,
    };

    let scene = Scene::new(camera, Box::new(sky), objects);
    let image = scene.render(options);

    let mut bitmap = bmp::Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ocular::Pixel { r, g, b } = image.get_pixel(x, y);
            bitmap.set_pixel(x as u32, y as u32, bmp::Pixel { r, g, b });
        }
    }

    bitmap.save("result.bmp").unwrap();
}
