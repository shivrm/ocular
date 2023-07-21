use ocular::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const LOOKFROM: Point = Point::new(-8.0, 3.0, 10.5);
const LOOKAT: Point = Point::new(0.0, 0.0, 0.0);
const CAMERA_UP: Point = Point::new(0.0, 1.0, 0.0);
const FOV: f32 = 20.0;
const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);
const APERTURE: f32 = 0.1;
const FOCUS_DIST: f32 = 13.5;

const SAMPLES_PER_PIXEL: usize = 64;
const BOUNCES: usize = 16;
const CLIP_START: f32 = 0.01;
const CLIP_END: f32 = f32::INFINITY;
const BLOCK_SIZE: usize = 32;

fn main() {
    let camera = Camera::new(
        LOOKFROM,
        LOOKAT,
        CAMERA_UP,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DIST,
    );
    let sky = texture::Sky;

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let ground = {
        let texture = texture::Solid::new(Color::new(0.5, 0.5, 0.5));
        let material = material::Diffuse::new(Box::new(texture));
        let sphere = object::Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material));
        Box::new(sphere)
    };

    objects.push(ground);

    let mesh = {
        let file = std::fs::File::open("./cube.obj").unwrap();
        let obj = obj::load_obj(std::io::BufReader::new(file)).unwrap();

        let texture = texture::Solid::new(Color::new(0.8, 0.2, 0.2));
        let material = material::Diffuse::new(Box::new(texture));
        let center = Point::new(0.0, 1.0, 0.0);
        let mesh = Mesh::from_obj(obj, center, Box::new(material));
        Box::new(mesh)
    };

    objects.push(mesh);

    let options = RenderOptions {
        width: WIDTH,
        height: HEIGHT,
        crop_region: ((0, WIDTH), (0, HEIGHT)),
        samples: SAMPLES_PER_PIXEL,
        bounces: BOUNCES,
        clip_start: CLIP_START,
        clip_end: CLIP_END,
        block_size: BLOCK_SIZE,
    };

    let scene = Scene::new(camera, Box::new(sky), objects);
    let image = scene.threaded_render(options);

    let mut bitmap = bmp::Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ocular::Pixel { r, g, b } = image.get_pixel(x, y);
            bitmap.set_pixel(x as u32, y as u32, bmp::Pixel { r, g, b });
        }
    }

    bitmap.save("result.bmp").unwrap();
}
